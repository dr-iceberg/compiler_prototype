use std::{error::Error, fmt::Display};

use crate::grammar::{BinOp, Decl, Expr, Function, Literal, Program, Stmt, Type, UnaryOp};

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
        if function.id == "main" && function.return_type != Type::Int {
            return Err(SemanticError::MainFunctionInvalidReturnType(
                function.return_type.clone(),
            ));
        }
        for stmt in &function.body {
            if let Stmt::Return(expr) = stmt {
                let actual_type = self.infer_expr_type(expr)?;
                if actual_type != function.return_type {
                    return Err(SemanticError::FunctionReturnTypeMismatch(
                        function.id.clone(),
                    ));
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
            Expr::Binary { left, op, right } => {
                let lhs = self.infer_expr_type(left)?;
                let rhs = self.infer_expr_type(right)?;
                match *op {
                    BinOp::Add => match (lhs.clone(), rhs.clone()) {
                        (Type::Int, Type::Int) => Ok(Type::Int),
                        (Type::Float, Type::Float) => Ok(Type::Float),
                        (Type::String, Type::String) => Ok(Type::String),
                        _ => Err(SemanticError::TypeMismatchInExpr(lhs, rhs, expr.clone())),
                    },
                    BinOp::Subtract | BinOp::Multiply | BinOp::Divide => {
                        match (lhs.clone(), rhs.clone()) {
                            (Type::Int, Type::Int) => Ok(Type::Int),
                            (Type::Float, Type::Float) => Ok(Type::Float),
                            _ => Err(SemanticError::TypeMismatchInExpr(lhs, rhs, expr.clone())),
                        }
                    }
                    BinOp::Greater | BinOp::GreaterEqual | BinOp::Less | BinOp::LessEqual => {
                        match (lhs.clone(), rhs.clone()) {
                            (Type::Int, Type::Int) => Ok(Type::Bool),
                            (Type::Float, Type::Float) => Ok(Type::Bool),
                            _ => Err(SemanticError::TypeMismatchInExpr(lhs, rhs, expr.clone())),
                        }
                    }
                    BinOp::Equal | BinOp::NotEqual => match (lhs.clone(), rhs.clone()) {
                        (Type::Int, Type::Int) => Ok(Type::Bool),
                        (Type::Float, Type::Float) => Ok(Type::Bool),
                        (Type::Bool, Type::Bool) => Ok(Type::Bool),
                        _ => Err(SemanticError::TypeMismatchInExpr(lhs, rhs, expr.clone())),
                    },
                }
            }
            Expr::Unary { op, expr } => {
                let ty = self.infer_expr_type(expr)?;
                match *op {
                    UnaryOp::Negate => match ty {
                        Type::Int | Type::Float => Ok(ty),
                        _ => Err(SemanticError::InvalidUnaryOperand(op.clone(), ty)),
                    },
                    UnaryOp::Not => match ty {
                        Type::Bool => Ok(Type::Bool),
                        _ => Err(SemanticError::InvalidUnaryOperand(op.clone(), ty)),
                    },
                }
            }
            Expr::Identifier(_id) => {
                todo!("Add identifier type inference")
            }
        }
    }

    #[allow(unused)]
    fn generate_id_table(&self) {
        todo!()
    }

    #[allow(unused)]
    fn check_stmt(stmt: &Stmt) {
        todo!()
    }

    #[allow(unused)]
    fn check_expr(expr: &Expr) {
        todo!()
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum SemanticError {
    FunctionReturnTypeMismatch(String),
    MainFunctionInvalidReturnType(Type),
    InvalidBinaryOperands(BinOp, Type, Type),
    TypeMismatchInExpr(Type, Type, Expr),
    InvalidUnaryOperand(UnaryOp, Type),
}

impl Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FunctionReturnTypeMismatch(id) => {
                write!(f, "Return type mismatch in function {id}")
            }
            Self::MainFunctionInvalidReturnType(ty) => {
                write!(
                    f,
                    "Invalid return type in function 'main'. Expected 'int' found {:#?}.",
                    ty
                )
            }
            Self::InvalidBinaryOperands(op, lhs, rhs) => {
                write!(
                    f,
                    "Invalid binary operands '{:#?}' and '{:#?}' in binary operation '{:#?}'",
                    lhs, rhs, op
                )
            }
            Self::TypeMismatchInExpr(ty1, ty2, expr) => {
                write!(
                    f,
                    "Type mismatch in expression: '{:#?}'\ntype1: '{:#?}' type2: '{:#?}' ",
                    expr, ty1, ty2
                )
            }
            Self::InvalidUnaryOperand(op, ty) => {
                write!(
                    f,
                    "Invalid unary operand '{:#?}' in unary operation '{:#?}'",
                    ty, op
                )
            }
        }
    }
}

impl Error for SemanticError {}
