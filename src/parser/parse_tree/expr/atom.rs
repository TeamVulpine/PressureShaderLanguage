use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::{TupleResult, expr::Expr, symbol::SymbolPath, try_keyword, try_number, try_tuple},
    source::Spanned,
    token::{Tokenizer, keyword::Keyword, number::NumberLiteral, symbol::Symbol},
};

#[derive(Debug)]
pub enum AtomExpr<'a> {
    Number(NumberLiteral),
    Boolean(bool),
    Symbol(SymbolPath<'a>),
    Tuple(Box<[Spanned<'a, Expr<'a>>]>),
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

        if let Some(result) = try_tuple(
            tokenizer,
            diagnostics,
            Expr::expect_parse,
            Symbol::ParenOpen,
            Symbol::ParenClose,
            Symbol::Comma,
        )? {
            match result {
                TupleResult::Parenthesized(expr) => return Ok(Some(expr)),
                TupleResult::Tuple(exprs) => {
                    return Ok(Some(exprs.map(Self::Tuple).map(Expr::Atom)));
                }
            }
        }

        return Ok(None);
    }
}
