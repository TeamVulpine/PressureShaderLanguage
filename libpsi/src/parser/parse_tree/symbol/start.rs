use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{symbol::ty::TypePart, try_ident, try_keyword},
        token::{Tokenizer, ident::PseudoKeyword, keyword::Keyword},
    },
    source::Spanned,
};

#[derive(Debug)]
pub enum SymbolPathStart {
    Ident(Option<PseudoKeyword>),
    Type(TypePart),
    SelfType,
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

        if let Some(self_ty) = try_keyword(tokenizer, diagnostics, Keyword::SelfType) {
            return Ok(Some(self_ty.into_spanned(Self::SelfType)));
        }

        return Ok(None);
    }
}
