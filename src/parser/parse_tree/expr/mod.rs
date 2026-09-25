use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parse_tree::{
        MismatchHandling, expect_parse,
        expr::{
            atom::AtomExpr,
            operator::{
                infix::InfixOperationExpr, postfix::PostfixOperationExpr,
                prefix::PrefixOperationExpr,
            },
        },
    },
    source::Spanned,
    token::Tokenizer,
};

pub mod atom;
pub mod operator;

#[derive(Debug)]
pub enum Expr<'a> {
    PostfixOperation(PostfixOperationExpr<'a>),
    PrefixOperation(PrefixOperationExpr<'a>),
    InfixOperation(InfixOperationExpr<'a>),
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
}
