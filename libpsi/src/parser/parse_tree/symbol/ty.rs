use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{MismatchHandling, expect_symbol, try_keyword, ty::Ty},
        token::{Tokenizer, keyword::Keyword, symbol::Symbol},
    },
    source::Spanned,
};

#[derive(Debug)]
pub struct TypePart {
    pub ty: Spanned<Ty>,
    pub as_ty: Option<Spanned<Ty>>,
}

impl TypePart {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
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
