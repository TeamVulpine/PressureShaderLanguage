use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    source::{SourceSpan, Spanned},
    token::{
        Token, TokenKind, Tokenizer, ident::PseudoKeyword, keyword::Keyword, number::NumberLiteral,
        symbol::Symbol,
    },
};

pub mod expr;
pub mod symbol;
pub mod ty;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MismatchHandling {
    Skip,
    Consume,
    ConsumeUntilSafe,
    Fatal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RecurseToken {
    Paren,
    Brace,
    Bracket,
}

impl RecurseToken {
    fn from_open(symbol: Symbol) -> Option<Self> {
        return match symbol {
            Symbol::ParenOpen => Some(Self::Paren),
            Symbol::BraceOpen => Some(Self::Brace),
            Symbol::BracketOpen => Some(Self::Bracket),
            _ => None,
        };
    }

    fn is_close(self, symbol: Symbol) -> bool {
        return self.get_close() == symbol;
    }

    fn get_close(self) -> Symbol {
        return match self {
            Self::Paren => Symbol::ParenClose,
            Self::Brace => Symbol::BraceClose,
            Self::Bracket => Symbol::BracketClose,
        };
    }
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

    fn apply_mismatch<'a>(
        &self,
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<(), FatalParsingError> {
        match self {
            Self::Consume => {
                _ = tokenizer.next();

                return Ok(());
            }
            Self::ConsumeUntilSafe => {
                let mut stack = vec![];

                let mut first = true;

                loop {
                    if first {
                        first = false
                    } else {
                        _ = tokenizer.next();
                    }

                    let peek = peek_token(tokenizer, diagnostics);

                    let TokenKind::Symbol(symbol) = peek.kind else {
                        if peek.kind == TokenKind::Eof {
                            return Ok(());
                        }

                        continue;
                    };

                    if let Some(recurse) = RecurseToken::from_open(symbol) {
                        stack.push(recurse);

                        continue;
                    }

                    let Some(recurse) = stack.last() else {
                        let (Symbol::ParenClose
                        | Symbol::BraceClose
                        | Symbol::BracketClose
                        | Symbol::Semicolon
                        | Symbol::Comma) = symbol
                        else {
                            continue;
                        };

                        return Ok(());
                    };

                    if recurse.is_close(symbol) {
                        stack.pop();
                    } else if let Symbol::ParenClose | Symbol::BraceClose | Symbol::BracketClose =
                        symbol
                    {
                        diagnostics.push_fatal(
                            DiagnosticKind::ExpectedSymbol {
                                symbol: recurse.get_close(),
                                got: Box::new(DiagnosticKind::UnexpectedToken(peek.span)),
                            },
                            peek.span,
                        )?;
                    }
                }
            }
            _ => {
                return Ok(());
            }
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

pub fn peek_symbol<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    symbol: Symbol,
) -> Option<SourceSpan<'a>> {
    let token = peek_token(tokenizer, diagnostics);

    if token.kind == TokenKind::Symbol(symbol) {
        return Some(token.span);
    }

    return None;
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

pub fn try_number<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
) -> Option<Spanned<'a, NumberLiteral>> {
    let token = peek_token(tokenizer, diagnostics);

    let TokenKind::NumberLiteral(number) = token.kind else {
        return None;
    };

    _ = tokenizer.next();

    return Some(token.span.into_spanned(number));
}

pub fn expect_ident<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    mismatch_handling: MismatchHandling,
) -> Result<Spanned<'a, Option<PseudoKeyword>>, FatalParsingError> {
    let token = peek_token(tokenizer, diagnostics);

    let TokenKind::Identifier(pseudo) = token.kind else {
        mismatch_handling.push_diagnostic(
            diagnostics,
            DiagnosticKind::ExpectedIdent {
                got: Box::new(DiagnosticKind::from_token(token)),
            },
            token.span,
        )?;

        mismatch_handling.apply_mismatch(tokenizer, diagnostics)?;

        return Ok(token.span.into_spanned(None));
    };

    _ = tokenizer.next();

    return Ok(token.span.into_spanned(pseudo));
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

