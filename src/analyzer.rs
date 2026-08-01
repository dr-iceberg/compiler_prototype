use std::{error::Error, fmt::Display};

use crate::grammar::{Decl, Expr, Function, Literal, Program, Stmt, Type};

pub struct SemanticAnalyzer {
    #[allow(unused)]
    id_table: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            id_table: Vec::new(),
        }
    }

    pub fn check_program(&self, program: &Program) -> Result<(), SemanticError> {
        for decl in &program.declarations {
            match decl {
                Decl::Function(function) => {
                    self.check_function(function)?;
                }
            }
        }
        Ok(())
    }

    fn check_function(&self, function: &Function) -> Result<(), SemanticError> {
        self.check_function_return_type(function)?;
        Ok(())
    }

    fn check_function_return_type(&self, function: &Function) -> Result<(), SemanticError> {
        for stmt in &function.body {
            match stmt {
                Stmt::Return(expr) => {
                    let actual_type = self.infer_expr_type(expr)?;
                    if actual_type != function.return_type {
                        return Err(SemanticError::FunctionReturnTypeMismatch(
                            function.id.clone(),
                        ));
                    }
                }
                _ => {
                    todo!()
                }
            }
        }
        Ok(())
    }

    fn infer_expr_type(&self, expr: &Expr) -> Result<Type, SemanticError> {
        match expr {
            Expr::Literal(literal) => match literal {
                Literal::Interger(_) => Ok(Type::Int),
                Literal::Float(_) => Ok(Type::Float),
                Literal::Bool(_) => Ok(Type::Bool),
                Literal::String(_) => Ok(Type::String),
            },
            _ => {
                todo!()
            }
        }
    }

    #[allow(unused)]
    fn generate_id_table(&self) {}

    #[allow(unused)]
    fn check_stmt(stmt: &Stmt) {}

    #[allow(unused)]
    fn check_expr(expr: &Expr) {}
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum SemanticError {
    FunctionReturnTypeMismatch(String),
}

impl Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FunctionReturnTypeMismatch(id) => {
                write!(f, "Return type mismatch in function {id}")
            }
        }
    }
}

impl Error for SemanticError {}
