use std::error::Error;

use inkwell::{builder::Builder, context::Context, module::Module, values::BasicValueEnum};

use crate::grammar::{Decl, Expr, Function, Literal, Program, Stmt, Type};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Self {
        Self {
            context: ctx,
            builder: ctx.create_builder(),
            module: ctx.create_module("main"),
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

    fn compile_expression(&self, expr: &Expr) -> BasicValueEnum<'ctx> {
        match expr {
            Expr::Literal(literal) => match literal {
                Literal::Interger(value) => self
                    .context
                    .i32_type()
                    .const_int(*value as u64, true)
                    .into(),
                _ => {
                    todo!()
                }
            },
            _ => {
                todo!()
            }
        }
    }
}
