use crate::parser::{parse_tree::expr::operator::OperationExpr, source::Spanned};

pub mod literal;
pub mod operator;

pub enum ExprKind<'a> {
    Operation(OperationExpr<'a>),
}

pub type Expr<'a> = Spanned<'a, ExprKind<'a>>;
