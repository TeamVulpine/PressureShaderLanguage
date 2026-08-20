use crate::parser::{
    source::{SourceSpan, Spanned},
    token::number::NumberLiteral,
};

pub enum LiteralExpr<'a> {
    Number(Spanned<'a, NumberLiteral>),
    Boolean(Spanned<'a, bool>),
    SelfValue(SourceSpan<'a>),
}
