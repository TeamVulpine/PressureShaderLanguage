use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::{
        symbol::{part::SymbolPathPart, start::SymbolPathStart, r#type::TypePart},
        try_ident,
    },
    source::Spanned,
    token::Tokenizer,
};

pub mod generic;
pub mod part;
pub mod start;
pub mod r#type;

#[derive(Debug)]
pub struct SymbolPath<'a> {
    pub first: Box<Spanned<'a, SymbolPathStart<'a>>>,
    pub parts: Box<[Spanned<'a, SymbolPathPart<'a>>]>,
}

impl<'a> SymbolPath<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
        lenient_generic_separation: bool,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        let Some(first) = SymbolPathStart::try_parse(tokenizer, diagnostics)? else {
            return Ok(None);
        };

        let parts = SymbolPathPart::try_many(tokenizer, diagnostics, lenient_generic_separation)?;

        let (parts_span, parts) = if let Some(parts) = parts {
            (parts.span, parts.value)
        } else {
            (first.span, Default::default())
        };

        let span = first.span + parts_span;

        return Ok(Some(span.into_spanned(Self {
            first: Box::new(first),
            parts,
        })));
    }
}
