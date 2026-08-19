pub mod ident;
pub mod keyword;
pub mod number;
pub mod symbol;

macro_rules! keywords {
    (
        $(
            $(#[$enum_meta:meta])*
            $vis:vis enum $t:ident {
                $($name:ident = $str:literal),* $(,)?
            }
        )+
    ) => {
        $(
            $(#[$enum_meta])*
            $vis enum $t {
                $($name),*
            }

            impl $t {
                pub fn from(keyword: &str) -> Option<Self> {
                    return match keyword {
                        $(
                            $str => Some(Self::$name),
                        )*
                        _ => None,
                    }
                }

                pub fn repr(&self) -> &'static str {
                    return match self {
                        $(Self::$name => $str),*
                    };
                }
            }
        )+
    }
}
pub(crate) use keywords;
use thiserror::Error;

use crate::parser::{source::{SourceCursor, SourceSpan}, token::{ident::PseudoKeyword, keyword::Keyword, number::NumberLiteral, symbol::Symbol}};

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Identifier(Option<PseudoKeyword>),
    Keyword(Keyword),
    Symbol(Symbol),
    NumberLiteral(NumberLiteral),
    StringLiteral,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub span: SourceSpan<'a>,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, Error)]
pub enum TokenErrorKind {
    #[error("Unexpected character in input: '{0}'.")]
    UnexpectedCharacter(char),
    #[error("Unclosed multiline comment in input.")]
    UnclosedMultilineComment,
    #[error("Number with trailing underscore in input.")]
    NumberTrailingUnderscore,
    #[error("Expected digits in input.")]
    ExpectedDigits,
    #[error("Expected exponent in input.")]
    ExpectedExponentDigits,
    #[error("Unclosed string literal in input.")]
    UnclosedStringLiteral,
}

#[derive(Debug, Clone, Error)]
#[error("error ({span}): {kind}")]
pub struct TokenError<'a> {
    pub span: SourceSpan<'a>,
    pub kind: TokenErrorKind,
}

pub struct Tokenizer<'a> {
    cursor: SourceCursor<'a>,
    peek: Option<Token<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str, file_path: Option<&'a str>) -> Self {
        return Self {
            cursor: SourceCursor::new(source, file_path),
            peek: None,
        };
    }

    pub fn empty_span(&self) -> SourceSpan<'a> {
        return self.cursor.empty_span();
    }

    fn skip_whitespace(&mut self) -> bool {
        let consumed = self.cursor.while_fn(char::is_whitespace);

        let _ = self.cursor.commit();

        return consumed;
    }

    fn skip_comments(&mut self) -> Result<bool, TokenError<'a>> {
        if self.cursor.consume_str("//") {
            self.cursor.while_fn(|c| c != '\n');

            let _ = self.cursor.commit();

            return Ok(true);
        }

        if self.cursor.consume_str("/*") {
            while !self.cursor.consume_str("*/") {
                if self.cursor.is_eof() {
                    return Err(TokenError {
                        span: self.cursor.commit(),
                        kind: TokenErrorKind::UnclosedMultilineComment,
                    });
                }

                self.cursor.advance();
            }

            let _ = self.cursor.commit();

            return Ok(true);
        }

        return Ok(false);
    }

    fn skip_comments_and_whitespace(&mut self) -> Result<(), TokenError<'a>> {
        while self.skip_comments()? || self.skip_whitespace() {}

        return Ok(());
    }

    const fn is_ident_start(c: char) -> bool {
        return c.is_ascii_alphabetic() || c == '_';
    }

    const fn is_ident_cont(c: char) -> bool {
        return Self::is_ident_start(c) || c.is_ascii_digit();
    }

    fn parse_ident(&mut self) -> Option<Token<'a>> {
        if !self.cursor.is_fn(Self::is_ident_start) {
            return None;
        }

        self.cursor.while_fn(Self::is_ident_cont);

        let span = self.cursor.commit();

        if let Some(keyword) = Keyword::from(span.slice()) {
            return Some(Token {
                span,
                kind: TokenKind::Keyword(keyword),
            });
        }

        return Some(Token {
            kind: TokenKind::Identifier(PseudoKeyword::from(span.slice())),
            span,
        });
    }

    fn parse_symbol(&mut self) -> Option<Token<'a>> {
        let symbol = Symbol::parse(&mut self.cursor)?;

        return Some(Token {
            span: self.cursor.commit(),
            kind: TokenKind::Symbol(symbol),
        });
    }

    fn parse_number(&mut self) -> Result<Option<Token<'a>>, TokenError<'a>> {
        let Some(number) = NumberLiteral::parse(&mut self.cursor)? else {
            self.cursor.rollback();

            return Ok(None);
        };

        return Ok(Some(Token {
            span: self.cursor.commit(),
            kind: TokenKind::NumberLiteral(number),
        }));
    }

    fn parse_string(&mut self) -> Result<Option<Token<'a>>, TokenError<'a>> {
        if !self.cursor.consume_char('"') {
            return Ok(None);
        }

        self.cursor.while_fn(|c| c != '"');

        if !self.cursor.consume_char('"') {
            return Err(TokenError {
                span: self.cursor.commit(),
                kind: TokenErrorKind::UnclosedStringLiteral,
            });
        }

        return Ok(Some(Token {
            span: self.cursor.commit(),
            kind: TokenKind::StringLiteral,
        }));
    }

    fn next_raw(&mut self) -> Result<Token<'a>, TokenError<'a>> {
        self.skip_comments_and_whitespace()?;

        if self.cursor.is_eof() {
            return Ok(Token {
                span: self.cursor.commit(),
                kind: TokenKind::Eof,
            });
        }

        if let Some(token) = self.parse_ident() {
            return Ok(token);
        }

        if let Some(token) = self.parse_number()? {
            return Ok(token);
        }

        if let Some(token) = self.parse_symbol() {
            return Ok(token);
        }

        if let Some(token) = self.parse_string()? {
            return Ok(token);
        }

        let c = self.cursor.current().unwrap_or('\0');
        self.cursor.advance();

        return Err(TokenError {
            span: self.cursor.commit(),
            kind: TokenErrorKind::UnexpectedCharacter(c),
        });
    }

    #[must_use]
    pub fn next(&mut self) -> Result<Token<'a>, TokenError<'a>> {
        if let Some(peek) = self.peek.take() {
            return Ok(peek);
        }

        return self.next_raw();
    }

    #[must_use]
    pub fn peek(&mut self) -> Result<Token<'a>, TokenError<'a>> {
        if let Some(peek) = self.peek.clone() {
            return Ok(peek);
        }

        let peek = self.next_raw()?;

        self.peek = Some(peek.clone());

        return Ok(peek);
    }
}
