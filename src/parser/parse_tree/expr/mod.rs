use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parse_tree::{
        MismatchHandling, expect_parse,
        expr::{
            atom::AtomExpr,
            operator::{
                cast::CastOperationExpression, infix::InfixOperationExpr,
                postfix::PostfixOperationExpr, prefix::PrefixOperationExpr,
            },
        },
        try_list,
    },
    source::Spanned,
    token::{Tokenizer, symbol::Symbol},
};

pub mod atom;
pub mod operator;

#[derive(Debug)]
pub enum Expr<'a> {
    PostfixOperation(PostfixOperationExpr<'a>),
    PrefixOperation(PrefixOperationExpr<'a>),
    InfixOperation(InfixOperationExpr<'a>),
    CastOperation(CastOperationExpression<'a>),
    Atom(AtomExpr<'a>),
    Error,
}

impl<'a> Expr<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        return InfixOperationExpr::try_parse(tokenizer, diagnostics, 0);
    }

    pub fn expect_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
        mismatch_handling: MismatchHandling,
    ) -> Result<Spanned<'a, Self>, FatalParsingError> {
        return expect_parse(
            tokenizer,
            diagnostics,
            mismatch_handling,
            Self::try_parse,
            |diagnostic| DiagnosticKind::ExpectedExpr {
                got: Box::new(diagnostic),
            },
            || Self::Error,
        );
    }

    pub fn try_list(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Box<[Spanned<'a, Self>]>>>, FatalParsingError> {
        return try_list(
            tokenizer,
            diagnostics,
            Self::expect_parse,
            Symbol::ParenOpen,
            Symbol::ParenClose,
            Symbol::Comma,
            true,
        );
    }
}
