use std::{collections::HashMap, error::Error};

use inkwell::{
    builder::{Builder, BuilderError},
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicTypeEnum},
    values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue},
};

use crate::{
    analyzer::SemanticInfo,
    grammar::{
        BinOp, Decl, Expr, ExprId, ExprKind, Function, FunctionId, Literal, Program, Stmt,
        SymbolId, Type, UnaryOp, VariableId,
    },
};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
    semantic_info: &'ctx SemanticInfo,
    variables: HashMap<VariableId, PointerValue<'ctx>>,
    functions: HashMap<FunctionId, FunctionValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, semantic_info: &'ctx SemanticInfo) -> Self {
        Self {
            context,
            builder: context.create_builder(),
            module: context.create_module("main"),
            semantic_info,
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn type_into_metadata_type_enum(
        context: &'ctx Context,
        ty: &Type,
    ) -> BasicMetadataTypeEnum<'ctx> {
        match ty {
            Type::Int => BasicMetadataTypeEnum::IntType(context.i32_type()),
            _ => {
                todo!()
            }
        }
    }

    fn type_into_type_enum(context: &'ctx Context, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::Int => context.i32_type().into(),
            _ => {
                todo!()
            }
        }
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<(), Box<dyn Error>> {
        for decl in &program.declarations {
            match decl {
                Decl::Function(function) => {
                    self.compile_function(function)?;
                }
            }
        }

        self.module.print_to_file("main.ll")?;
        Ok(())
    }

    fn compile_function(&mut self, function: &Function) -> Result<(), BuilderError> {
        let return_type = match function.return_type {
            Type::Int => self.context.i32_type(),
            _ => {
                todo!()
            }
        };

        let function_signature = self.semantic_info.get_function_signature(&function.id);

        let param_types: Vec<BasicMetadataTypeEnum> = function_signature
            .params()
            .iter()
            .map(|ty| CodeGen::<'ctx>::type_into_metadata_type_enum(self.context, ty))
            .collect();

        let function_type = return_type.fn_type(&param_types, false);

        let llvm_function = self
            .module
            .add_function(function.id.get(), function_type, None);
        self.functions.insert(function.id.clone(), llvm_function);

        let entry = self.context.append_basic_block(llvm_function, "entry");

        self.builder.position_at_end(entry);

        for (i, param) in function.parameters.iter().enumerate() {
            let param_value = llvm_function
                .get_nth_param(i as u32)
                .expect("Could not retrieve the ith function param");
            param_value.set_name(param.id.get());

            let mut id: String = param.id.get().to_string();
            id.push_str(".addr"); // append ".addr" to distinguish between the SSA value
            // and the allocated register

            let ty = CodeGen::<'ctx>::type_into_type_enum(self.context, &param.ty);
            let ptr = self.builder.build_alloca(ty, &id)?;
            self.builder.build_store(ptr, param_value)?;

            self.variables.insert(VariableId::new(param.id.get()), ptr);
        }

        for stmt in &function.body {
            self.compile_statement(stmt)?;
        }
        Ok(())
    }

    fn compile_function_call(
        &self,
        id: &FunctionId,
        args: &[Expr],
    ) -> Result<BasicValueEnum<'ctx>, BuilderError> {
        let function = *self
            .functions
            .get(id)
            .expect("Could not retrieve a FunctionValue");
        let args: Vec<BasicMetadataValueEnum> = args
            .iter()
            .map(|arg| self.compile_expression(arg).into())
            .collect();
        let val = self.builder.build_call(function, &args, id.get())?;

        Ok(val
            .try_as_basic_value()
            .basic()
            .expect("Could not convert CallSiteValue into a BasicValue"))
    }

    fn compile_statement(&mut self, stmt: &Stmt) -> Result<(), BuilderError> {
        match stmt {
            Stmt::Return(expr) => {
                let value = self.compile_expression(expr);
                self.builder
                    .build_return(Some(&value))
                    .expect("Could not build a return statement");
            }
            Stmt::LetAssign { id, expr } => {
                let value = self.compile_expression(expr);
                let ptr = self.builder.build_alloca(value.get_type(), id.get())?;
                self.builder.build_store(ptr, value)?;

                self.variables.insert(id.clone(), ptr);
            }
            Stmt::Expr(_expr) => todo!("Implement standalone function calls"),
        }
        Ok(())
    }

    fn get_expr_type(&self, id: &ExprId) -> &Type {
        self.semantic_info.expr_type(id)
    }

    fn compile_expression(&self, expr: &Expr) -> BasicValueEnum<'ctx> {
        let id = expr.id();
        let kind = expr.kind();
        let ty = self.get_expr_type(&id);
        match kind {
            ExprKind::Literal(literal) => match literal {
                Literal::Interger(value) => self
                    .context
                    .i32_type()
                    .const_int(*value as u64, true)
                    .into(),
                Literal::Float(value) => self.context.f32_type().const_float(*value).into(),
                _ => {
                    todo!()
                }
            },
            ExprKind::Binary { left, op, right } => {
                let lhs = self.compile_expression(left);
                let rhs = self.compile_expression(right);
                match ty {
                    Type::Int => {
                        // into_type_value() functions can panic
                        // if the expected type doesnt match

                        let lhs = lhs.into_int_value();
                        let rhs = rhs.into_int_value();
                        match *op {
                            BinOp::Add => self
                                .builder
                                .build_int_add(lhs, rhs, "add")
                                .expect("Could not build int add")
                                .into(),
                            BinOp::Subtract => self
                                .builder
                                .build_int_sub(lhs, rhs, "sub")
                                .expect("Could not build int sub")
                                .into(),
                            BinOp::Multiply => self
                                .builder
                                .build_int_mul(lhs, rhs, "mul")
                                .expect("Could not build int mul")
                                .into(),
                            BinOp::Divide => self
                                .builder
                                .build_int_signed_div(lhs, rhs, "div")
                                .expect("Could not build int div")
                                .into(),
                            BinOp::Equal => todo!(),
                            BinOp::NotEqual => todo!(),
                            BinOp::Less => todo!(),
                            BinOp::LessEqual => todo!(),
                            BinOp::Greater => todo!(),
                            BinOp::GreaterEqual => todo!(),
                        }
                    }
                    Type::Float => {
                        let lhs = lhs.into_float_value();
                        let rhs = rhs.into_float_value();
                        match *op {
                            BinOp::Add => self
                                .builder
                                .build_float_add(lhs, rhs, "add")
                                .expect("Could not build float add")
                                .into(),
                            BinOp::Subtract => self
                                .builder
                                .build_float_sub(lhs, rhs, "sub")
                                .expect("Could not build float sub")
                                .into(),
                            BinOp::Multiply => self
                                .builder
                                .build_float_mul(lhs, rhs, "mul")
                                .expect("Could not build float mul")
                                .into(),
                            BinOp::Divide => self
                                .builder
                                .build_float_div(lhs, rhs, "div")
                                .expect("Could not build float div")
                                .into(),
                            _ => {
                                todo!()
                            }
                        }
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            ExprKind::Unary { op, expr } => {
                let value = self.compile_expression(expr);
                match ty {
                    Type::Int => {
                        let value = value.into_int_value();
                        match *op {
                            UnaryOp::Negate => self
                                .builder
                                .build_int_neg(value, "neg")
                                .expect("Could not build an int negative value")
                                .into(),
                            UnaryOp::Not => unimplemented!(),
                        }
                    }
                    Type::Float => {
                        let value = value.into_float_value();
                        match *op {
                            UnaryOp::Negate => self
                                .builder
                                .build_float_neg(value, "neg")
                                .expect("Could not build float negative value")
                                .into(),
                            UnaryOp::Not => unimplemented!(),
                        }
                    }
                    Type::Bool => todo!(),
                    Type::String | Type::Void | Type::Unknown => unimplemented!(),
                }
            }
            ExprKind::Identifier(id) => {
                let ptr = self
                    .variables
                    .get(id)
                    .expect("Could not find the variable with specified id");
                let ty = self
                    .semantic_info
                    .get_variable_type(id)
                    .expect("Could not get the type of the variable with specified id");

                self.builder
                    .build_load(
                        CodeGen::<'ctx>::type_into_type_enum(self.context, ty),
                        *ptr,
                        id.get(),
                    )
                    .expect("Could not build load instruction")
            }
            ExprKind::Call { id, args } => self
                .compile_function_call(id, args)
                .expect("Could not compile function call"),
        }
    }
}
