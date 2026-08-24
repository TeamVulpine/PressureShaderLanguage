use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::{symbol::SymbolPath, try_keyword, try_number},
    source::Spanned,
    token::{Tokenizer, keyword::Keyword, number::NumberLiteral},
};

pub enum AtomExpr<'a> {
    Number(NumberLiteral),
    Boolean(bool),
    Symbol(SymbolPath<'a>),
}

impl<'a> AtomExpr<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        if let Some(path) = SymbolPath::try_parse(tokenizer, diagnostics, false)? {
            return Ok(Some(path.map(Self::Symbol)));
        }

        if let Some(number) = try_number(tokenizer, diagnostics) {
            return Ok(Some(number.map(Self::Number)));
        }

        if let Some(span) = try_keyword(tokenizer, diagnostics, Keyword::True) {
            return Ok(Some(span.into_spanned(Self::Boolean(true))));
        }

        if let Some(span) = try_keyword(tokenizer, diagnostics, Keyword::False) {
            return Ok(Some(span.into_spanned(Self::Boolean(false))));
        }

        return Ok(None);
    }
}
