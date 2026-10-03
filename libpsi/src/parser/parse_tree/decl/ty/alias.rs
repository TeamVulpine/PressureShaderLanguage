use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{MismatchHandling, expect_ident, expect_symbol, try_keyword, ty::Ty},
        token::{Tokenizer, ident::PseudoKeyword, keyword::Keyword, symbol::Symbol},
    },
    source::Spanned,
};

pub struct TypeAliasDecl {
    pub name: Spanned<Option<PseudoKeyword>>,
    pub ty: Spanned<Ty>,
}

impl TypeAliasDecl {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let Some(start) = try_keyword(tokenizer, diagnostics, Keyword::Type) else {
            return Ok(None);
        };

        let name = expect_ident(tokenizer, diagnostics, MismatchHandling::Consume)?;

        expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::Assign,
            MismatchHandling::Consume,
        )?;

        let ty = Ty::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        let end = expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::Semicolon,
            MismatchHandling::Consume,
        )?;

        return Ok(Some((start + end).into_spanned(Self { name, ty })));
    }
}
