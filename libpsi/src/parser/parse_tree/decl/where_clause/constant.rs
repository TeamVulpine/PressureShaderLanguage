use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{MismatchHandling, expect_ident, expect_symbol, try_keyword, ty::Ty},
        token::{Tokenizer, ident::PseudoKeyword, keyword::Keyword, symbol::Symbol},
    },
    source::Spanned,
};

#[derive(Debug)]
pub struct WhereConstant {
    pub name: Spanned<Option<PseudoKeyword>>,
    pub ty: Spanned<Ty>,
}

impl WhereConstant {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let Some(start) = try_keyword(tokenizer, diagnostics, Keyword::Const) else {
            return Ok(None);
        };

        let name = expect_ident(tokenizer, diagnostics, MismatchHandling::Consume)?;

        expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::Colon,
            MismatchHandling::Consume,
        )?;

        let ty = Ty::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        return Ok(Some((start + ty.span).into_spanned(Self { name, ty })));
    }
}
