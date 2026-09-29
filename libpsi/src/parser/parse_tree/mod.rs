use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::token::{
        Token, TokenKind, Tokenizer, ident::PseudoKeyword, keyword::Keyword, number::NumberLiteral,
        symbol::Symbol,
    },
    source::{SourceSpan, Spanned},
};

pub mod attr;
pub mod decl;
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
    fn push_diagnostic(
        &self,
        diagnostics: &mut Diagnostics,
        kind: DiagnosticKind,
        span: SourceSpan,
    ) -> Result<(), FatalParsingError> {
        if let Self::Fatal = self {
            diagnostics.push_fatal(kind, span)?;
        }

        diagnostics.push_error(kind, span);

        return Ok(());
    }

    fn apply_mismatch(
        &self,
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
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

fn peek_token<'a>(tokenizer: &mut Tokenizer<'a>, diagnostics: &mut Diagnostics) -> Token {
    loop {
        match tokenizer.peek() {
            Ok(it) => return it,
            Err(err) => {
                let span = err.span;

                diagnostics.push_error(DiagnosticKind::Token(err), span);
            }
        }
    }
}

fn peek_symbol(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    symbol: Symbol,
) -> Option<SourceSpan> {
    let token = peek_token(tokenizer, diagnostics);

    if token.kind == TokenKind::Symbol(symbol) {
        return Some(token.span);
    }

    return None;
}

fn try_ident(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
) -> Option<Spanned<Option<PseudoKeyword>>> {
    let token = peek_token(tokenizer, diagnostics);

    let TokenKind::Identifier(pseudo) = token.kind else {
        return None;
    };

    _ = tokenizer.next();

    return Some(token.span.into_spanned(pseudo));
}

fn try_string(tokenizer: &mut Tokenizer, diagnostics: &mut Diagnostics) -> Option<SourceSpan> {
    let token = peek_token(tokenizer, diagnostics);

    let TokenKind::StringLiteral = token.kind else {
        return None;
    };

    _ = tokenizer.next();

    return Some(token.span);
}

fn try_number(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
) -> Option<Spanned<NumberLiteral>> {
    let token = peek_token(tokenizer, diagnostics);

    let TokenKind::NumberLiteral(number) = token.kind else {
        return None;
    };

    _ = tokenizer.next();

    return Some(token.span.into_spanned(number));
}

