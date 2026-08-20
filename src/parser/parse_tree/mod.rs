use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    source::{SourceSpan, Spanned},
    token::{Token, TokenKind, Tokenizer, ident::PseudoKeyword, keyword::Keyword, symbol::Symbol},
};

pub mod expr;
pub mod path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MismatchHandling {
    Skip,
    SkipTo(&'static [TokenKind]),
    Consume,
    Fatal,
}

impl MismatchHandling {
    fn push_diagnostic<'a>(
        &self,
        diagnostics: &mut Diagnostics<'a>,
        kind: DiagnosticKind<'a>,
        span: SourceSpan<'a>,
    ) -> Result<(), FatalParsingError> {
        if let Self::Fatal = self {
            diagnostics.push_fatal(kind, span)?;
        }

        diagnostics.push_error(kind, span);

        return Ok(());
    }

    fn apply_mismatch<'a>(&self, tokenizer: &mut Tokenizer<'a>, diagnostics: &mut Diagnostics<'a>) {
        match self {
            Self::Consume => {
                _ = tokenizer.next();
            }
            Self::SkipTo(tokens) => {
                _ = tokenizer.next();

                loop {
                    let peek = peek_token(tokenizer, diagnostics);

                    if tokens.contains(&peek.kind) || peek.kind == TokenKind::Eof {
                        break;
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn peek_token<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
) -> Token<'a> {
    loop {
        match tokenizer.peek() {
            Ok(it) => return it,
            Err(err) => {
                _ = tokenizer.next();

                diagnostics.push_error(err.kind.into(), err.span);
            }
        }
    }
}

pub fn try_ident<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
) -> Option<Spanned<'a, Option<PseudoKeyword>>> {
    let token = peek_token(tokenizer, diagnostics);

    let TokenKind::Identifier(pseudo) = token.kind else {
        return None;
    };

    _ = tokenizer.next();

    return Some(token.span.into_spanned(pseudo));
}

pub fn expect_ident<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    mismatch_handling: MismatchHandling,
) -> Result<Option<Spanned<'a, Option<PseudoKeyword>>>, FatalParsingError> {
    let token = peek_token(tokenizer, diagnostics);

    let TokenKind::Identifier(pseudo) = token.kind else {
        mismatch_handling.push_diagnostic(
            diagnostics,
            DiagnosticKind::ExpectedIdent {
                got: Box::new(DiagnosticKind::from_token(token)),
            },
            token.span,
        )?;

        mismatch_handling.apply_mismatch(tokenizer, diagnostics);

        return Ok(None);
    };

    _ = tokenizer.next();

    return Ok(Some(token.span.into_spanned(pseudo)));
}

pub fn try_token<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    expected_token: TokenKind,
) -> Option<SourceSpan<'a>> {
    let token = peek_token(tokenizer, diagnostics);

    if token.kind == expected_token {
        _ = tokenizer.next();
        return Some(token.span);
    }

    return None;
}

pub fn try_symbol<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    symbol: Symbol,
) -> Option<SourceSpan<'a>> {
    return try_token(tokenizer, diagnostics, TokenKind::Symbol(symbol));
}

pub fn try_keyword<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    keyword: Keyword,
) -> Option<SourceSpan<'a>> {
    return try_token(tokenizer, diagnostics, TokenKind::Keyword(keyword));
}

pub fn try_pseudo_keyword<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    keyword: PseudoKeyword,
) -> Option<SourceSpan<'a>> {
    return try_token(tokenizer, diagnostics, TokenKind::Identifier(Some(keyword)));
}

pub fn expect_token<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    expected_token: TokenKind,
    mismatch_handling: MismatchHandling,
    produce_diagnostic: impl FnOnce(DiagnosticKind<'a>) -> DiagnosticKind<'a>,
) -> Result<SourceSpan<'a>, FatalParsingError> {
    let token = peek_token(tokenizer, diagnostics);

    if token.kind == expected_token {
        _ = tokenizer.next();

        return Ok(token.span);
    }

    mismatch_handling.apply_mismatch(tokenizer, diagnostics);

    mismatch_handling.push_diagnostic(
        diagnostics,
        produce_diagnostic(DiagnosticKind::from_token(token)),
        token.span,
    )?;

    return Ok(token.span);
}

pub fn expect_symbol<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    symbol: Symbol,
    mismatch_handling: MismatchHandling,
) -> Result<SourceSpan<'a>, FatalParsingError> {
    return expect_token(
        tokenizer,
        diagnostics,
        TokenKind::Symbol(symbol),
        mismatch_handling,
        |diagnostic| DiagnosticKind::ExpectedSymbol {
            symbol,
            got: Box::new(diagnostic),
        },
    );
}

pub fn expect_keyword<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    keyword: Keyword,
    mismatch_handling: MismatchHandling,
) -> Result<SourceSpan<'a>, FatalParsingError> {
    return expect_token(
        tokenizer,
        diagnostics,
        TokenKind::Keyword(keyword),
        mismatch_handling,
        |diagnostic| DiagnosticKind::ExpectedKeyword {
            keyword,
            got: Box::new(diagnostic),
        },
    );
}

pub fn expect_pseudo_keyword<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    keyword: PseudoKeyword,
    mismatch_handling: MismatchHandling,
) -> Result<SourceSpan<'a>, FatalParsingError> {
    return expect_token(
        tokenizer,
        diagnostics,
        TokenKind::Identifier(Some(keyword)),
        mismatch_handling,
        |diagnostic| DiagnosticKind::ExpectedPseudoKeyword {
            keyword,
            got: Box::new(diagnostic),
        },
    );
}
