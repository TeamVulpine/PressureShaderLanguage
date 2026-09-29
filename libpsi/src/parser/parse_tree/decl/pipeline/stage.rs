use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{MismatchHandling, expect_parse, try_infallible, try_pseudo_keyword},
        token::{Tokenizer, ident::PseudoKeyword},
    },
    source::Spanned,
};

#[derive(Debug)]
pub enum PipelineKind {
    Raster,
    Error,
}

impl PipelineKind {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Option<Spanned<Self>> {
        if let Some(span) = try_pseudo_keyword(tokenizer, diagnostics, PseudoKeyword::Raster) {
            return Some(span.into_spanned(Self::Raster));
        }

        return None;
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
            try_infallible(Self::try_parse),
            |diagnostic| DiagnosticKind::ExpectedPipelineKind {
                got: Box::new(diagnostic),
            },
            || Self::Error,
        );
    }
}
