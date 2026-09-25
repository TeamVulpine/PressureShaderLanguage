use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::{
        MismatchHandling, expect_symbol, expr::Expr, symbol::SymbolPath, try_keyword, try_number,
        try_symbol,
    },
    source::Spanned,
    token::{Tokenizer, keyword::Keyword, number::NumberLiteral, symbol::Symbol},
};

#[derive(Debug)]
pub enum AtomExpr<'a> {
    Number(NumberLiteral),
    Boolean(bool),
    Symbol(SymbolPath<'a>),
}

impl<'a> AtomExpr<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Expr<'a>>>, FatalParsingError> {
        if let Some(path) = SymbolPath::try_parse(tokenizer, diagnostics, false)? {
            return Ok(Some(path.map(Self::Symbol).map(Expr::Atom)));
        }

        if let Some(number) = try_number(tokenizer, diagnostics) {
            return Ok(Some(number.map(Self::Number).map(Expr::Atom)));
        }

        if let Some(span) = try_keyword(tokenizer, diagnostics, Keyword::True) {
            return Ok(Some(span.into_spanned(Self::Boolean(true)).map(Expr::Atom)));
        }

        if let Some(span) = try_keyword(tokenizer, diagnostics, Keyword::False) {
            return Ok(Some(
                span.into_spanned(Self::Boolean(false)).map(Expr::Atom),
            ));
        }

        if let Some(start) = try_symbol(tokenizer, diagnostics, Symbol::ParenOpen) {
            let expr =
                Expr::expect_parse(tokenizer, diagnostics, MismatchHandling::ConsumeUntilSafe)?;

            let end = expect_symbol(
                tokenizer,
                diagnostics,
                Symbol::ParenClose,
                MismatchHandling::Consume,
            )?;

            return Ok(Some((start + end).into_spanned(expr.value)));
        }

        return Ok(None);
    }
}
