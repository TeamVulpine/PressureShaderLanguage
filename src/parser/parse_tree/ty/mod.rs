use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parse_tree::{
        MismatchHandling, expect_parse,
        symbol::SymbolPath,
        ty::{reference::ReferenceTy, slice::SliceTy},
    },
    source::Spanned,
    token::Tokenizer,
};

pub mod reference;
pub mod slice;

#[derive(Debug)]
pub enum Ty<'a> {
    Symbol(SymbolPath<'a>),
    Slice(SliceTy<'a>),
    Reference(ReferenceTy<'a>),
    Error,
}

impl<'a> Ty<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        if let Some(symbol) = SymbolPath::try_parse(tokenizer, diagnostics, true)? {
            return Ok(Some(symbol.map(Self::Symbol)));
        }

        if let Some(slice) = SliceTy::try_parse(tokenizer, diagnostics)? {
            return Ok(Some(slice.map(Self::Slice)));
        }

        if let Some(reference) = ReferenceTy::try_parse(tokenizer, diagnostics)? {
            return Ok(Some(reference.map(Self::Reference)));
        }

        return Ok(None);
    }

    pub fn expect_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
        mismatch_handling: MismatchHandling,
    ) -> Result<Spanned<'a, Self>, FatalParsingError> {
        return expect_parse(
            tokenizer,
            diagnostics,
            mismatch_handling,
            Self::try_parse,
            |diagnostic| DiagnosticKind::ExpectedTy {
                got: Box::new(diagnostic),
            },
            || Self::Error,
        );
    }
}
