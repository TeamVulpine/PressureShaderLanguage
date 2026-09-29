use crate::parser::token::keywords;

keywords! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Keyword {
        // Modules
        Mod = "mod",
        Pub = "pub",

        // Declarators
        Fn = "fn",
        Type = "type",
        Struct = "struct",
        Enum = "enum",
        Let = "let",
        Const = "const",
        Trait = "trait",
        Impl = "impl",
        Pipeline = "pipeline",
        Param = "param",
        Bitfield = "bitfield",

        // Modifiers
        Mut = "mut",
        As = "as",
        In = "in",
        Out = "out",

        // Values / Types
        True = "true",
        False = "false",
        SelfValue = "self",
        SelfType = "Self",
        DiscardValue = "_",

        // Control Flow
        If = "if",
        Else = "else",
        For = "for",
        Loop = "loop",
        While = "while",
        Match = "match",
        Return = "return",
        Break = "break",
        Continue = "continue",
    }
}
