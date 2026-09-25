use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parse_tree::{peek_token, symbol::generic::GenericArgument, try_ident, try_many, try_symbol},
    source::Spanned,
    token::{Tokenizer, ident::PseudoKeyword, symbol::Symbol},
};

#[derive(Debug)]
pub enum SymbolPathPart<'a> {
    Ident(Option<PseudoKeyword>),
    Generics(Box<[Spanned<'a, GenericArgument<'a>>]>),
    Error,
}

impl<'a> SymbolPathPart<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
        lenient_generic_separation: bool,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        if lenient_generic_separation
            && let Some(generics) = GenericArgument::try_list(tokenizer, diagnostics)?
        {
            return Ok(Some(generics.map(Self::Generics)));
        }

        let Some(start) = try_symbol(tokenizer, diagnostics, Symbol::DoubleColon) else {
            return Ok(None);
        };

        if let Some(generics) = GenericArgument::try_list(tokenizer, diagnostics)? {
            return Ok(Some(generics.map(Self::Generics)));
        }

        if let Some(ident) = try_ident(tokenizer, diagnostics) {
            return Ok(Some(ident.map(Self::Ident)));
        }

        let token = peek_token(tokenizer, diagnostics);

        diagnostics.push_error(
            DiagnosticKind::ExpectedSymbolPathPart {
                got: Box::new(DiagnosticKind::from_token(token)),
            },
            start + token.span,
        );

        return Ok(Some((start + token.span).into_spanned(Self::Error)));
    }

    pub fn try_many(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
        lenient_generic_separation: bool,
    ) -> Result<Option<Spanned<'a, Box<[Spanned<'a, Self>]>>>, FatalParsingError> {
        return try_many(tokenizer, diagnostics, |tokenizer, diagnostics| {
            Self::try_parse(tokenizer, diagnostics, lenient_generic_separation)
        });
    }
}
