use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parse_tree::{MismatchHandling, expect_parse, expr::Expr, try_keyword, try_list, ty::Ty},
    source::Spanned,
    token::{Tokenizer, keyword::Keyword, symbol::Symbol},
};

#[derive(Debug)]
pub enum GenericArgument<'a> {
    Ty(Spanned<'a, Ty<'a>>),
    Const(Spanned<'a, Expr<'a>>),
    Error,
}

impl<'a> GenericArgument<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        if let Some(start) = try_keyword(tokenizer, diagnostics, Keyword::Const) {
            let expr =
                Expr::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

            return Ok(Some((start + expr.span).into_spanned(Self::Const(expr))));
        }

        let Some(ty) = Ty::try_parse(tokenizer, diagnostics)? else {
            return Ok(None);
        };

        return Ok(Some(ty.span.clone().into_spanned(Self::Ty(ty))));
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
            |diagnostic| DiagnosticKind::ExpectedGenericArgument {
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
            Symbol::BracketOpen,
            Symbol::BracketClose,
            Symbol::Comma,
            true,
        );
    }

    pub fn expect_list(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
        mismatch_handling: MismatchHandling,
    ) -> Result<Spanned<'a, Box<[Spanned<'a, Self>]>>, FatalParsingError> {
        return expect_parse(
            tokenizer,
            diagnostics,
            mismatch_handling,
            Self::try_list,
            |diagnostic| DiagnosticKind::ExpectedGenericArgument {
                got: Box::new(diagnostic),
            },
            Default::default,
        );
    }
}
