use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{MismatchHandling, expect_parse, expr::Expr, try_keyword, try_list, ty::Ty},
        token::{Tokenizer, keyword::Keyword, symbol::Symbol},
    },
    source::Spanned,
};

#[derive(Debug)]
pub enum GenericArgument {
    Ty(Spanned<Ty>),
    Const(Spanned<Expr>),
    Error,
}

impl GenericArgument {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
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
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
        mismatch_handling: MismatchHandling,
    ) -> Result<Spanned<Self>, FatalParsingError> {
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
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Box<[Spanned<Self>]>>>, FatalParsingError> {
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
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
        mismatch_handling: MismatchHandling,
    ) -> Result<Spanned<Box<[Spanned<Self>]>>, FatalParsingError> {
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
