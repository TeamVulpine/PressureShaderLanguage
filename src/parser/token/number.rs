use std::num::NonZeroU32;

use crate::parser::{
    source::{SourceCursor, SourceSpan},
    token::{TokenError, TokenErrorKind},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntegerBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberLiteralKind {
    Integer(IntegerBase),
    Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberSuffixKind {
    U,
    U8,
    U16,
    U32,
    U64,

    I,
    I8,
    I16,
    I32,
    I64,

    F,
    F16,
    F32,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumberSuffix {
    pub kind: NumberSuffixKind,
    pub start: NonZeroU32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumberLiteral {
    pub kind: NumberLiteralKind,
    pub suffix: Option<NumberSuffix>,
}

impl IntegerBase {
    pub fn prefix_size(&self) -> usize {
        let IntegerBase::Decimal = self else {
            return 2;
        };

        return 0;
    }

    pub fn base(&self) -> u32 {
        return match self {
            Self::Binary => 2,
            Self::Octal => 8,
            Self::Decimal => 10,
            Self::Hexadecimal => 16,
        };
    }
}

impl NumberLiteral {
    pub fn decompose<'a>(&self, span: &SourceSpan<'a>) -> (&'a str, Option<NumberSuffixKind>) {
        let Some(suffix) = self.suffix else {
            return (span.slice(), None);
        };

        return (
            &span.slice()[..suffix.start.get() as usize],
            Some(suffix.kind),
        );
    }

    fn parse_base(cursor: &mut SourceCursor) -> IntegerBase {
        if cursor.consume_str("0b") {
            return IntegerBase::Binary;
        }

        if cursor.consume_str("0o") {
            return IntegerBase::Octal;
        }

        if cursor.consume_str("0x") {
            return IntegerBase::Hexadecimal;
        }

        return IntegerBase::Decimal;
    }

    fn is_binary_digit(c: char) -> bool {
        return matches!(c, '0' | '1');
    }

    fn is_octal_digit(c: char) -> bool {
        return ('0'..'8').contains(&c);
    }

    fn is_hex_digit(c: char) -> bool {
        return c.is_ascii_hexdigit();
    }

    fn is_decimal_digit(c: char) -> bool {
        return c.is_ascii_digit();
    }

    fn parse_digits<'a>(
        cursor: &mut SourceCursor<'a>,
        base: IntegerBase,
    ) -> Result<bool, TokenError<'a>> {
        let matcher = {
            if let IntegerBase::Binary = base {
                Self::is_binary_digit
            } else if let IntegerBase::Octal = base {
                Self::is_octal_digit
            } else if let IntegerBase::Hexadecimal = base {
                Self::is_hex_digit
            } else {
                Self::is_decimal_digit
            }
        };

        if !cursor.is_fn(matcher) {
            return Ok(false);
        }

        while cursor.is_fn(matcher) {
            cursor.advance();
            if cursor.while_char('_') && !cursor.is_fn(matcher) {
                return Err(TokenError {
                    span: cursor.commit(),
                    kind: TokenErrorKind::NumberTrailingUnderscore,
                });
            }
        }

        return Ok(true);
    }

    fn parse_suffix(cursor: &mut SourceCursor) -> Option<NumberSuffix> {
        const KINDS: &[(&str, NumberSuffixKind)] = &[
            ("u", NumberSuffixKind::U),
            ("u8", NumberSuffixKind::U8),
            ("u16", NumberSuffixKind::U16),
            ("u32", NumberSuffixKind::U32),
            ("u64", NumberSuffixKind::U64),
            
            ("i", NumberSuffixKind::I),
            ("i8", NumberSuffixKind::I8),
            ("i16", NumberSuffixKind::I16),
            ("i32", NumberSuffixKind::I32),
            ("i64", NumberSuffixKind::I64),

            ("f", NumberSuffixKind::F),
            ("f16", NumberSuffixKind::F16),
            ("f32", NumberSuffixKind::F32),
            ("f64", NumberSuffixKind::F64),
        ];

        let start = cursor.relative_offset();

        for (suffix, kind) in KINDS {
            if cursor.consume_str(*suffix) {
                return Some(NumberSuffix {
                    kind: *kind,
                    start: NonZeroU32::new(start).unwrap(),
                });
            }
        }

        return None;
    }

    pub fn parse<'a>(cursor: &mut SourceCursor<'a>) -> Result<Option<Self>, TokenError<'a>> {
        let base = Self::parse_base(cursor);

        let parsed_whole = Self::parse_digits(cursor, base)?;

        if !parsed_whole {
            let IntegerBase::Decimal = base else {
                return Err(TokenError {
                    span: cursor.commit(),
                    kind: TokenErrorKind::ExpectedDigits,
                });
            };

            if !cursor.is_char('.') {
                return Ok(None);
            }
        };

        let IntegerBase::Decimal = base else {
            return Ok(Some(Self {
                kind: NumberLiteralKind::Integer(base),
                suffix: Self::parse_suffix(cursor),
            }));
        };

        if !cursor.is_fn(|c| c == '.' || c == 'e' || c == 'E') {
            return Ok(Some(Self {
                kind: NumberLiteralKind::Integer(base),
                suffix: Self::parse_suffix(cursor),
            }));
        }

        if cursor.is_char_and_then_fn('.', Self::is_decimal_digit) {
            cursor.advance();

            Self::parse_digits(cursor, IntegerBase::Decimal)?;
        } else if !parsed_whole {
            return Ok(None);
        }

        if cursor.is_fn(|c| c == 'e' || c == 'E') {
            cursor.advance();

            cursor.consume_fn(|c| c == '+' || c == '-');

            if !Self::parse_digits(cursor, IntegerBase::Decimal)? {
                return Err(TokenError {
                    span: cursor.commit(),
                    kind: TokenErrorKind::ExpectedExponentDigits,
                });
            }
        }

        return Ok(Some(Self {
            kind: NumberLiteralKind::Float,
            suffix: Self::parse_suffix(cursor),
        }));
    }
}
