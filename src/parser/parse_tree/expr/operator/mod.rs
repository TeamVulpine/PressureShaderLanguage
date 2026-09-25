use crate::parser::parse_tree::expr::operator::{
    infix::InfixOperationExpr, postfix::PostfixOperationExpr, prefix::PrefixOperationExpr,
};

pub mod infix;
pub mod postfix;
pub mod prefix;
