use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parse_tree::{
        MismatchHandling, expect_ident, expect_parse, expect_symbol,
        expr::{Expr, atom::AtomExpr},
        symbol::generic::GenericArgument,
        try_many, try_symbol,
    },
    source::Spanned,
    token::{Tokenizer, ident::PseudoKeyword, symbol::Symbol},
};

#[derive(Debug)]
pub enum PostfixOperator<'a> {
    Field(Option<PseudoKeyword>),
    Invoke(Box<[Spanned<'a, Expr<'a>>]>),
    Access(Box<Expr<'a>>),
    Generics(Box<[Spanned<'a, GenericArgument<'a>>]>),
}

#[derive(Debug)]
pub struct PostfixOperationExpr<'a> {
    pub operand: Box<Spanned<'a, Expr<'a>>>,
    pub operators: Spanned<'a, Box<[Spanned<'a, PostfixOperator<'a>>]>>,
}

impl<'a> PostfixOperator<'a> {
    fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        if let Some(start) = try_symbol(tokenizer, diagnostics, Symbol::Dot) {
            let ident = expect_ident(tokenizer, diagnostics, MismatchHandling::Consume)?;

            let span = start + ident.span;

            return Ok(Some(span.into_spanned(Self::Field(ident.value))));
        }

        if let Some(values) = Expr::try_list(tokenizer, diagnostics)? {
            return Ok(Some(values.map(Self::Invoke)));
        }

        if let Some(start) = try_symbol(tokenizer, diagnostics, Symbol::BracketOpen) {
            let expr =
                Expr::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

            let end = expect_symbol(
                tokenizer,
                diagnostics,
                Symbol::BracketClose,
                MismatchHandling::Consume,
            )?;

            return Ok(Some(
                (start + end).into_spanned(Self::Access(Box::new(expr.value))),
            ));
        }

        if let Some(start) = try_symbol(tokenizer, diagnostics, Symbol::DoubleColon) {
            let generics = GenericArgument::expect_list(
                tokenizer,
                diagnostics,
                MismatchHandling::ConsumeUntilSafe,
            )?;

            let span = start + generics.span;

            return Ok(Some(span.into_spanned(Self::Generics(generics.value))));
        }

        return Ok(None);
    }

    pub fn try_many(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Box<[Spanned<'a, Self>]>>>, FatalParsingError> {
        return try_many(tokenizer, diagnostics, Self::try_parse);
    }
}

impl<'a> PostfixOperationExpr<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Expr<'a>>>, FatalParsingError> {
        let Some(expr) = AtomExpr::try_parse(tokenizer, diagnostics)? else {
            return Ok(None);
        };

        if let Some(operators) = PostfixOperator::try_many(tokenizer, diagnostics)? {
            let span = expr.span + operators.span;

            return Ok(Some(span.into_spanned(Expr::PostfixOperation(Self {
                operand: Box::new(expr),
                operators,
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
