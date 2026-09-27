use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, expect_parse,
            expr::{Expr, operator::postfix::PostfixOperationExpr},
            try_many_infallible, try_symbol,
        },
        token::{Tokenizer, symbol::Symbol},
    },
    source::Spanned,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrefixOperator {
    Positive,   // +
    Negative,   // -
    Not,        // !
    BitwiseNot, // ~
}

#[derive(Debug)]
pub struct PrefixOperationExpr {
    pub operand: Box<Spanned<Expr>>,
    pub operators: Spanned<Box<[Spanned<PrefixOperator>]>>,
}

impl PrefixOperator {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Option<Spanned<Self>> {
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

impl PrefixOperationExpr {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Expr>>, FatalParsingError> {
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
