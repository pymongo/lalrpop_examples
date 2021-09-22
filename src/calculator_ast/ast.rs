#[derive(Debug)]
pub enum BinaryOperand {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Expr {
    Expr(Box<Expr>, BinaryOperand, Box<Expr>),
    Num(u32),
}
