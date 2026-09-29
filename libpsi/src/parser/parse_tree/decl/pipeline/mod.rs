use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling,
            decl::pipeline::{
                entrypoint::PipelineEntrypoint, param::PipelineParamConfig, stage::PipelineKind,
            },
            expect_ident, expect_symbol, try_keyword,
        },
        token::{Tokenizer, ident::PseudoKeyword, keyword::Keyword, symbol::Symbol},
    },
    source::Spanned,
};

pub mod entrypoint;
pub mod param;
pub mod stage;

#[derive(Debug)]
pub struct PipelineBlock {
    pub name: Spanned<Option<PseudoKeyword>>,
    pub kind: Spanned<PipelineKind>,
    // Not spanned boxes because there is no guaranteed ordering of statement types.
    pub param_configs: Box<[Spanned<PipelineParamConfig>]>,
    pub entrypoints: Box<[Spanned<PipelineEntrypoint>]>,
}

impl PipelineBlock {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let Some(start) = try_keyword(tokenizer, diagnostics, Keyword::Pipeline) else {
            return Ok(None);
        };

        let name = expect_ident(tokenizer, diagnostics, MismatchHandling::Consume)?;

        expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::Colon,
            MismatchHandling::Consume,
        )?;

        let kind = PipelineKind::expect_parse(tokenizer, diagnostics, MismatchHandling::Consume)?;

        expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::BraceOpen,
            MismatchHandling::Consume,
        )?;

        let mut param_configs = vec![];
        let mut entrypoints = vec![];

        loop {
            if let Some(param_config) = PipelineParamConfig::try_parse(tokenizer, diagnostics)? {
                param_configs.push(param_config);
                continue;
            }

            if let Some(entrypoint) = PipelineEntrypoint::try_parse(tokenizer, diagnostics)? {
                entrypoints.push(entrypoint);
                continue;
            }

            let end = expect_symbol(
                tokenizer,
                diagnostics,
                Symbol::BraceClose,
                MismatchHandling::Consume,
            )?;

            return Ok(Some((start + end).into_spanned(Self {
                name,
                kind,
                param_configs: param_configs.into(),
                entrypoints: entrypoints.into(),
            })));
        }
    }
}
