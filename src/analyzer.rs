use crate::{
    ast::NodeId,
    common::SemanticInfo,
    grammar::{DeclKind, Expr, Parameter, StmtKind},
};
use std::{collections::HashMap, error::Error, fmt::Display};

use crate::grammar::{BinOp, Decl, ExprKind, Function, Literal, Program, Stmt, Type, UnaryOp};

/// Scope holds symbol names (`String`) and corresponding `NodeId`'s
#[derive(Debug, Clone, Default)]
pub struct Scope {
    symbols: HashMap<String, NodeId>,
}

impl Scope {
    pub fn add_symbol(&mut self, name: String, id: NodeId) {
        self.symbols.insert(name, id);
    }

    pub fn get_symbol(&self, name: &String) -> Option<&NodeId> {
        self.symbols.get(name)
    }

    pub fn add_function_params_to_scope(&mut self, params: &[Parameter]) {
        for param in params {
            self.add_symbol(param.name.clone(), param.id);
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FunctionSignature {
    pub params: Vec<Type>,
    pub return_type: Type,
}

pub struct SemanticAnalyzer {
    semantic_info: SemanticInfo,
    scopes: Vec<Scope>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            semantic_info: SemanticInfo::default(),
            scopes: Vec::new(),
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<SemanticInfo, Vec<SemanticError>> {
        self.enter_scope();
        let mut errors: Vec<SemanticError> = Vec::new();
        self.collect_function_signatures(&program.declarations);
        for decl in &program.declarations {
            match &decl.kind {
                DeclKind::Function(function) => {
                    if let Err(errs) = self.check_function(function) {
                        errors.extend(errs);
                    }
                }
            }
        }
        self.leave_scope();
        if errors.is_empty() {
            Ok(self.semantic_info.clone())
        } else {
            Err(errors)
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    fn get_current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().expect("Could not retrieve a scope")
    }

    fn resolve_symbol(&self, name: &String) -> Option<&NodeId> {
        for scope in self.scopes.iter().rev() {
            if let Some(id) = scope.get_symbol(name) {
                return Some(id);
            }
        }
        None
    }

    fn add_variable(&mut self, id: NodeId, name: String, ty: Type) {
        let scope = self.get_current_scope();
        scope.add_symbol(name, id);
        self.semantic_info.add_type(id, ty);
    }

    fn add_params(&mut self, params: &[Parameter]) {
        self.get_current_scope()
            .add_function_params_to_scope(params);
        self.semantic_info.add_function_params(params);
    }

    fn collect_function_signatures(&mut self, declatarations: &Vec<Decl>) {
        for decl in declatarations {
            match &decl.kind {
                DeclKind::Function(function) => {
                    let mut params: Vec<Type> = Vec::new();
                    for param in &function.parameters {
                        params.push(param.ty.clone());
                    }
                    let function_signature = FunctionSignature {
                        params,
                        return_type: function.return_type.clone(),
                    };
                    self.semantic_info
                        .add_function_signature(function.id, function_signature);
                }
            }
        }
    }

    fn check_function(&mut self, function: &Function) -> Result<(), Vec<SemanticError>> {
        let mut errors: Vec<SemanticError> = Vec::new();
        self.enter_scope();
        self.add_params(&function.parameters);
        if let Err(errs) = self.check_function_statements(function) {
            errors.extend(errs);
        }
        self.leave_scope();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn check_function_statements(&mut self, function: &Function) -> Result<(), Vec<SemanticError>> {
        let mut errors: Vec<SemanticError> = Vec::new();
        // if let Err(err) = self.check_function_return_type(function) {
        //     errors.push(err);
        // }
        for stmt in &function.body {
            match &stmt.kind {
                StmtKind::Return(expr) => {
                    if let Err(err) = self.check_function_return_stmt(function, expr) {
                        errors.push(err);
                    }
                }
                StmtKind::LetAssign { id, name, expr } => {
                    if let Err(err) = self.check_let_assign_stmt(id, name.clone(), expr) {
                        errors.push(err);
                    }
                }
                StmtKind::Expr(_) => {
                    unimplemented!()
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn check_let_assign_stmt(
        &mut self,
        id: &NodeId,
        name: String,
        expr: &Expr,
    ) -> Result<(), SemanticError> {
        let ty = self.infer_expr_type(expr)?;
        self.add_variable(*id, name, ty);
        Ok(())
    }

    #[allow(unused)]
    fn check_function_return_type(&mut self, function: &Function) -> Result<(), SemanticError> {
        todo!()
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
                        function.name.clone(),
                    ));
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
        Ok(())
    }

    fn infer_expr_type(&mut self, expr: &Expr) -> Result<Type, SemanticError> {
        let ty = match &expr.kind {
            ExprKind::Literal(literal) => match literal {
                Literal::Interger(_) => Type::Int,
                Literal::Float(_) => Type::Float,
                Literal::Bool(_) => Type::Bool,
                Literal::String(_) => Type::String,
            },
            ExprKind::Binary { left, op, right } => {
                let lhs = self.infer_expr_type(left)?;
                let rhs = self.infer_expr_type(right)?;
                match op {
                    BinOp::Add => match (lhs.clone(), rhs.clone()) {
                        (Type::Int, Type::Int) => Type::Int,
                        (Type::Float, Type::Float) => Type::Float,
                        (Type::String, Type::String) => Type::String,
                        _ => {
                            return Err(SemanticError::TypeMismatchInExpr(
                                lhs,
                                rhs,
                                expr.kind.clone(),
                            ));
                        }
                    },
                    BinOp::Subtract | BinOp::Multiply | BinOp::Divide => {
                        match (lhs.clone(), rhs.clone()) {
                            (Type::Int, Type::Int) => Type::Int,
                            (Type::Float, Type::Float) => Type::Float,
                            _ => {
                                return Err(SemanticError::TypeMismatchInExpr(
                                    lhs,
                                    rhs,
                                    expr.kind.clone(),
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
                                    expr.kind.clone(),
                                ));
                            }
                        }
                    }
                    BinOp::Equal | BinOp::NotEqual => match (lhs.clone(), rhs.clone()) {
                        (Type::Int, Type::Int) => Type::Bool,
                        (Type::Float, Type::Float) => Type::Bool,
                        (Type::Bool, Type::Bool) => Type::Bool,
                        _ => {
                            return Err(SemanticError::TypeMismatchInExpr(
                                lhs,
                                rhs,
                                expr.kind.clone(),
                            ));
                        }
                    },
                }
            }
            ExprKind::Unary { op, expr } => {
                let ty = self.infer_expr_type(expr)?;
                match op {
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
            ExprKind::Identifier { id: _, name } => {
                let resolved_id = self.resolve_symbol(name);
                if let Some(resolved_id) = resolved_id {
                    self.semantic_info
                        .get_type(resolved_id)
                        .expect("Could not resolve symbol")
                        .clone()
                } else {
                    return Err(SemanticError::UnknownVariableInExpr(
                        name.clone(),
                        expr.kind.clone(),
                    ));
                }
            }
            ExprKind::Call { id: _, name, args } => {
                //todo!("Check if provided argument types match the function argument types")
                let mut arg_types: Vec<Type> = Vec::new();
                for arg in args {
                    arg_types.push(self.infer_expr_type(arg)?);
                }
                let resolved_id = self.resolve_symbol(name).expect("Could not resolve symbol");
                let function_sig = self
                    .semantic_info
                    .get_function_signature(resolved_id)
                    .expect("Could not retrieve function signature");
                let expected_arg_count = function_sig.params.len();
                let received_arg_count = arg_types.len();
                if received_arg_count == expected_arg_count {
                    for (i, ty) in arg_types.iter().enumerate() {
                        if *ty != function_sig.params[i] {
                            return Err(SemanticError::IncorrectFunctionArgumentType(
                                expr.kind.clone(),
                                function_sig.params[i].clone(),
                                arg_types[i].clone(),
                            ));
                        }
                    }
                    function_sig.return_type.clone()
                } else {
                    return Err(SemanticError::InvalidNumberOfFunctionArguments(
                        expr.kind.clone(),
                        expected_arg_count,
                        received_arg_count,
                    ));
                }
            }
        };
        self.semantic_info.add_type(expr.id, ty.clone());
        Ok(ty)
    }

    #[allow(unused)]
    fn check_stmt(stmt: &Stmt) {
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
    InvalidNumberOfFunctionArguments(ExprKind, usize, usize),
    IncorrectFunctionArgumentType(ExprKind, Type, Type),
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
            Self::InvalidNumberOfFunctionArguments(expr, expected, got) => {
                write!(
                    f,
                    "Invalid number of arguments in function: {:#?}\nexpected: {}\ngot: {}",
                    expr, expected, got
                )
            }
            Self::IncorrectFunctionArgumentType(expr, expected, got) => {
                write!(
                    f,
                    "Incorrect argument type in function {:#?}\nexpected {:#?}\ngot: {:#?}",
                    expr, expected, got
                )
            }
        }
    }
}

impl Error for SemanticError {}
