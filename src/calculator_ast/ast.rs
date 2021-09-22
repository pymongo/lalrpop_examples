#[derive(Debug)]
pub enum BinaryOperand {
    Add,
    Sub,
    Mul,
    Div,
}

/**
1. ExprAddSub 会 reduce 成 ExprMulDiv // { ExprAddSub AddSubOp ExprMulDiv, ExprMulDiv } 这两种情况的各个 Expr 都会往下 reduce
2. ExprMulDiv 可能会 reduce 成 Term
3. Term 有两种情况: base case 是数字，遇到括号则吃掉括号吐出 ExprAddSub 继续步骤 1

1 + (2 * 3 - 4) / 5
= ExprAddSub(ExprMulDiv(Term::Num(1)))) + ExprMulDiv
*/
#[derive(Debug)]
pub enum Expr {
    Expr(Box<Expr>, BinaryOperand, Box<Expr>),
    Num(i32),
}

impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Self::Expr(lhs, op, rhs) => {
                let (lhs, rhs) = (lhs.eval(), rhs.eval());
                match op {
                    BinaryOperand::Add => lhs + rhs,
                    BinaryOperand::Sub => lhs - rhs,
                    BinaryOperand::Mul => lhs * rhs,
                    BinaryOperand::Div => lhs / rhs,
                }
            },
            Self::Num(num) => *num
        }
    }
}
