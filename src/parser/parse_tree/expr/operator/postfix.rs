use crate::parser::{parse_tree::expr::Expr, source::Spanned};

pub enum PostfixOperator {}

pub struct PostfixOperationExpr<'a> {
    pub operand: Box<Spanned<'a, Expr<'a>>>,
    pub operator: Spanned<'a, PostfixOperator>,
}
