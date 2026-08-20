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

        // Values
        True = "true",
        False = "false",
        SelfValue = "self",

        // Types
        SelfType = "Self",
        Bool = "bool",

        U8 = "u8",
        U16 = "u16",
        U32 = "u32",
        U64 = "u64",

        I8 = "i8",
        I16 = "i16",
        I32 = "i32",
        I64 = "i64",

        F16 = "f16",
        F32 = "f32",
        F64 = "f64",
    }
}
