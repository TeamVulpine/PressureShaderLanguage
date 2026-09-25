use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::{MismatchHandling, expect_symbol, try_keyword, ty::Ty},
    source::Spanned,
    token::{Tokenizer, keyword::Keyword, symbol::Symbol},
};

#[derive(Debug)]
pub struct TypePart<'a> {
    pub ty: Spanned<'a, Ty<'a>>,
    pub as_ty: Option<Spanned<'a, Ty<'a>>>,
}

impl<'a> TypePart<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        let Some(start) = try_keyword(tokenizer, diagnostics, Keyword::Type) else {
            return Ok(None);
        };

        expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::BracketOpen,
            MismatchHandling::Consume,
        )?;

        let ty = Ty::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        let as_ty = if try_keyword(tokenizer, diagnostics, Keyword::As).is_some() {
            Some(Ty::expect_parse(
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

        return Ok(Some((start + end).into_spanned(Self { ty, as_ty })));
    }
}
