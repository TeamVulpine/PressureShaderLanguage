use std::fmt::Display;

use thiserror::Error;

use crate::parser::{
    source::SourceSpan,
    token::{
        Token, TokenError, TokenErrorKind, TokenKind, ident::PseudoKeyword, keyword::Keyword,
        symbol::Symbol,
    },
};

#[derive(Debug, Error, Clone)]
pub enum DiagnosticKind<'a> {
    #[error("failed to parse token: {0}")]
    Token(#[from] TokenErrorKind),

    #[error("found unexpected token {0:?}.")]
    UnexpectedToken(SourceSpan<'a>),

    #[error("reached EOF.")]
    UnexpectedEof,

    #[error("expected symbol '{}', {got}", symbol.repr())]
    ExpectedSymbol {
        symbol: Symbol,
        got: Box<DiagnosticKind<'a>>,
    },
    #[error("expected keyword '{}', {got}", keyword.repr())]
    ExpectedKeyword {
        keyword: Keyword,
        got: Box<DiagnosticKind<'a>>,
    },
    #[error("expected keyword '{}', {got}", keyword.repr())]
    ExpectedPseudoKeyword {
        keyword: PseudoKeyword,
        got: Box<DiagnosticKind<'a>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Error, Clone)]
#[error("{level} ({span}): {kind}")]
pub struct Diagnostic<'a> {
    pub kind: DiagnosticKind<'a>,
    pub level: DiagnosticLevel,
    pub span: SourceSpan<'a>,
}

pub struct Diagnostics<'a> {
    diagnostics: Vec<Diagnostic<'a>>,
    has_error_or_fatal: bool,
}

impl<'a> Diagnostics<'a> {
    pub fn new() -> Self {
        return Self {
            diagnostics: Vec::new(),
            has_error_or_fatal: false,
        };
    }

    pub fn push(&mut self, diagnostic: Diagnostic<'a>) {
        if diagnostic.level == DiagnosticLevel::Error || diagnostic.level == DiagnosticLevel::Fatal
        {
            self.has_error_or_fatal = true;
        }

        self.diagnostics.push(diagnostic);
    }

    pub fn has_error_or_fatal(&self) -> bool {
        return self.has_error_or_fatal;
    }

    pub fn into_iter(self) -> impl Iterator<Item = Diagnostic<'a>> {
        self.diagnostics.into_iter()
    }
}

impl<'a> DiagnosticKind<'a> {
    pub fn from_token(token: Token<'a>) -> Self {
        if let TokenKind::Eof = token.kind {
            return Self::UnexpectedEof;
        }

        return Self::UnexpectedToken(token.span);
    }
}

impl<'a> From<TokenError<'a>> for Diagnostic<'a> {
    fn from(value: TokenError<'a>) -> Self {
        return Self {
            kind: value.kind.into(),
            level: DiagnosticLevel::Error,
            span: value.span,
        };
    }
}

impl Display for DiagnosticLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Warning => f.write_str("Warning"),
            Self::Error => f.write_str("Error"),
            Self::Fatal => f.write_str("Fatal"),
        };
    }
}