fn expect_ident(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    mismatch_handling: MismatchHandling,
) -> Result<Spanned<Option<PseudoKeyword>>, FatalParsingError> {
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

fn try_token(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    expected_token: TokenKind,
) -> Option<SourceSpan> {
    let token = peek_token(tokenizer, diagnostics);

    if token.kind == expected_token {
        _ = tokenizer.next();
        return Some(token.span);
    }

    return None;
}

fn try_symbol(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    symbol: Symbol,
) -> Option<SourceSpan> {
    return try_token(tokenizer, diagnostics, TokenKind::Symbol(symbol));
}

fn try_keyword(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    keyword: Keyword,
) -> Option<SourceSpan> {
    return try_token(tokenizer, diagnostics, TokenKind::Keyword(keyword));
}

fn try_pseudo_keyword(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    keyword: PseudoKeyword,
) -> Option<SourceSpan> {
    return try_token(tokenizer, diagnostics, TokenKind::Identifier(Some(keyword)));
}

fn expect_parse<'a, T>(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    mismatch_handling: MismatchHandling,
    try_parse: impl FnOnce(
        &mut Tokenizer,
        &mut Diagnostics,
    ) -> Result<Option<Spanned<T>>, FatalParsingError>,
    produce_diagnostic: impl FnOnce(DiagnosticKind) -> DiagnosticKind,
    produce_fallback: impl FnOnce() -> T,
) -> Result<Spanned<T>, FatalParsingError> {
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

fn expect_token(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    expected_token: TokenKind,
    mismatch_handling: MismatchHandling,
    produce_diagnostic: impl FnOnce(DiagnosticKind) -> DiagnosticKind,
) -> Result<SourceSpan, FatalParsingError> {
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

fn expect_symbol(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    symbol: Symbol,
    mismatch_handling: MismatchHandling,
) -> Result<SourceSpan, FatalParsingError> {
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

fn expect_keyword(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    keyword: Keyword,
    mismatch_handling: MismatchHandling,
) -> Result<SourceSpan, FatalParsingError> {
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

fn expect_pseudo_keyword(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    keyword: PseudoKeyword,
    mismatch_handling: MismatchHandling,
) -> Result<SourceSpan, FatalParsingError> {
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

fn try_symbol_sequence(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    symbols: &[Symbol],
) -> Result<Option<SourceSpan>, FatalParsingError> {
    let [first, rest @ ..] = symbols else {
        return Ok(None);
    };

    let Some(mut span) = try_symbol(tokenizer, diagnostics, *first) else {
        return Ok(None);
    };

    for &symbol in rest {
        let current = expect_symbol(tokenizer, diagnostics, symbol, MismatchHandling::Consume)?;

        span = span + current;
    }

    return Ok(Some(span));
}

fn try_list<T>(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    expect_parse: impl Fn(
        &mut Tokenizer,
        &mut Diagnostics,
        MismatchHandling,
    ) -> Result<Spanned<T>, FatalParsingError>,
    starting_symbols: &[Symbol],
    closing_symbol: Symbol,
    delimiter: Symbol,
    allow_trailing: bool,
) -> Result<Option<Spanned<Box<[Spanned<T>]>>>, FatalParsingError> {
    let Some(start) = try_symbol_sequence(tokenizer, diagnostics, starting_symbols)? else {
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

fn try_list_pseudo_keyword<T>(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    expect_parse: impl Fn(
        &mut Tokenizer,
        &mut Diagnostics,
        MismatchHandling,
    ) -> Result<Spanned<T>, FatalParsingError>,
    starting_keyword: PseudoKeyword,
    closing_symbol: Symbol,
    delimiter: Symbol,
    allow_trailing: bool,
) -> Result<Option<Spanned<Box<[Spanned<T>]>>>, FatalParsingError> {
    let Some(start) = try_pseudo_keyword(tokenizer, diagnostics, starting_keyword) else {
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

enum TupleResult<T> {
    Parenthesized(Spanned<T>),
    Tuple(Spanned<Box<[Spanned<T>]>>),
    Unit(SourceSpan),
}

fn try_tuple<T>(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    expect_parse: impl Fn(
        &mut Tokenizer,
        &mut Diagnostics,
        MismatchHandling,
    ) -> Result<Spanned<T>, FatalParsingError>,
    starting_symbol: Symbol,
    closing_symbol: Symbol,
    delimiter: Symbol,
) -> Result<Option<TupleResult<T>>, FatalParsingError> {
    let Some(start) = try_symbol(tokenizer, diagnostics, starting_symbol) else {
        return Ok(None);
    };

    if let Some(end) = try_symbol(tokenizer, diagnostics, closing_symbol) {
        return Ok(Some(TupleResult::Unit(start + end)));
    }

    let first = expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

    let Some(_) = try_symbol(tokenizer, diagnostics, delimiter) else {
        let end = expect_symbol(
            tokenizer,
            diagnostics,
            closing_symbol,
            MismatchHandling::Consume,
        )?;

        return Ok(Some(TupleResult::Parenthesized(
            (start + end).into_spanned(first.value),
        )));
    };

    let mut values = vec![first];

    loop {
        if let Some(end) = try_symbol(tokenizer, diagnostics, closing_symbol) {
            return Ok(Some(TupleResult::Tuple(
                (start + end).into_spanned(values.into()),
            )));
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

        return Ok(Some(TupleResult::Tuple(
            (start + end).into_spanned(values.into()),
        )));
    }
}

fn try_many<'a, T>(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    try_parse: impl Fn(
        &mut Tokenizer,
        &mut Diagnostics,
    ) -> Result<Option<Spanned<T>>, FatalParsingError>,
) -> Result<Option<Spanned<Box<[Spanned<T>]>>>, FatalParsingError> {
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

fn try_many_infallible<'a, T>(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    try_parse: impl Fn(&mut Tokenizer, &mut Diagnostics) -> Option<Spanned<T>>,
) -> Option<Spanned<Box<[Spanned<T>]>>> {
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

fn try_one_or_more<T>(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    try_parse: impl Fn(
        &mut Tokenizer,
        &mut Diagnostics,
    ) -> Result<Option<Spanned<T>>, FatalParsingError>,
    expect_parse: impl Fn(
        &mut Tokenizer,
        &mut Diagnostics,
        MismatchHandling,
    ) -> Result<Spanned<T>, FatalParsingError>,
    delimiter: Symbol,
) -> Result<Option<Spanned<Box<[Spanned<T>]>>>, FatalParsingError> {
    let Some(first) = try_parse(tokenizer, diagnostics)? else {
        return Ok(None);
    };

    let mut values = vec![first];

    while try_symbol(tokenizer, diagnostics, delimiter).is_some() {
        values.push(expect_parse(
            tokenizer,
            diagnostics,
            MismatchHandling::ConsumeUntilSafe,
        )?);
    }

    let span = values
        .first()
        .zip(values.last())
        .map(|(a, b)| a.span + b.span)
        .unwrap();

    return Ok(Some(span.into_spanned(values.into())));
}

fn expect_one_or_more<T>(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
    expect_parse: impl Fn(
        &mut Tokenizer,
        &mut Diagnostics,
        MismatchHandling,
    ) -> Result<Spanned<T>, FatalParsingError>,
    delimiter: Symbol,
) -> Result<Spanned<Box<[Spanned<T>]>>, FatalParsingError> {
    let first = expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

    let mut values = vec![first];

    while try_symbol(tokenizer, diagnostics, delimiter).is_some() {
        values.push(expect_parse(
            tokenizer,
            diagnostics,
            MismatchHandling::ConsumeUntilSafe,
        )?);
    }

    let span = values
        .first()
        .zip(values.last())
        .map(|(a, b)| a.span + b.span)
        .unwrap();

    return Ok(span.into_spanned(values.into()));
}

fn try_infallible<T>(
    try_parse: impl Fn(&mut Tokenizer, &mut Diagnostics) -> Option<Spanned<T>>,
) -> impl Fn(&mut Tokenizer, &mut Diagnostics) -> Result<Option<Spanned<T>>, FatalParsingError> {
    return move |tokenizer, diagnostics| Ok(try_parse(tokenizer, diagnostics));
}

fn try_ident_path(
    tokenizer: &mut Tokenizer,
    diagnostics: &mut Diagnostics,
) -> Result<Option<Spanned<Box<[Spanned<Option<PseudoKeyword>>]>>>, FatalParsingError> {
    return try_one_or_more(
        tokenizer,
        diagnostics,
        try_infallible(try_ident),
        expect_ident,
        Symbol::DoubleColon,
    );
}
