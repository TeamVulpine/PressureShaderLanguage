use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parse_tree::{
        MismatchHandling, expect_parse,
        expr::{Expr, operator::prefix::PrefixOperationExpr},
        try_keyword, try_many,
        ty::Ty,
    },
    source::Spanned,
    token::{Tokenizer, keyword::Keyword},
};

#[derive(Debug)]
pub struct CastOperator<'a> {
    pub ty: Spanned<'a, Ty<'a>>,
}

#[derive(Debug)]
pub struct CastOperationExpression<'a> {
    pub expr: Box<Spanned<'a, Expr<'a>>>,
    pub casts: Spanned<'a, Box<[Spanned<'a, CastOperator<'a>>]>>,
}

impl<'a> CastOperator<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        let Some(start) = try_keyword(tokenizer, diagnostics, Keyword::As) else {
            return Ok(None);
        };

        let ty = Ty::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        let span = start + ty.span;

        return Ok(Some(span.into_spanned(Self { ty })));
    }

    pub fn try_many(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Box<[Spanned<'a, Self>]>>>, FatalParsingError> {
        return try_many(tokenizer, diagnostics, Self::try_parse);
    }
}

impl<'a> CastOperationExpression<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Expr<'a>>>, FatalParsingError> {
        let Some(expr) = PrefixOperationExpr::try_parse(tokenizer, diagnostics)? else {
            return Ok(None);
        };

        if let Some(casts) = CastOperator::try_many(tokenizer, diagnostics)? {
            let span = expr.span + casts.span;

            return Ok(Some(span.into_spanned(Expr::CastOperation(Self {
                expr: Box::new(expr),
                casts,
            }))));
        }

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
