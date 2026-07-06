#[derive(Debug, Clone)]
pub enum BinOps {
    Add,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Empty,
    Number(i64),
    Identifier(String),
    BinOp {
        left: Box<Expr>,
        op: BinOps,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Empty,
    Assign { id: String, expr: Expr },
}

#[derive(Debug)]
pub enum Scope {
    Scope,
    Function(Vec<Stmt>),
}
