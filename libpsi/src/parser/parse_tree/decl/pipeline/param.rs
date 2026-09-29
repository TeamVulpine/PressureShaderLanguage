use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{MismatchHandling, expect_symbol, expr::Expr, try_keyword, ty::Ty},
        token::{Tokenizer, keyword::Keyword, symbol::Symbol},
    },
    source::Spanned,
};

#[derive(Debug)]
pub struct PipelineParamConfig {
    pub ty: Spanned<Ty>,
    pub slot: Spanned<Expr>,
}

impl PipelineParamConfig {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let Some(start) = try_keyword(tokenizer, diagnostics, Keyword::Param) else {
            return Ok(None);
        };

        let ty = Ty::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::Colon,
            MismatchHandling::Consume,
        )?;

        let slot = Expr::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        let end = expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::Semicolon,
            MismatchHandling::Consume,
        )?;

        return Ok(Some((start + end).into_spanned(Self { ty, slot })));
    }
}
