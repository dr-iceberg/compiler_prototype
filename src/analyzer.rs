use crate::grammar::{Expr, ExprId, FunctionId, Parameter, SymbolId, VariableId};
use std::{collections::HashMap, error::Error, fmt::Display};

use crate::grammar::{BinOp, Decl, ExprKind, Function, Literal, Program, Stmt, Type, UnaryOp};

#[derive(Debug, Clone)]
pub struct VariableInfo {
    ty: Type,
}

impl VariableInfo {
    pub fn new(ty: &Type) -> Self {
        Self { ty: ty.clone() }
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

#[derive(Debug, Clone, Default)]
pub struct Scope {
    variables: HashMap<VariableId, VariableInfo>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: Default::default(),
        }
    }

    pub fn add_variable(&mut self, id: &VariableId, info: &VariableInfo) {
        self.variables.insert(id.clone(), info.clone());
    }

    pub fn get_variable(&self, id: &VariableId) -> Option<&VariableInfo> {
        self.variables.get(id)
    }

    pub fn add_function_params_to_scope(&mut self, params: &Vec<Parameter>) {
        for param in params {
            let id = param.id.clone().into();
            let info = VariableInfo::new(&param.ty);
            self.variables.insert(id, info);
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    params: Vec<Type>,
    return_type: Type,
}

impl FunctionSignature {
    pub fn new(params: Vec<Type>, return_type: Type) -> Self {
        Self {
            params,
            return_type,
        }
    }

    pub fn params(&self) -> &[Type] {
        &self.params
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }
}

#[derive(Clone, Default)]
pub struct SemanticInfo {
    expr_types: Vec<Type>,
    variable_types: HashMap<VariableId, Type>,
    function_signatures: HashMap<FunctionId, FunctionSignature>,
}
impl SemanticInfo {
    pub fn new() -> Self {
        Self {
            expr_types: Vec::new(),
            variable_types: HashMap::new(),
            function_signatures: HashMap::new(),
        }
    }

    pub fn add_variable_type(&mut self, id: &VariableId, ty: &Type) {
        self.variable_types.insert(id.clone(), ty.clone());
    }

    pub fn get_variable_type(&self, id: &VariableId) -> Option<&Type> {
        self.variable_types.get(id)
    }

    pub fn add_expr_type(&mut self, id: &ExprId, ty: &Type) {
        let index = id.index();

        if index >= self.expr_types.len() {
            self.expr_types.resize(index + 1, Type::Unknown);
        }

        self.expr_types[index] = ty.clone();
    }

    pub fn expr_type(&self, id: &ExprId) -> &Type {
        &self.expr_types[id.index()]
    }

    pub fn get_function_signature(&self, id: &FunctionId) -> Option<&FunctionSignature> {
        self.function_signatures.get(id)
    }

    pub fn add_function_signature(
        &mut self,
        id: &FunctionId,
        function_signature: &FunctionSignature,
    ) {
        self.function_signatures
            .insert(id.clone(), function_signature.clone());
    }

    pub fn add_function_params(&mut self, params: &Vec<Parameter>) {
        for param in params {
            let id = param.id.clone().into();
            let ty = param.ty.clone();
            self.add_variable_type(&id, &ty);
        }
    }
}

pub struct SemanticAnalyzer {
    semantic_info: SemanticInfo,
    scopes: Vec<Scope>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            semantic_info: SemanticInfo::new(),
            scopes: Vec::new(),
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<SemanticInfo, Vec<SemanticError>> {
        self.enter_scope();
        let mut errors: Vec<SemanticError> = Vec::new();
        self.collect_function_signatures(&program.declarations);
        for decl in &program.declarations {
            match decl {
                Decl::Function(function) => {
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
        self.scopes.push(Scope::new());
    }

    fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    fn get_current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn get_variable(&mut self, id: &VariableId) -> Option<&VariableInfo> {
        for scope in &self.scopes {
            if let Some(info) = scope.get_variable(id) {
                return Some(info);
            }
        }
        None
    }

    fn add_variable(&mut self, id: &VariableId, ty: &Type) {
        let info = VariableInfo::new(ty);
        let scope = self.get_current_scope();
        scope.add_variable(id, &info);
        self.semantic_info.add_variable_type(id, ty);
    }

    fn add_params(&mut self, params: &Vec<Parameter>) {
        self.get_current_scope()
            .add_function_params_to_scope(params);
        self.semantic_info.add_function_params(params);
    }

    fn collect_function_signatures(&mut self, declatarations: &Vec<Decl>) {
        for decl in declatarations {
            match decl {
                Decl::Function(function) => {
                    let mut params: Vec<Type> = Vec::new();
                    for param in &function.parameters {
                        params.push(param.ty.clone());
                    }
                    let function_signature =
                        FunctionSignature::new(params, function.return_type.clone());
                    self.semantic_info
                        .add_function_signature(&function.id, &function_signature);
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
                Stmt::Expr(_) => {}
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn check_let_assign_stmt(&mut self, id: &VariableId, expr: &Expr) -> Result<(), SemanticError> {
        let ty = self.infer_expr_type(expr)?;
        self.add_variable(id, &ty);
        Ok(())
    }

    fn check_function_return_type(&mut self, function: &Function) -> Result<(), SemanticError> {
        if function.id.get() == "main" && function.return_type != Type::Int {
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
                if let Some(info) = self.get_variable(id) {
                    info.ty().clone()
                } else {
                    return Err(SemanticError::UnknownVariableInExpr(
                        id.clone(),
                        kind.clone(),
                    ));
                }
            }
            ExprKind::Call { id, args } => {
                //todo!("Check if provided argument types match the function argument types")
                let mut arg_types: Vec<Type> = Vec::new();
                for arg in args {
                    arg_types.push(self.infer_expr_type(arg)?);
                }
                let function_sig = self.semantic_info.get_function_signature(id).unwrap();
                let expected_arg_count = function_sig.params.len();
                let received_arg_count = arg_types.len();
                if received_arg_count == expected_arg_count {
                    for (i, ty) in arg_types.iter().enumerate() {
                        if *ty != function_sig.params[i] {
                            return Err(SemanticError::IncorrectFunctionArgumentType(
                                kind.clone(),
                                function_sig.params[i].clone(),
                                arg_types[i].clone(),
                            ));
                        }
                    }
                    function_sig.return_type.clone()
                } else {
                    return Err(SemanticError::InvalidNumberOfFunctionArguments(
                        kind.clone(),
                        expected_arg_count,
                        received_arg_count,
                    ));
                }
            }
        };
        self.semantic_info.add_expr_type(&id, &ty);
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
    FunctionReturnTypeMismatch(FunctionId),
    MainFunctionInvalidReturnType(Type),
    InvalidBinaryOperands(BinOp, Type, Type),
    TypeMismatchInExpr(Type, Type, ExprKind),
    InvalidUnaryOperand(UnaryOp, Type),
    UnknownVariableInExpr(VariableId, ExprKind),
    InvalidNumberOfFunctionArguments(ExprKind, usize, usize),
    IncorrectFunctionArgumentType(ExprKind, Type, Type),
}

impl Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FunctionReturnTypeMismatch(id) => {
                let id = id.get();
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
