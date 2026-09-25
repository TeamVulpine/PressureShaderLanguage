use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parse_tree::{
        MismatchHandling, expect_parse,
        expr::{Expr, operator::prefix::PrefixOperationExpr},
        peek_symbol,
    },
    source::Spanned,
    token::{Tokenizer, symbol::Symbol},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InfixOperator {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %

    Eq, // ==
    Ne, // !=
    Gt, // >
    Lt, // <
    Ge, // >=
    Le, // <=

    BoolAnd, // &&
    BoolOr,  // ||

    BitAnd, // &
    BitOr,  // |
    BitXor, // ^

    Shl, // <<
    Shr, // >>

    Range,   // ..
    RangeTo, // ..=
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum InfixOperatorPrecedence {
    Range = 10,
    BoolOr = 20,
    BoolAnd = 30,
    Compare = 40,
    BitOr = 43,
    BitXor = 44,
    BitAnd = 45,
    Shift = 50,
    AddSub = 60,
    MulDiv = 70,
}

#[derive(Debug)]
pub struct InfixOperationExpr<'a> {
    pub operands: Box<[Spanned<'a, Expr<'a>>; 2]>,
    pub operator: Spanned<'a, InfixOperator>,
}

impl InfixOperator {
    const fn precedence(self) -> InfixOperatorPrecedence {
        return match self {
            Self::Mul | Self::Div | Self::Mod => InfixOperatorPrecedence::MulDiv,

            Self::Add | Self::Sub => InfixOperatorPrecedence::AddSub,

            Self::Shl | Self::Shr => InfixOperatorPrecedence::Shift,

            Self::BitAnd => InfixOperatorPrecedence::BitAnd,
            Self::BitXor => InfixOperatorPrecedence::BitXor,
            Self::BitOr => InfixOperatorPrecedence::BitOr,

            Self::Eq | Self::Ne | Self::Gt | Self::Lt | Self::Ge | Self::Le => {
                InfixOperatorPrecedence::Compare
            }

            Self::BoolAnd => InfixOperatorPrecedence::BoolAnd,
            Self::BoolOr => InfixOperatorPrecedence::BoolOr,

            Self::Range | Self::RangeTo => InfixOperatorPrecedence::Range,
        };
    }

    pub const fn binding_power(self) -> (u8, u8) {
        return self.precedence().binding_power();
    }

    pub fn peek_parse<'a>(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Option<Spanned<'a, Self>> {
        const MAPPING: &[(Symbol, InfixOperator)] = &[
            (Symbol::Add, InfixOperator::Add),
            (Symbol::Subtract, InfixOperator::Sub),
            (Symbol::Multiply, InfixOperator::Mul),
            (Symbol::Divide, InfixOperator::Div),
            (Symbol::Remainder, InfixOperator::Mod),
            (Symbol::Equal, InfixOperator::Eq),
            (Symbol::NotEqual, InfixOperator::Ne),
            (Symbol::GreaterThan, InfixOperator::Gt),
            (Symbol::LessThan, InfixOperator::Lt),
            (Symbol::GreaterThanOrEqual, InfixOperator::Ge),
            (Symbol::LessThanOrEqual, InfixOperator::Le),
            (Symbol::BooleanAnd, InfixOperator::BoolAnd),
            (Symbol::BooleanOr, InfixOperator::BoolOr),
            (Symbol::BitwiseAnd, InfixOperator::BitAnd),
            (Symbol::BitwiseOr, InfixOperator::BitOr),
            (Symbol::BitwiseXor, InfixOperator::BitXor),
            (Symbol::ShiftLeft, InfixOperator::Shl),
            (Symbol::ShiftRight, InfixOperator::Shr),
            (Symbol::Range, InfixOperator::Range),
            (Symbol::RangeTo, InfixOperator::RangeTo),
        ];

        for (symbol, operator) in MAPPING {
            let Some(span) = peek_symbol(tokenizer, diagnostics, *symbol) else {
                continue;
            };

            return Some(span.into_spanned(*operator));
        }

        return None;
    }
}

impl InfixOperatorPrecedence {
    const fn binding_power(&self) -> (u8, u8) {
        return (*self as u8, *self as u8 + 1);
    }
}

impl<'a> InfixOperationExpr<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
        min_binding: u8,
    ) -> Result<Option<Spanned<'a, Expr<'a>>>, FatalParsingError> {
        let Some(mut lhs) = PrefixOperationExpr::try_parse(tokenizer, diagnostics)? else {
            return Ok(None);
        };

        loop {
            let Some(op) = InfixOperator::peek_parse(tokenizer, diagnostics) else {
                break;
            };

            let (left_binding, right_binding) = op.value.binding_power();

            if left_binding < min_binding {
                break;
            }

            _ = tokenizer.next();

            let rhs = Self::expect_parse(
                tokenizer,
                diagnostics,
                MismatchHandling::ConsumeUntilSafe,
                right_binding,
            )?;

            let span = lhs.span + rhs.span;

            lhs = span.into_spanned(Expr::InfixOperation(Self {
                operands: Box::new([lhs, rhs]),
                operator: op,
            }));
        }

        return Ok(Some(lhs));
    }

    pub fn expect_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
        mismatch_handling: MismatchHandling,
        min_binding: u8,
    ) -> Result<Spanned<'a, Expr<'a>>, FatalParsingError> {
        return expect_parse(
            tokenizer,
            diagnostics,
            mismatch_handling,
            |tokenizer, diagnostics| Self::try_parse(tokenizer, diagnostics, min_binding),
            |diagnostic| DiagnosticKind::ExpectedExpr {
                got: Box::new(diagnostic),
            },
            || Expr::Error,
        );
    }
}
