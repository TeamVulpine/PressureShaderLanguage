use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, TupleResult, expect_parse, symbol::SymbolPath, try_tuple,
            ty::slice::SliceTy,
        },
        token::{Tokenizer, symbol::Symbol},
    },
    source::Spanned,
};

pub mod slice;

#[derive(Debug)]
pub enum Ty {
    Symbol(SymbolPath),
    Slice(SliceTy),
    Tuple(Box<[Spanned<Ty>]>),
    Error,
}

impl Ty {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
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
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
        mismatch_handling: MismatchHandling,
    ) -> Result<Spanned<Self>, FatalParsingError> {
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
