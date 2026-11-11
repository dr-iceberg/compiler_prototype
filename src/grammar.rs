use crate::ast::NodeId;

#[derive(Debug, Clone)]
pub struct Expr {
    pub id: NodeId,
    pub kind: ExprKind,
    // todo: Add a span here
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Literal(Literal),
    Identifier {
        id: NodeId,
        name: String,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Call {
        id: NodeId,
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Interger(i128),
    Float(f64),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    /// example: `-` in `-1`
    Negate,
    /// example: `!` in `!x`
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    /// `+`
    Add,
    /// `-`
    Subtract,
    /// `*`
    Multiply,
    /// `/`
    Divide,

    /// `==`
    Equal,
    /// `!=`
    NotEqual,

    /// `<`
    Less,
    /// `<=`
    LessEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterEqual,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub id: NodeId,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Stmt>,
    pub return_type: Type,
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub id: NodeId,
    pub kind: StmtKind,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Return(Expr),
    LetAssign {
        id: NodeId,
        name: String,
        expr: Expr,
    },
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub id: NodeId,
    pub name: String,
    pub ty: Type,
}

#[derive(Debug)]
pub struct Program {
    pub id: NodeId,
    pub declarations: Vec<Decl>,
}

#[derive(Debug)]
pub struct Decl {
    pub id: NodeId,
    pub kind: DeclKind,
}
#[derive(Debug)]
pub enum DeclKind {
    Function(Function), // not a declaration in a C sense but just a definition
                        // later add things like struct declarations, constants, etc.
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Type {
    #[default]
    Unknown,
    Int,
    Float,
    Bool,
    String,
    Void,
}