pub fn expect_parse<'a, T>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    mismatch_handling: MismatchHandling,
    try_parse: impl FnOnce(
        &mut Tokenizer<'a>,
        &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, T>>, FatalParsingError>,
    produce_diagnostic: impl FnOnce(DiagnosticKind<'a>) -> DiagnosticKind<'a>,
    produce_fallback: impl FnOnce() -> T,
) -> Result<Spanned<'a, T>, FatalParsingError> {
    let Some(result) = try_parse(tokenizer, diagnostics)? else {
        let token = peek_token(tokenizer, diagnostics);
        mismatch_handling.push_diagnostic(
            diagnostics,
            produce_diagnostic(DiagnosticKind::from_token(token)),
            token.span,
        )?;

        mismatch_handling.apply_mismatch(tokenizer, diagnostics)?;

        return Ok(token.span.into_spanned(produce_fallback()));
    };

    return Ok(result);
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

    mismatch_handling.push_diagnostic(
        diagnostics,
        produce_diagnostic(DiagnosticKind::from_token(token)),
        token.span,
    )?;

    mismatch_handling.apply_mismatch(tokenizer, diagnostics)?;

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

pub fn try_list<'a, T>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    expect_parse: impl Fn(
        &mut Tokenizer<'a>,
        &mut Diagnostics<'a>,
        MismatchHandling,
    ) -> Result<Spanned<'a, T>, FatalParsingError>,
    starting_symbol: Symbol,
    closing_symbol: Symbol,
    delimiter: Symbol,
    allow_trailing: bool,
) -> Result<Option<Spanned<'a, Box<[Spanned<'a, T>]>>>, FatalParsingError> {
    let Some(start) = try_symbol(tokenizer, diagnostics, starting_symbol) else {
        return Ok(None);
    };

    let mut values = vec![];

    loop {
        if (values.is_empty() || allow_trailing)
            && let Some(end) = try_symbol(tokenizer, diagnostics, closing_symbol)
        {
            return Ok(Some((start + end).into_spanned(values.into())));
        }

        values.push(expect_parse(
            tokenizer,
            diagnostics,
            MismatchHandling::ConsumeUntilSafe,
        )?);

        if try_symbol(tokenizer, diagnostics, delimiter).is_some() {
            continue;
        }

        let end = expect_symbol(
            tokenizer,
            diagnostics,
            closing_symbol,
            MismatchHandling::Consume,
        )?;

        return Ok(Some((start + end).into_spanned(values.into())));
    }
}

pub fn try_many<'a, T>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    try_parse: impl Fn(
        &mut Tokenizer<'a>,
        &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, T>>, FatalParsingError>,
) -> Result<Option<Spanned<'a, Box<[Spanned<'a, T>]>>>, FatalParsingError> {
    let mut values = vec![];

    loop {
        let Some(value) = try_parse(tokenizer, diagnostics)? else {
            break;
        };

        values.push(value);
    }

    let Some(span) = values
        .first()
        .zip(values.last())
        .map(|(a, b)| a.span + b.span)
    else {
        return Ok(None);
    };

    return Ok(Some(span.into_spanned(values.into())));
}

pub fn try_many_infallible<'a, T>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    try_parse: impl Fn(&mut Tokenizer<'a>, &mut Diagnostics<'a>) -> Option<Spanned<'a, T>>,
) -> Option<Spanned<'a, Box<[Spanned<'a, T>]>>> {
    let mut values = vec![];

    loop {
        let Some(value) = try_parse(tokenizer, diagnostics) else {
            break;
        };

        values.push(value);
    }

    let Some(span) = values
        .first()
        .zip(values.last())
        .map(|(a, b)| a.span + b.span)
    else {
        return None;
    };

    return Some(span.into_spanned(values.into()));
}
