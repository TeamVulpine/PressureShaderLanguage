use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, attr::argument::AttributeArgument, expect_parse, try_ident_path,
            try_list,
        },
        token::{Tokenizer, ident::PseudoKeyword, symbol::Symbol},
    },
    source::Spanned,
};

pub mod argument;

pub enum Attribute {
    Parsed {
        path: Spanned<Box<[Spanned<Option<PseudoKeyword>>]>>,
        arguments: Option<Spanned<Box<[Spanned<AttributeArgument>]>>>,
    },
    Error,
}

impl Attribute {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let Some(path) = try_ident_path(tokenizer, diagnostics)? else {
            return Ok(None);
        };

        let arguments = AttributeArgument::try_list(tokenizer, diagnostics)?;

        let end = if let Some(arguments) = &arguments {
            arguments.span
        } else {
            path.span
        };

        return Ok(Some(
            (path.span + end).into_spanned(Self::Parsed { path, arguments }),
        ));
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
            |got| DiagnosticKind::ExpectedAttribute { got: Box::new(got) },
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
            &[Symbol::Hash, Symbol::BracketOpen],
            Symbol::BracketClose,
            Symbol::Comma,
            true,
        );
    }

    pub fn try_list_file(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Box<[Spanned<Self>]>>>, FatalParsingError> {
        return try_list(
            tokenizer,
            diagnostics,
            Self::expect_parse,
            &[Symbol::Not, Symbol::BracketOpen],
            Symbol::BracketClose,
            Symbol::Comma,
            true,
        );
    }
}
