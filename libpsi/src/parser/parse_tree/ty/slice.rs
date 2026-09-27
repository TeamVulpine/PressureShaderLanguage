use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{MismatchHandling, expect_symbol, expr::Expr, try_symbol, ty::Ty},
        token::{Tokenizer, symbol::Symbol},
    },
    source::Spanned,
};

#[derive(Debug)]
pub struct SliceTy {
    pub base: Box<Spanned<Ty>>,
    pub len: Option<Spanned<Expr>>,
}

impl SliceTy {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
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
