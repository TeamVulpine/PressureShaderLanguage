use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, attr::Attribute, expect_ident, expect_parse, expect_symbol,
            try_ident, try_list, ty::Ty,
        },
        token::{Tokenizer, ident::PseudoKeyword, symbol::Symbol},
    },
    source::Spanned,
};

pub enum StructureField {
    Parsed {
        attrs: Option<Spanned<Box<[Spanned<Attribute>]>>>,
        name: Spanned<Option<PseudoKeyword>>,
        ty: Spanned<Ty>,
    },
    Error,
}

impl StructureField {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let attrs = Attribute::try_list(tokenizer, diagnostics)?;

        let name = if attrs.is_some() {
            expect_ident(tokenizer, diagnostics, MismatchHandling::Consume)?
        } else {
            let Some(name) = try_ident(tokenizer, diagnostics) else {
                return Ok(None);
            };

            name
        };

        let start = attrs.as_ref().map(|it| it.span).unwrap_or(name.span);

        expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::Colon,
            MismatchHandling::Consume,
        )?;

        let ty = Ty::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

        let end = expect_symbol(
            tokenizer,
            diagnostics,
            Symbol::Semicolon,
            MismatchHandling::Consume,
        )?;

        return Ok(Some((start + end).into_spanned(Self::Parsed {
            attrs,
            name,
            ty,
        })));
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
            |got| DiagnosticKind::ExpectedStructField { got: Box::new(got) },
            || Self::Error,
        );
    }
}
