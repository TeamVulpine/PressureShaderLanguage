use crate::{
    diagnostic::{Diagnostics, FatalParsingError},
    parser::{
        parse_tree::{
            TupleResult, expr::Expr, symbol::SymbolPath, try_keyword, try_number, try_tuple,
        },
        token::{Tokenizer, keyword::Keyword, number::NumberLiteral, symbol::Symbol},
    },
    source::Spanned,
};

#[derive(Debug)]
pub enum AtomExpr {
    Number(NumberLiteral),
    Boolean(bool),
    SelfValue,
    Unit,
    Symbol(SymbolPath),
    Tuple(Box<[Spanned<Expr>]>),
}

impl AtomExpr {
    pub fn try_parse(
        tokenizer: &mut Tokenizer,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Spanned<Expr>>, FatalParsingError> {
        if let Some(path) = SymbolPath::try_parse(tokenizer, diagnostics, false)? {
            return Ok(Some(path.map(Self::Symbol).map(Expr::Atom)));
        }

        if let Some(number) = try_number(tokenizer, diagnostics) {
            return Ok(Some(number.map(Self::Number).map(Expr::Atom)));
        }

        if let Some(span) = try_keyword(tokenizer, diagnostics, Keyword::True) {
            return Ok(Some(span.into_spanned(Expr::Atom(Self::Boolean(true)))));
        }

        if let Some(span) = try_keyword(tokenizer, diagnostics, Keyword::False) {
            return Ok(Some(span.into_spanned(Expr::Atom(Self::Boolean(false)))));
        }

        if let Some(span) = try_keyword(tokenizer, diagnostics, Keyword::SelfValue) {
            return Ok(Some(span.into_spanned(Expr::Atom(Self::SelfValue))));
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
                TupleResult::Unit(span) => {
                    return Ok(Some(span.into_spanned(Expr::Atom(AtomExpr::Unit))));
                }
            }
        }

        return Ok(None);
    }
}
