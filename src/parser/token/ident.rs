use crate::parser::token::keywords;

keywords! {
    /// A pseudo-keyword is an identifier that can act as a keyword in the right context.
    /// Also known as contextual keywords
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum PseudoKeyword {
        Discard = "_",
    }
}
