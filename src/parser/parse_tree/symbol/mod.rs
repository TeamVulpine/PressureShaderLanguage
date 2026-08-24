use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::{MismatchHandling, expect_ident, try_ident, try_symbol},
    source::Spanned,
    token::{TokenKind, Tokenizer, ident::PseudoKeyword, symbol::Symbol},
};

#[derive(Debug)]
pub enum SymbolPathPart {
    Ident(Option<PseudoKeyword>),
}

#[derive(Debug)]
pub struct SymbolPath<'a> {
    pub values: Box<[Spanned<'a, SymbolPathPart>]>,
}

impl<'a> SymbolPath<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
        lenient_generic_separation: bool,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        let mut values = vec![];

        let Some(first) = try_ident(tokenizer, diagnostics) else {
            return Ok(None);
        };

        values.push(first.map(SymbolPathPart::Ident));

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

            values.push(ident.map(SymbolPathPart::Ident));
        }

        let span = values.first().unwrap().span + values.last().unwrap().span;

        return Ok(Some(span.into_spanned(Self {
            values: values.into(),
        })));
    }
}
