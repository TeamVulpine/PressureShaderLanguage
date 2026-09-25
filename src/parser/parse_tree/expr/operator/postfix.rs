use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parse_tree::{
        MismatchHandling, expect_parse,
        expr::{Expr, atom::AtomExpr},
    },
    source::Spanned,
    token::Tokenizer,
};

#[derive(Debug)]
pub enum PostfixOperator {}

#[derive(Debug)]
pub struct PostfixOperationExpr<'a> {
    pub operand: Box<Spanned<'a, Expr<'a>>>,
    pub operators: Spanned<'a, Box<[Spanned<'a, PostfixOperator>]>>,
}

impl<'a> PostfixOperationExpr<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Expr<'a>>>, FatalParsingError> {
        let Some(expr) = AtomExpr::try_parse(tokenizer, diagnostics)? else {
            return Ok(None);
        };

        // We don't have any postfix operators yet.
        return Ok(Some(expr));
    }

    pub fn expect_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
        mismatch_handling: MismatchHandling,
    ) -> Result<Spanned<'a, Expr<'a>>, FatalParsingError> {
        return expect_parse(
            tokenizer,
            diagnostics,
            mismatch_handling,
            Self::try_parse,
            |diagnostic| DiagnosticKind::ExpectedExpr {
                got: Box::new(diagnostic),
            },
            || Expr::Error,
        );
    }
}
