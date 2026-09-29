use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, expect_ident, expect_one_or_more, symbol::SymbolPath, try_keyword,
            try_symbol,
        },
        token::{Tokenizer, ident::PseudoKeyword, keyword::Keyword, symbol::Symbol},
    },
    source::Spanned,
};

#[derive(Debug)]
pub struct WhereTy {
    pub name: Spanned<Option<PseudoKeyword>>,
    pub bounds: Option<Spanned<Box<[Spanned<SymbolPath>]>>>,
}

impl WhereTy {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let Some(start) = try_keyword(tokenizer, diagnostics, Keyword::Type) else {
            return Ok(None);
        };

        let name = expect_ident(tokenizer, diagnostics, MismatchHandling::Consume)?;

        let bounds = if try_symbol(tokenizer, diagnostics, Symbol::Colon).is_some() {
            Some(expect_one_or_more(
                tokenizer,
                diagnostics,
                SymbolPath::expect_parse_with(true),
                Symbol::Add,
            )?)
        } else {
            None
        };

        let end = bounds.as_ref().map(|it| it.span).unwrap_or(name.span);

        return Ok(Some((start + end).into_spanned(Self { name, bounds })));
    }
}
