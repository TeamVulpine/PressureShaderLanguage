use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, expect_parse, try_ident_path, try_list, try_number, try_string,
        },
        token::{Tokenizer, ident::PseudoKeyword, number::NumberLiteral, symbol::Symbol},
    },
    source::Spanned,
};

pub enum AttributeArgument {
    String,
    IdentPath(Box<[Spanned<Option<PseudoKeyword>>]>),
    Number(NumberLiteral),
    Error,
}

impl AttributeArgument {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        if let Some(span) = try_string(tokenizer, diagnostics) {
            return Ok(Some(span.into_spanned(Self::String)));
        }

        if let Some(idents) = try_ident_path(tokenizer, diagnostics)? {
            return Ok(Some(idents.map(Self::IdentPath)));
        }

        if let Some(number) = try_number(tokenizer, diagnostics) {
            return Ok(Some(number.map(Self::Number)));
        }

        return Ok(None);
    }

    pub fn expect_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
        mismatch_handling: MismatchHandling,
    ) -> Result<Spanned<Self>, FatalParsingError> {
        return expect_parse(
            tokenizer,
            diagnostics,
            mismatch_handling,
            Self::try_parse,
            |got| DiagnosticKind::ExpectedAttributeParameter { got: Box::new(got) },
            || Self::Error,
        );
    }

    pub fn try_list(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Box<[Spanned<Self>]>>>, FatalParsingError> {
        return try_list(
            tokenizer,
            diagnostics,
            Self::expect_parse,
            &[Symbol::ParenOpen],
            Symbol::ParenClose,
            Symbol::Comma,
            true,
        );
    }
}
