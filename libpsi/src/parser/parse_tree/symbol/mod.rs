use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, expect_parse,
            symbol::{part::SymbolPathPart, start::SymbolPathStart},
        },
        token::Tokenizer,
    },
    source::Spanned,
};

pub mod generic;
pub mod part;
pub mod start;
pub mod ty;

#[derive(Debug)]
pub enum SymbolPath {
    Parsed {
        first: Box<Spanned<SymbolPathStart>>,
        // parts: Box<[Spanned<SymbolPathPart>]>,
    },
    Error,
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

        // let parts = SymbolPathPart::try_many(tokenizer, diagnostics, lenient_generic_separation)?;

        // let (parts_span, parts) = if let Some(parts) = parts {
        //     (parts.span, parts.value)
        // } else {
        //     (first.span, Default::default())
        // };

        // let span = first.span + parts_span;

        let span = first.span;

        return Ok(Some(span.into_spanned(Self::Parsed {
            first: Box::new(first),
            // parts,
        })));
    }

    pub fn expect_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
        mismatch_handling: MismatchHandling,
        lenient_generic_separation: bool,
    ) -> Result<Spanned<Self>, FatalParsingError> {
        return expect_parse(
            tokenizer,
            diagnostics,
            mismatch_handling,
            |tokenizer, diagnostics| {
                Self::try_parse(tokenizer, diagnostics, lenient_generic_separation)
            },
            |diagnostic| DiagnosticKind::ExpectedSymbolPath {
                got: Box::new(diagnostic),
            },
            || Self::Error,
        );
    }

    pub fn expect_parse_with(
        lenient_generic_separation: bool,
    ) -> impl Fn(
        &mut Tokenizer,
        &mut Diagnostics,
        MismatchHandling,
    ) -> Result<Spanned<Self>, FatalParsingError> {
        return move |tokenizer, diagnostics, mismatch_handling| {
            Self::expect_parse(
                tokenizer,
                diagnostics,
                mismatch_handling,
                lenient_generic_separation,
            )
        };
    }
}
