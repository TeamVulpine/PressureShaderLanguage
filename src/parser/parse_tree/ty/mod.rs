use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::symbol::SymbolPath,
    source::Spanned,
    token::Tokenizer,
};

pub mod slice;

pub enum Ty<'a> {
    Symbol(SymbolPath<'a>),
}

impl<'a> Ty<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        if let Some(symbol) = SymbolPath::try_parse(tokenizer, diagnostics, true)? {
            return Ok(Some(symbol.map(Self::Symbol)));
        }

        return Ok(None);
    }
}
