use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::{MismatchHandling, expect_ident, try_ident, try_symbol},
    source::Spanned,
    token::{TokenKind, Tokenizer, ident::PseudoKeyword, symbol::Symbol},
};

pub type Path<'a> = Spanned<'a, Box<[Spanned<'a, Option<PseudoKeyword>>]>>;

impl<'a> Path<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Self>, FatalParsingError> {
        let mut values = vec![];

        let Some(first) = try_ident(tokenizer, diagnostics) else {
            return Ok(None);
        };

        values.push(first);

        while let Some(_) = try_symbol(tokenizer, diagnostics, Symbol::DoubleColon) {
            let Some(ident) = expect_ident(
                tokenizer,
                diagnostics,
                MismatchHandling::SkipTo(&[
                    TokenKind::Symbol(Symbol::Semicolon),
                    TokenKind::Symbol(Symbol::BracketClose),
                ]),
            )?
            else {
                break;
            };

            values.push(ident);
        }

        let span = values.first().unwrap().span + values.last().unwrap().span;

        return Ok(Some(span.into_spanned(values.into())));
    }
}
