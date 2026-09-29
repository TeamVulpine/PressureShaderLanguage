use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, expect_ident, expect_symbol, expr::Expr, try_keyword, ty::Ty,
        },
        token::{Tokenizer, ident::PseudoKeyword, keyword::Keyword, symbol::Symbol},
    },
    source::Spanned,
};

pub struct ConstantDecl {
    pub name: Spanned<Option<PseudoKeyword>>,
    pub ty: Spanned<Ty>,
    pub value: Spanned<Expr>,
}

impl ConstantDecl {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let Some(start) = try_keyword(tokenizer, diagnostics, Keyword::Const) else {
            return Ok(None);
        };

        let name = expect_ident(tokenizer, diagnostics, MismatchHandling::Skip)?;

        expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::Colon,
            MismatchHandling::Consume,
        )?;

        let ty = Ty::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::Assign,
            MismatchHandling::Consume,
        )?;

        let value = Expr::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        return Ok(Some((start + value.span).into_spanned(Self {
            name,
            ty,
            value,
        })));
    }
}
