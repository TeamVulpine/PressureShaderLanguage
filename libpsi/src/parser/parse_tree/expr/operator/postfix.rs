use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, expect_ident, expect_parse, expect_symbol,
            expr::{Expr, atom::AtomExpr},
            symbol::generic::GenericArgument,
            try_many, try_symbol,
        },
        token::{Tokenizer, ident::PseudoKeyword, symbol::Symbol},
    },
    source::Spanned,
};

#[derive(Debug)]
pub enum PostfixOperator {
    Field(Spanned<Option<PseudoKeyword>>),
    Invoke(Box<[Spanned<Expr>]>),
    Access(Box<Spanned<Expr>>),
    // Generics(Box<[Spanned<GenericArgument>]>),
}

#[derive(Debug)]
pub struct PostfixOperationExpr {
    pub operand: Box<Spanned<Expr>>,
    pub operators: Spanned<Box<[Spanned<PostfixOperator>]>>,
}

impl PostfixOperator {
    fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        if let Some(start) = try_symbol(tokenizer, diagnostics, Symbol::Dot) {
            let ident = expect_ident(tokenizer, diagnostics, MismatchHandling::Consume)?;

            let span = start + ident.span;

            return Ok(Some(span.into_spanned(Self::Field(ident))));
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
                (start + end).into_spanned(Self::Access(Box::new(expr))),
            ));
        }

        // if let Some(start) = try_symbol(tokenizer, diagnostics, Symbol::DoubleColon) {
        //     let generics = GenericArgument::expect_list(
        //         tokenizer,
        //         diagnostics,
        //         MismatchHandling::ConsumeUntilSafe,
        //     )?;

        //     let span = start + generics.span;

        //     return Ok(Some(span.into_spanned(Self::Generics(generics.value))));
        // }

        return Ok(None);
    }

    pub fn try_many(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Box<[Spanned<Self>]>>>, FatalParsingError> {
        return try_many(tokenizer, diagnostics, Self::try_parse);
    }
}

impl PostfixOperationExpr {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Expr>>, FatalParsingError> {
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
