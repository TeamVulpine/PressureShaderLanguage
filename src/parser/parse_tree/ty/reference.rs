use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::{MismatchHandling, try_keyword, try_symbol, ty::Ty},
    source::{SourceSpan, Spanned},
    token::{Tokenizer, keyword::Keyword, symbol::Symbol},
};

#[derive(Debug)]
pub struct ReferenceTy<'a> {
    pub base: Box<Spanned<'a, Ty<'a>>>,
    pub mutable: Option<SourceSpan<'a>>,
}

impl<'a> ReferenceTy<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        let Some(start) = try_symbol(tokenizer, diagnostics, Symbol::BitwiseAnd) else {
            return Ok(None);
        };

        let mutable = try_keyword(tokenizer, diagnostics, Keyword::Mut);

        let ty = Ty::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        return Ok(Some((start + ty.span).into_spanned(Self {
            base: Box::new(ty),
            mutable,
        })));
    }
}
