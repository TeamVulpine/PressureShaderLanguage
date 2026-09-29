use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{MismatchHandling, expect_symbol, symbol::SymbolPath, try_pseudo_keyword},
        token::{Tokenizer, ident::PseudoKeyword, symbol::Symbol},
    },
    source::Spanned,
};

#[derive(Debug)]
pub enum PipelineEntrypointKind {
    Vertex,
    Fragment,
}

#[derive(Debug)]
pub struct PipelineEntrypoint {
    pub kind: Spanned<PipelineEntrypointKind>,
    pub symbol: Spanned<SymbolPath>,
}

impl PipelineEntrypointKind {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Option<Spanned<Self>> {
        if let Some(span) = try_pseudo_keyword(tokenizer, diagnostics, PseudoKeyword::Vertex) {
            return Some(span.into_spanned(Self::Vertex));
        }

        if let Some(span) = try_pseudo_keyword(tokenizer, diagnostics, PseudoKeyword::Fragment) {
            return Some(span.into_spanned(Self::Fragment));
        }

        return None;
    }
}

impl PipelineEntrypoint {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let Some(kind) = PipelineEntrypointKind::try_parse(tokenizer, diagnostics) else {
            return Ok(None);
        };

        let symbol = SymbolPath::expect_parse(
            tokenizer,
            diagnostics,
            MismatchHandling::ConsumeUntilSafe,
            true,
        )?;

        let end = expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::Semicolon,
            MismatchHandling::Consume,
        )?;

        return Ok(Some((kind.span + end).into_spanned(Self { kind, symbol })));
    }
}
