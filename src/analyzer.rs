use crate::grammar::{Expr, ExprId};
use std::{collections::HashMap, error::Error, fmt::Display};

use crate::grammar::{BinOp, Decl, ExprKind, Function, Literal, Program, Stmt, Type, UnaryOp};

#[derive(Clone, Default)]
pub struct SemanticInfo {
    expr_types: Vec<Type>,
    symbol_type: HashMap<String, Type>,
}
impl SemanticInfo {
    pub fn new() -> Self {
        Self {
            expr_types: Vec::new(),
            symbol_type: HashMap::new(),
        }
    }

    pub fn set_symbol_type(&mut self, id: String, ty: Type) {
        self.symbol_type.insert(id, ty);
    }

    pub fn get_symbol_type(&self, id: &String) -> Option<&Type> {
        self.symbol_type.get(id)
    }

    pub fn set_expr_type(&mut self, id: ExprId, ty: &Type) {
        let index = id.index();

        if index >= self.expr_types.len() {
            self.expr_types.resize(index + 1, Type::Unknown);
        }

        self.expr_types[index] = ty.clone();
    }

    pub fn expr_type(&self, id: &ExprId) -> &Type {
        &self.expr_types[id.index()]
    }
}

pub struct SemanticAnalyzer {
    semantic_info: SemanticInfo,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            semantic_info: SemanticInfo::new(),
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<SemanticInfo, Vec<SemanticError>> {
        let mut errors: Vec<SemanticError> = Vec::new();
        for decl in &program.declarations {
            match decl {
                Decl::Function(function) => {
                    if let Err(errs) = self.check_function(function) {
                        errors.extend(errs);
                    }
                }
            }
        }
        if errors.is_empty() {
            Ok(self.semantic_info.clone())
        } else {
            Err(errors)
        }
    }

    fn check_function(&mut self, function: &Function) -> Result<(), Vec<SemanticError>> {
        let mut errors: Vec<SemanticError> = Vec::new();
        if let Err(errs) = self.check_function_statements(function) {
            errors.extend(errs);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn check_function_statements(&mut self, function: &Function) -> Result<(), Vec<SemanticError>> {
        let mut errors: Vec<SemanticError> = Vec::new();
        if let Err(err) = self.check_function_return_type(function) {
            errors.push(err);
        }
        for stmt in &function.body {
            match stmt {
                Stmt::Return(expr) => {
                    if let Err(err) = self.check_function_return_stmt(function, expr) {
                        errors.push(err);
                    }
                }
                Stmt::LetAssign { id, expr } => {
                    if let Err(err) = self.check_let_assign_stmt(id, expr) {
                        errors.push(err);
                    }
                }
            }
        }
        Ok(())
    }

    fn check_let_assign_stmt(&mut self, id: &str, expr: &Expr) -> Result<(), SemanticError> {
        let ty = self.infer_expr_type(expr)?;
        self.semantic_info.set_symbol_type(id.to_owned(), ty);
        Ok(())
    }

    fn check_function_return_type(&mut self, function: &Function) -> Result<(), SemanticError> {
        if function.id == "main" && function.return_type != Type::Int {
            return Err(SemanticError::MainFunctionInvalidReturnType(
                function.return_type.clone(),
            ));
        }
        Ok(())
    }

    fn check_function_return_stmt(
        &mut self,
        function: &Function,
        expr: &Expr,
    ) -> Result<(), SemanticError> {
        match self.infer_expr_type(expr) {
            Ok(actual_type) => {
                if actual_type != function.return_type {
                    return Err(SemanticError::FunctionReturnTypeMismatch(
                        function.id.clone(),
                    ));
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
        Ok(())
    }

    fn set_expr_type(&mut self, id: ExprId, ty: &Type) {
        self.semantic_info.set_expr_type(id, ty);
    }

    fn infer_expr_type(&mut self, expr: &Expr) -> Result<Type, SemanticError> {
        let kind = expr.kind();
        let id = expr.id();
        let ty = match kind {
            ExprKind::Literal(literal) => match literal {
                Literal::Interger(_) => Type::Int,
                Literal::Float(_) => Type::Float,
                Literal::Bool(_) => Type::Bool,
                Literal::String(_) => Type::String,
            },
            ExprKind::Binary { left, op, right } => {
                let lhs = self.infer_expr_type(left)?;
                let rhs = self.infer_expr_type(right)?;
                match *op {
                    BinOp::Add => match (lhs.clone(), rhs.clone()) {
                        (Type::Int, Type::Int) => Type::Int,
                        (Type::Float, Type::Float) => Type::Float,
                        (Type::String, Type::String) => Type::String,
                        _ => return Err(SemanticError::TypeMismatchInExpr(lhs, rhs, kind.clone())),
                    },
                    BinOp::Subtract | BinOp::Multiply | BinOp::Divide => {
                        match (lhs.clone(), rhs.clone()) {
                            (Type::Int, Type::Int) => Type::Int,
                            (Type::Float, Type::Float) => Type::Float,
                            _ => {
                                return Err(SemanticError::TypeMismatchInExpr(
                                    lhs,
                                    rhs,
                                    kind.clone(),
                                ));
                            }
                        }
                    }
                    BinOp::Greater | BinOp::GreaterEqual | BinOp::Less | BinOp::LessEqual => {
                        match (lhs.clone(), rhs.clone()) {
                            (Type::Int, Type::Int) => Type::Bool,
                            (Type::Float, Type::Float) => Type::Bool,
                            _ => {
                                return Err(SemanticError::TypeMismatchInExpr(
                                    lhs,
                                    rhs,
                                    kind.clone(),
                                ));
                            }
                        }
                    }
                    BinOp::Equal | BinOp::NotEqual => match (lhs.clone(), rhs.clone()) {
                        (Type::Int, Type::Int) => Type::Bool,
                        (Type::Float, Type::Float) => Type::Bool,
                        (Type::Bool, Type::Bool) => Type::Bool,
                        _ => return Err(SemanticError::TypeMismatchInExpr(lhs, rhs, kind.clone())),
                    },
                }
            }
            ExprKind::Unary { op, expr } => {
                let ty = self.infer_expr_type(expr)?;
                match *op {
                    UnaryOp::Negate => match ty {
                        Type::Int | Type::Float => ty,
                        _ => return Err(SemanticError::InvalidUnaryOperand(op.clone(), ty)),
                    },
                    UnaryOp::Not => match ty {
                        Type::Bool => Type::Bool,
                        _ => return Err(SemanticError::InvalidUnaryOperand(op.clone(), ty)),
                    },
                }
            }
            ExprKind::Identifier(id) => {
                if let Some(ty) = self.semantic_info.get_symbol_type(id) {
                    ty.clone()
                } else {
                    return Err(SemanticError::UnknownVariableInExpr(
                        id.clone(),
                        kind.clone(),
                    ));
                }
            }
        };
        self.set_expr_type(id, &ty);
        Ok(ty)
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
    fn check_expr(expr: &ExprKind) {
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
    TypeMismatchInExpr(Type, Type, ExprKind),
    InvalidUnaryOperand(UnaryOp, Type),
    UnknownVariableInExpr(String, ExprKind),
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
                    "Invalid return type in function 'main'. Expected '{:#?}' found '{:#?}'.",
                    Type::Int,
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
            Self::UnknownVariableInExpr(id, expr) => {
                write!(
                    f,
                    "Unknown variable '{:#?}' in expression '{:#?}'",
                    id, expr
                )
            }
        }
    }
}

impl Error for SemanticError {}
