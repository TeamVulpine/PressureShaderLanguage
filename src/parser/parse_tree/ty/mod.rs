use crate::parser::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parse_tree::{
        MismatchHandling, TupleResult, expect_parse, symbol::SymbolPath, try_tuple,
        ty::slice::SliceTy,
    },
    source::Spanned,
    token::{Tokenizer, symbol::Symbol},
};

pub mod slice;

#[derive(Debug)]
pub enum Ty<'a> {
    Symbol(SymbolPath<'a>),
    Slice(SliceTy<'a>),
    Tuple(Box<[Spanned<'a, Ty<'a>>]>),
    Error,
}

impl<'a> Ty<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        if let Some(result) = try_tuple(
            tokenizer,
            diagnostics,
            Self::expect_parse,
            Symbol::ParenOpen,
            Symbol::ParenClose,
            Symbol::Comma,
        )? {
            match result {
                TupleResult::Parenthesized(ty) => return Ok(Some(ty)),
                TupleResult::Tuple(tys) => {
                    return Ok(Some(tys.map(Self::Tuple)));
                }
            }
        }

        if let Some(symbol) = SymbolPath::try_parse(tokenizer, diagnostics, true)? {
            return Ok(Some(symbol.map(Self::Symbol)));
        }

        if let Some(slice) = SliceTy::try_parse(tokenizer, diagnostics)? {
            return Ok(Some(slice.map(Self::Slice)));
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
