use crate::parser::{
    diagnostic::{Diagnostic, DiagnosticKind, DiagnosticLevel, Diagnostics},
    source::SourceSpan,
    token::{TokenKind, Tokenizer, ident::PseudoKeyword, keyword::Keyword, symbol::Symbol},
};

pub mod expr;

fn push_error<'a>(
    diagnostics: &mut Diagnostics<'a>,
    kind: DiagnosticKind<'a>,
    span: SourceSpan<'a>,
) {
    diagnostics.push(Diagnostic {
        kind,
        level: DiagnosticLevel::Error,
        span,
    });
}

pub fn expect_symbol<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    symbol: Symbol,
    consume_on_mismatch: bool,
) -> SourceSpan<'a> {
    let token = match tokenizer.peek() {
        Ok(token) => token,
        Err(err) => {
            push_error(
                diagnostics,
                DiagnosticKind::ExpectedSymbol {
                    symbol,
                    got: Box::new(err.kind.into()),
                },
                err.span,
            );

            return err.span;
        }
    };

    if consume_on_mismatch {
        _ = tokenizer.next();
    }

    if token.kind == TokenKind::Symbol(symbol) {
        if !consume_on_mismatch {
            _ = tokenizer.next();
        }

        return token.span;
    }

    push_error(
        diagnostics,
        DiagnosticKind::ExpectedSymbol {
            symbol,
            got: Box::new(DiagnosticKind::from_token(token)),
        },
        token.span,
    );

    return token.span;
}

pub fn expect_keyword<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    keyword: Keyword,
    consume_on_mismatch: bool,
) -> SourceSpan<'a> {
    let token = match tokenizer.peek() {
        Ok(token) => token,
        Err(err) => {
            push_error(
                diagnostics,
                DiagnosticKind::ExpectedKeyword {
                    keyword,
                    got: Box::new(err.kind.into()),
                },
                err.span,
            );

            return err.span;
        }
    };

    if consume_on_mismatch {
        _ = tokenizer.next();
    }

    if token.kind == TokenKind::Keyword(keyword) {
        if !consume_on_mismatch {
            _ = tokenizer.next();
        }

        return token.span;
    }

    push_error(
        diagnostics,
        DiagnosticKind::ExpectedKeyword {
            keyword,
            got: Box::new(DiagnosticKind::from_token(token)),
        },
        token.span,
    );

    return token.span;
}

pub fn expect_pseudo_keyword<'a>(
    tokenizer: &mut Tokenizer<'a>,
    diagnostics: &mut Diagnostics<'a>,
    keyword: PseudoKeyword,
    consume_on_mismatch: bool,
) -> SourceSpan<'a> {
    let token = match tokenizer.peek() {
        Ok(token) => token,
        Err(err) => {
            push_error(
                diagnostics,
                DiagnosticKind::ExpectedPseudoKeyword {
                    keyword,
                    got: Box::new(err.kind.into()),
                },
                err.span,
            );

            return err.span;
        }
    };

    if consume_on_mismatch {
        _ = tokenizer.next();
    }

    if token.kind == TokenKind::Identifier(Some(keyword)) {
        if !consume_on_mismatch {
            _ = tokenizer.next();
        }

        return token.span;
    }

    push_error(
        diagnostics,
        DiagnosticKind::ExpectedPseudoKeyword {
            keyword,
            got: Box::new(DiagnosticKind::from_token(token)),
        },
        token.span,
    );

    return token.span;
}
