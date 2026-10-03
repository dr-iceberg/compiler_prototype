/// Used for saving expression type
/// in order to fetch the expression type during codegen
/// without having to reevaluate the expression
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
    // todo: Add a span here
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
    Identifier(VariableId),
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
        id: FunctionId,
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

pub trait SymbolId {
    fn get(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableId(String);

impl VariableId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl From<ParameterId> for VariableId {
    fn from(value: ParameterId) -> Self {
        Self(value.0)
    }
}

impl SymbolId for VariableId {
    fn get(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionId(String);

impl FunctionId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl SymbolId for FunctionId {
    fn get(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParameterId(String);

impl ParameterId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl SymbolId for ParameterId {
    fn get(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub id: FunctionId,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Stmt>,
    pub return_type: Type,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Return(Expr),
    LetAssign { id: VariableId, expr: Expr },
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub id: ParameterId,
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
