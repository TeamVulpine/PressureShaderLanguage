use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    source::SourceSpan,
    token::{Token, TokenKind, Tokenizer, ident::PseudoKeyword, keyword::Keyword, symbol::Symbol},
};

pub mod expr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MismatchHandling {
    Skip,
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

    fn consume_on_mismatch(&self) -> bool {
        return *self == Self::Consume;
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

    if mismatch_handling.consume_on_mismatch() {
        _ = tokenizer.next();
    }

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
