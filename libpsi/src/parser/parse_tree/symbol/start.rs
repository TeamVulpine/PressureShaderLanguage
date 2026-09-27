use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{symbol::ty::TypePart, try_ident},
        token::{Tokenizer, ident::PseudoKeyword},
    },
    source::Spanned,
};

#[derive(Debug)]
pub enum SymbolPathStart {
    Ident(Option<PseudoKeyword>),
    Type(TypePart),
}

impl SymbolPathStart {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        if let Some(ident) = try_ident(tokenizer, diagnostics) {
            return Ok(Some(ident.map(Self::Ident)));
        }

        if let Some(type_as) = TypePart::try_parse(tokenizer, diagnostics)? {
            return Ok(Some(type_as.map(Self::Type)));
        }

        return Ok(None);
    }
}
