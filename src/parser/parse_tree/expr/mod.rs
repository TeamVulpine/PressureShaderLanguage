use crate::parser::{
    diagnostic::{Diagnostics, FatalParsingError},
    parse_tree::expr::{atom::AtomExpr, operator::OperationExpr},
    source::Spanned,
    token::Tokenizer,
};

pub mod atom;
pub mod operator;

pub enum Expr<'a> {
    Operation(OperationExpr<'a>),
    Atom(AtomExpr<'a>),
}

impl<'a> Expr<'a> {
    pub fn try_parse(
        tokenizer: &mut Tokenizer<'a>,
        diagnostics: &mut Diagnostics<'a>,
    ) -> Result<Option<Spanned<'a, Self>>, FatalParsingError> {
        let Some(atom) = AtomExpr::try_parse(tokenizer, diagnostics)? else {
            return Ok(None);
        };

        // for now we just return the atom. I still need to implement the pratt parser.
        return Ok(Some(atom.map(Expr::Atom)));
    }
}
