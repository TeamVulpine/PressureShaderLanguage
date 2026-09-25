use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parse_tree::{
        MismatchHandling, expect_parse,
        expr::{Expr, operator::postfix::PostfixOperationExpr},
        try_many_infallible, try_symbol,
    },
    source::Spanned,
    token::{Tokenizer, symbol::Symbol},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrefixOperator {
    Positive,   // +
    Negative,   // -
    Not,        // !
    BitwiseNot, // ~
}

#[derive(Debug)]
pub struct PrefixOperationExpr<'a> {
    pub operand: Box<Spanned<'a, Expr<'a>>>,
    pub operators: Spanned<'a, Box<[Spanned<'a, PrefixOperator>]>>,
}

impl PrefixOperator {
    pub fn try_parse<'a>(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Option<Spanned<'a, Self>> {
        const MAPPING: &[(Symbol, PrefixOperator)] = &[
            (Symbol::Add, PrefixOperator::Positive),
            (Symbol::Subtract, PrefixOperator::Negative),
            (Symbol::Not, PrefixOperator::Not),
            (Symbol::BitwiseNot, PrefixOperator::BitwiseNot),
        ];

        for (symbol, operator) in MAPPING {
            let Some(span) = try_symbol(tokenizer, diagnostics, *symbol) else {
                continue;
            };

            return Some(span.into_spanned(*operator));
        }

        return None;
    }
}

impl<'a> PrefixOperationExpr<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Expr<'a>>>, FatalParsingError> {
        let Some(operators) =
            try_many_infallible(tokenizer, diagnostics, PrefixOperator::try_parse)
        else {
            return PostfixOperationExpr::try_parse(tokenizer, diagnostics);
        };

        let expr = PostfixOperationExpr::expect_parse(
            tokenizer,
            diagnostics,
            MismatchHandling::ConsumeUntilSafe,
        )?;

        let span = operators.span + expr.span;

        return Ok(Some(span.into_spanned(Expr::PrefixOperation(Self {
            operand: Box::new(expr),
            operators,
        }))));
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
