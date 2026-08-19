use std::num::NonZeroU32;

use crate::parser::{
    source::{SourceCursor, SourceSpan},
    token::{TokenError, TokenErrorKind, Tokenizer},
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
pub struct NumberLiteral {
    pub kind: NumberLiteralKind,
    pub suffix_start: Option<NonZeroU32>,
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

    pub fn parse(&self, text: &str) -> u32 {
        return u32::from_str_radix(&text.replace('_', ""), self.base()).unwrap();
    }
}

impl NumberLiteral {
    pub fn decompose<'a>(&self, span: &SourceSpan<'a>) -> (&'a str, &'a str) {
        let Some(suffix) = self.suffix_start else {
            return (span.slice(), "");
        };

        return span.slice().split_at(suffix.get() as usize);
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

    fn parse_digits<'a>(cursor: &mut SourceCursor<'a>, base: IntegerBase) -> Result<bool, TokenError<'a>> {
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

    fn parse_suffix(cursor: &mut SourceCursor) -> Option<NonZeroU32> {
        if !cursor.is_fn(Tokenizer::is_ident_start) {
            return None;
        }

        let pos = cursor.relative_offset();

        cursor.while_fn(Tokenizer::is_ident_cont);

        return NonZeroU32::new(pos);
    }

    pub fn parse<'a>(cursor: &mut SourceCursor<'a>) -> Result<Option<Self>, TokenError<'a>> {
        let base = Self::parse_base(cursor);

        if !Self::parse_digits(cursor, base)? {
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
                suffix_start: Self::parse_suffix(cursor),
            }));
        };

        if !cursor.is_fn(|c| c == '.' || c == 'e' || c == 'E') {
            return Ok(Some(Self {
                kind: NumberLiteralKind::Integer(base),
                suffix_start: Self::parse_suffix(cursor),
            }));
        }

        if cursor.is_char_and_then_fn('.', Self::is_decimal_digit) {
            cursor.advance();

            Self::parse_digits(cursor, IntegerBase::Decimal)?;
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
            suffix_start: Self::parse_suffix(cursor),
        }));
    }
}
