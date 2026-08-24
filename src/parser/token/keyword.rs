use crate::parser::token::keywords;

keywords! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Keyword {
        // Modules
        Mod = "mod",
        Use = "use",

        // Declarators
        Fn = "fn",
        Type = "type",
        Struct = "struct",
        Enum = "enum",
        Let = "let",
        Static = "static",
        Mut = "mut",
        Const = "const",
        Trait = "trait",
        Impl = "impl",

        // Values
        True = "true",
        False = "false",
        SelfValue = "self",
    }
}
