use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, decl::ty::structure::body::StructureBody, expect_ident, try_keyword,
        },
        token::{Tokenizer, ident::PseudoKeyword, keyword::Keyword},
    },
    source::Spanned,
};

pub mod body;
pub mod field;

pub enum StructureKind {
    Struct,
    Param,
}

pub struct StructureDecl {
    pub kind: Spanned<StructureKind>,
    pub name: Spanned<Option<PseudoKeyword>>,
    pub body: Spanned<StructureBody>,
}

impl StructureKind {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Option<Spanned<Self>> {
        if let Some(span) = try_keyword(tokenizer, diagnostics, Keyword::Struct) {
            return Some(span.into_spanned(Self::Struct));
        }

        if let Some(span) = try_keyword(tokenizer, diagnostics, Keyword::Param) {
            return Some(span.into_spanned(Self::Param));
        }

        return None;
    }
}

impl StructureDecl {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let Some(kind) = StructureKind::try_parse(tokenizer, diagnostics) else {
            return Ok(None);
        };

        let name = expect_ident(tokenizer, diagnostics, MismatchHandling::Consume)?;

        let body = StructureBody::expect_parse(
            tokenizer,
            diagnostics,
            MismatchHandling::ConsumeUntilSafe,
        )?;

        let span = kind.span + body.span;

        return Ok(Some(span.into_spanned(Self { kind, name, body })));
    }
}
