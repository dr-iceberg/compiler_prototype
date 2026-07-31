#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
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

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,

    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub struct Function {
    id: String,
    parameters: Vec<Parameter>,
    body: Vec<Stmt>,
    return_type: Literal,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Return(Expr),
    Assign { id: String, expr: Expr },
}

pub struct Parameter {
    name: String,
    ty: Type,
}

pub struct Program {
    pub declarations: Vec<Decl>,
}

pub enum Decl {
    Function(Function), // not a declaration in a C sense but just a definition
                        // later add things like struct declarations, constants, etc.
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Void,
}
