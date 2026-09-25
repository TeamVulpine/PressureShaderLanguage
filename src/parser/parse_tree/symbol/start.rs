use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::{symbol::r#type::TypePart, try_ident},
    source::Spanned,
    token::{Tokenizer, ident::PseudoKeyword},
};

#[derive(Debug)]
pub enum SymbolPathStart<'a> {
    Ident(Option<PseudoKeyword>),
    Type(TypePart<'a>),
}

impl<'a> SymbolPathStart<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        if let Some(ident) = try_ident(tokenizer, diagnostics) {
            return Ok(Some(ident.map(Self::Ident)));
        }

        if let Some(type_as) = TypePart::try_parse(tokenizer, diagnostics)? {
            return Ok(Some(type_as.map(Self::Type)));
        }

        return Ok(None);
    }
}
