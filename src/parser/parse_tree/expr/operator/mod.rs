use crate::parser::parse_tree::expr::operator::{
    infix::InfixOperationExpr, postfix::PostfixOperationExpr, prefix::PrefixOperationExpr,
};

pub mod infix;
pub mod postfix;
pub mod prefix;

pub enum OperationExpr<'a> {
    Infix(InfixOperationExpr<'a>),
    Prefix(PrefixOperationExpr<'a>),
    Postfix(PostfixOperationExpr<'a>),
}
