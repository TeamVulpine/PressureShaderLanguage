use crate::parser::token::keywords;

keywords! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Keyword {
        // Declarators
        Fn = "fn",
        Struct = "struct",
        Enum = "enum",
        Let = "let",
        Static = "static",
        Mut = "mut",
        Const = "const",
    }
}
