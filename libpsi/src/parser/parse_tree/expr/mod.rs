use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
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
        token::{Tokenizer, symbol::Symbol},
    },
    source::Spanned,
};

pub mod atom;
pub mod operator;

#[derive(Debug)]
pub enum Expr {
    PostfixOperation(PostfixOperationExpr),
    PrefixOperation(PrefixOperationExpr),
    InfixOperation(InfixOperationExpr),
    CastOperation(CastOperationExpression),
    Atom(AtomExpr),
    Error,
}

impl Expr {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        return InfixOperationExpr::try_parse(tokenizer, diagnostics, 0);
    }

    pub fn expect_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
        mismatch_handling: MismatchHandling,
    ) -> Result<Spanned<Self>, FatalParsingError> {
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
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Box<[Spanned<Self>]>>>, FatalParsingError> {
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
