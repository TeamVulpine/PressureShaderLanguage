use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling,
            decl::where_clause::{constant::WhereConstant, ty::WhereTy},
            expect_parse, try_list_pseudo_keyword,
        },
        token::{Tokenizer, ident::PseudoKeyword, symbol::Symbol},
    },
    source::Spanned,
};

pub mod constant;
pub mod ty;

#[derive(Debug)]
pub enum WhereParam {
    Constant(WhereConstant),
    Ty(WhereTy),
    Error,
}

impl WhereParam {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        if let Some(decl) = WhereTy::try_parse(tokenizer, diagnostics)? {
            return Ok(Some(decl.map(Self::Ty)));
        }

        if let Some(decl) = WhereConstant::try_parse(tokenizer, diagnostics)? {
            return Ok(Some(decl.map(Self::Constant)));
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
            |diagnostic| DiagnosticKind::ExpectedGenericParam {
                got: Box::new(diagnostic),
            },
            || Self::Error,
        );
    }

    pub fn try_list(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Box<[Spanned<Self>]>>>, FatalParsingError> {
        return try_list_pseudo_keyword(
            tokenizer,
            diagnostics,
            Self::expect_parse,
            PseudoKeyword::Where,
            Symbol::Semicolon,
            Symbol::Comma,
            true,
        );
    }
}
