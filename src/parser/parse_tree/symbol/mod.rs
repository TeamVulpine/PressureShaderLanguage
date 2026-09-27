use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::symbol::{part::SymbolPathPart, start::SymbolPathStart},
        token::Tokenizer,
    },
    source::Spanned,
};

pub mod generic;
pub mod part;
pub mod start;
pub mod ty;

#[derive(Debug)]
pub struct SymbolPath {
    pub first: Box<Spanned<SymbolPathStart>>,
    pub parts: Box<[Spanned<SymbolPathPart>]>,
}

impl SymbolPath {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
        lenient_generic_separation: bool,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
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
