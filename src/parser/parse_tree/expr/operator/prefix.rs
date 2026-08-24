use crate::parser::{parse_tree::expr::Expr, source::Spanned};

pub enum PrefixOperator {}

pub struct PrefixOperationExpr<'a> {
    pub operand: Box<Spanned<'a, Expr<'a>>>,
    pub operator: Spanned<'a, PrefixOperator>,
}
