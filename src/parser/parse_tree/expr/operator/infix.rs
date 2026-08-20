use crate::parser::{
    diagnostic::Diagnostics,
    parse_tree::{expr::Expr, try_symbol},
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

pub struct InfixOperationExpr<'a> {
    pub operands: Box<[Expr<'a>; 2]>,
    pub operator: Spanned<'a, InfixOperator>,
}

impl InfixOperator {
    pub fn binding_power(self) -> (u8, u8) {
        match self {
            Self::Mul | Self::Div | Self::Mod => (70, 71),

            Self::Add | Self::Sub => (60, 61),

            Self::Shl | Self::Shr => (50, 51),

            Self::BitAnd => (45, 46),
            Self::BitXor => (44, 45),
            Self::BitOr => (43, 44),

            Self::Eq | Self::Ne | Self::Gt | Self::Lt | Self::Ge | Self::Le => (40, 41),

            Self::BoolAnd => (30, 31),
            Self::BoolOr => (20, 21),

            Self::Range | Self::RangeTo => (10, 11),
        }
    }

    pub fn try_parse<'a>(
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
            let Some(span) = try_symbol(tokenizer, diagnostics, *symbol) else {
                continue;
            };

            return Some(span.into_spanned(*operator));
        }

        return None;
    }
}
