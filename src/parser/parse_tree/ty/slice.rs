use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::{MismatchHandling, expect_symbol, expr::Expr, try_symbol, ty::Ty},
    source::Spanned,
    token::{Tokenizer, symbol::Symbol},
};

#[derive(Debug)]
pub struct SliceTy<'a> {
    pub base: Box<Spanned<'a, Ty<'a>>>,
    pub len: Option<Spanned<'a, Expr<'a>>>,
}

impl<'a> SliceTy<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        let Some(start) = try_symbol(tokenizer, diagnostics, Symbol::BracketOpen) else {
            return Ok(None);
        };

        let base = Ty::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        let len = if try_symbol(tokenizer, diagnostics, Symbol::Semicolon).is_some() {
            Some(Expr::expect_parse(
                tokenizer,
                diagnostics,
                MismatchHandling::ConsumeUntilSafe,
            )?)
        } else {
            None
        };

        let end = expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::BracketClose,
            MismatchHandling::Consume,
        )?;

        return Ok(Some((start + end).into_spanned(Self {
            base: Box::new(base),
            len,
        })));
    }
}
