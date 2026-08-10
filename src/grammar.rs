#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(usize);

impl ExprId {
    pub fn new(idx: usize) -> Self {
        Self(idx)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Expr {
    id: ExprId,
    kind: ExprKind,
}

impl Expr {
    pub(crate) fn new(id: ExprId, kind: ExprKind) -> Self {
        Self { id, kind }
    }

    pub fn id(&self) -> ExprId {
        self.id
    }

    pub fn kind(&self) -> &ExprKind {
        &self.kind
    }
}

#[derive(Debug, Clone)]
pub enum ExprKind {
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

#[derive(Debug)]
pub struct Function {
    pub id: String,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Stmt>,
    pub return_type: Type,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Return(Expr),
    LetAssign { id: String, expr: Expr },
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<Decl>,
}

#[derive(Debug)]
pub enum Decl {
    Function(Function), // not a declaration in a C sense but just a definition
                        // later add things like struct declarations, constants, etc.
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Void,
    Unknown,
}
