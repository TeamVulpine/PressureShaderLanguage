use crate::{
    diagnostic::{DiagnosticKind, Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            MismatchHandling, decl::ty::structure::field::StructureField, expect_parse,
            expect_symbol, try_symbol,
        },
        token::{Tokenizer, symbol::Symbol},
    },
    source::Spanned,
};

pub struct StructureBody {
    pub fields: Option<Spanned<Box<[Spanned<StructureField>]>>>,
}

impl StructureBody {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Self>>, FatalParsingError> {
        let Some(start) = try_symbol(tokenizer, diagnostics, Symbol::BracketOpen) else {
            return Ok(None);
        };

        let mut values: Vec<Spanned<StructureField>> = vec![];

        loop {
            if let Some(end) = try_symbol(tokenizer, diagnostics, Symbol::BracketClose) {
                let values = values
                    .first()
                    .zip(values.last())
                    .map(|(a, b)| a.span + b.span)
                    .map(|it| it.into_spanned(values.into_boxed_slice()));

                return Ok(Some((start + end).into_spanned(Self { fields: values })));
            }

            values.push(StructureField::expect_parse(
                tokenizer,
                diagnostics,
                MismatchHandling::ConsumeUntilSafe,
            )?);

            if try_symbol(tokenizer, diagnostics, Symbol::Comma).is_some() {
                continue;
            }

            let end = expect_symbol(
                tokenizer,
                diagnostics,
                Symbol::BracketClose,
                MismatchHandling::Consume,
            )?;

            let values = values
                .first()
                .zip(values.last())
                .map(|(a, b)| a.span + b.span)
                .map(|it| it.into_spanned(values.into_boxed_slice()));

            return Ok(Some((start + end).into_spanned(Self { fields: values })));
        }
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
            |got| DiagnosticKind::ExpectedStructBody { got: Box::new(got) },
            || Self { fields: None },
        );
    }
}
