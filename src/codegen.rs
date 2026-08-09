use std::error::Error;

use inkwell::{builder::Builder, context::Context, module::Module, values::BasicValueEnum};

use crate::{
    analyzer::SemanticInfo,
    grammar::{
        BinOp, Decl, Expr, ExprId, ExprKind, Function, Literal, Program, Stmt, Type, UnaryOp,
    },
};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
    semantic_info: &'ctx SemanticInfo,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(ctx: &'ctx Context, semantic_info: &'ctx SemanticInfo) -> Self {
        Self {
            context: ctx,
            builder: ctx.create_builder(),
            module: ctx.create_module("main"),
            semantic_info,
        }
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<(), Box<dyn Error>> {
        for decl in &program.declarations {
            match decl {
                Decl::Function(function) => {
                    self.compile_function(function);
                }
            }
        }

        self.module.print_to_file("main.ll")?;
        Ok(())
    }

    fn compile_function(&mut self, function: &Function) {
        let return_type = match function.return_type {
            Type::Int => self.context.i32_type(),
            _ => {
                todo!()
            }
        };

        let function_type = return_type.fn_type(&[], false);

        let llvm_function = self.module.add_function(&function.id, function_type, None);

        let entry = self.context.append_basic_block(llvm_function, "entry");

        self.builder.position_at_end(entry);

        for stmt in &function.body {
            self.compile_statement(stmt);
        }
    }

    fn compile_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Return(expr) => {
                let value = self.compile_expression(expr);
                self.builder.build_return(Some(&value)).unwrap();
            }
            _ => {
                todo!()
            }
        }
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
                            BinOp::Add => {
                                self.builder.build_int_add(lhs, rhs, "add").unwrap().into()
                            }
                            BinOp::Subtract => {
                                self.builder.build_int_sub(lhs, rhs, "sub").unwrap().into()
                            }
                            BinOp::Multiply => {
                                self.builder.build_int_mul(lhs, rhs, "mul").unwrap().into()
                            }
                            BinOp::Divide => self
                                .builder
                                .build_int_signed_div(lhs, rhs, "div")
                                .unwrap()
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
                                .unwrap()
                                .into(),
                            BinOp::Subtract => self
                                .builder
                                .build_float_sub(lhs, rhs, "sub")
                                .unwrap()
                                .into(),
                            BinOp::Multiply => self
                                .builder
                                .build_float_mul(lhs, rhs, "mul")
                                .unwrap()
                                .into(),
                            BinOp::Divide => self
                                .builder
                                .build_float_div(lhs, rhs, "div")
                                .unwrap()
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
                            UnaryOp::Negate => {
                                self.builder.build_int_neg(value, "neg").unwrap().into()
                            }
                            UnaryOp::Not => unimplemented!(),
                        }
                    }
                    Type::Float => {
                        let value = value.into_float_value();
                        match *op {
                            UnaryOp::Negate => {
                                self.builder.build_float_neg(value, "neg").unwrap().into()
                            }
                            UnaryOp::Not => unimplemented!(),
                        }
                    }
                    Type::Bool => todo!(),
                    Type::String | Type::Void | Type::Unknown => unimplemented!(),
                }
            }
            _ => {
                todo!()
            }
        }
    }
}
