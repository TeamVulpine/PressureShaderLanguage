use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, expect_parse,
            expr::{Expr, operator::prefix::PrefixOperationExpr},
            try_keyword, try_many,
            ty::Ty,
        },
        token::{Tokenizer, keyword::Keyword},
    },
    source::Spanned,
};

#[derive(Debug)]
pub struct CastOperator {
    pub ty: Spanned<Ty>,
}

#[derive(Debug)]
pub struct CastOperationExpression {
    pub expr: Box<Spanned<Expr>>,
    pub casts: Spanned<Box<[Spanned<CastOperator>]>>,
}

impl CastOperator {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let Some(start) = try_keyword(tokenizer, diagnostics, Keyword::As) else {
            return Ok(None);
        };

        let ty = Ty::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        let span = start + ty.span;

        return Ok(Some(span.into_spanned(Self { ty })));
    }

    pub fn try_many(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Box<[Spanned<Self>]>>>, FatalParsingError> {
        return try_many(tokenizer, diagnostics, Self::try_parse);
    }
}

impl CastOperationExpression {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Expr>>, FatalParsingError> {
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
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
        mismatch_handling: MismatchHandling,
    ) -> Result<Spanned<Expr>, FatalParsingError> {
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
