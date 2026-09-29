use std::{borrow::Cow, fmt::Display};

use thiserror::Error;

use crate::{
    parser::token::{
        Token, TokenError, TokenErrorKind, TokenKind, ident::PseudoKeyword, keyword::Keyword,
        symbol::Symbol,
    },
    report::{IntoReport, Report, ReportSeverity},
    source::SourceSpan,
};

#[derive(Debug, Error, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[error("Fatal parsing error. See diagnostics for more information.")]
pub struct FatalParsingError;

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    Token(TokenError),

    UnexpectedToken(SourceSpan),

    UnexpectedEof,

    ExpectedSymbol {
        symbol: Symbol,
        got: Box<DiagnosticKind>,
    },

    ExpectedKeyword {
        keyword: Keyword,
        got: Box<DiagnosticKind>,
    },

    ExpectedPseudoKeyword {
        keyword: PseudoKeyword,
        got: Box<DiagnosticKind>,
    },

    ExpectedIdent {
        got: Box<DiagnosticKind>,
    },

    ExpectedExpr {
        got: Box<DiagnosticKind>,
    },

    ExpectedTy {
        got: Box<DiagnosticKind>,
    },

    ExpectedGenericParam {
        got: Box<DiagnosticKind>,
    },

    ExpectedGenericArgument {
        got: Box<DiagnosticKind>,
    },

    ExpectedSymbolPath {
        got: Box<DiagnosticKind>,
    },

    ExpectedSymbolPathPart {
        got: Box<DiagnosticKind>,
    },

    ExpectedAttributeParameter {
        got: Box<DiagnosticKind>,
    },

    ExpectedAttribute {
        got: Box<DiagnosticKind>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub level: DiagnosticLevel,
    pub span: SourceSpan,
}

pub struct Diagnostics {
    diagnostics: Vec<Diagnostic>,
    has_error_or_fatal: bool,
}

impl Diagnostics {
    pub fn new() -> Self {
        return Self {
            diagnostics: Vec::new(),
            has_error_or_fatal: false,
        };
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        if diagnostic.level == DiagnosticLevel::Error || diagnostic.level == DiagnosticLevel::Fatal
        {
            self.has_error_or_fatal = true;
        }

        self.diagnostics.push(diagnostic);
    }

    pub fn push_warning(&mut self, kind: DiagnosticKind, span: SourceSpan) {
        self.push(Diagnostic {
            kind,
            level: DiagnosticLevel::Warning,
            span,
        });
    }

    pub fn push_error(&mut self, kind: DiagnosticKind, span: SourceSpan) {
        self.push(Diagnostic {
            kind,
            level: DiagnosticLevel::Error,
            span,
        });
    }

    pub fn push_fatal(
        &mut self,
        kind: DiagnosticKind,
        span: SourceSpan,
    ) -> Result<!, FatalParsingError> {
        self.push(Diagnostic {
            kind,
            level: DiagnosticLevel::Fatal,
            span,
        });

        return Err(FatalParsingError);
    }

    pub fn has_error_or_fatal(&self) -> bool {
        return self.has_error_or_fatal;
    }

    pub fn into_iter(self) -> impl Iterator<Item = Diagnostic> {
        self.diagnostics.into_iter()
    }
}

impl DiagnosticKind {
    pub fn from_token(token: Token) -> Self {
        if let TokenKind::Eof = token.kind {
            return Self::UnexpectedEof;
        }

        return Self::UnexpectedToken(token.span);
    }
}

impl From<TokenError> for Diagnostic {
    fn from(value: TokenError) -> Self {
        return Self {
            span: value.span,
            kind: DiagnosticKind::Token(value),
            level: DiagnosticLevel::Error,
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

impl IntoReport for Diagnostic {
    fn into_report(self) -> Report {
        return Report {
            severity: self.level.into(),
            message: self.kind.message(),
            span: Some(self.span),
            notes: Box::new([]),
        };
    }
}

impl DiagnosticKind {
    fn message(&self) -> Cow<'static, str> {
        match self {
            Self::Token(token_error) => {
                return Cow::Owned(token_error.kind.to_string());
            }

            Self::UnexpectedToken(_) => {
                return Cow::Owned(format!("found unexpected token."));
            }

            Self::UnexpectedEof => {
                return Cow::Borrowed("reached EOF.");
            }

            Self::ExpectedSymbol { symbol, got } => {
                return Cow::Owned(format!(
                    "expected symbol '{}', {}",
                    symbol.repr(),
                    got.message(),
                ));
            }

            Self::ExpectedKeyword { keyword, got } => {
                return Cow::Owned(format!(
                    "expected keyword '{}', {}",
                    keyword.repr(),
                    got.message(),
                ));
            }

            Self::ExpectedPseudoKeyword { keyword, got } => {
                return Cow::Owned(format!(
                    "expected keyword '{}', {}",
                    keyword.repr(),
                    got.message(),
                ));
            }

            Self::ExpectedIdent { got } => {
                return Cow::Owned(format!("expected identifier, {}", got.message()));
            }

            Self::ExpectedExpr { got } => {
                return Cow::Owned(format!("expected expression, {}", got.message()));
            }

            Self::ExpectedTy { got } => {
                return Cow::Owned(format!("expected type, {}", got.message()));
            }

            Self::ExpectedGenericParam { got } => {
                return Cow::Owned(format!(
                    "expected generic parameter or closing ';', {}",
                    got.message()
                ));
            }

            Self::ExpectedGenericArgument { got } => {
                return Cow::Owned(format!(
                    "expected generic argument or closing ']', {}",
                    got.message()
                ));
            }

            Self::ExpectedSymbolPath { got } => {
                return Cow::Owned(format!("expected symbol, {}", got.message()));
            }

            Self::ExpectedSymbolPathPart { got } => {
                return Cow::Owned(format!(
                    "expected identifier or generic arguments, {}",
                    got.message()
                ));
            }

            Self::ExpectedAttributeParameter { got } => {
                return Cow::Owned(format!("expected attribute parameter, {}", got.message()));
            }

            Self::ExpectedAttribute { got } => {
                return Cow::Owned(format!("expected attribute, {}", got.message()));
            }
        }
    }
}

impl From<DiagnosticLevel> for ReportSeverity {
    fn from(level: DiagnosticLevel) -> Self {
        return match level {
            DiagnosticLevel::Warning => Self::Warning,
            DiagnosticLevel::Error => Self::Error,
            DiagnosticLevel::Fatal => Self::Error,
        };
    }
}
