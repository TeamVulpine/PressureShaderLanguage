use crate::parser::token::keywords;

keywords! {
    /// A pseudo-keyword is an identifier that can act as a keyword in the right context.
    /// Also known as contextual keywords
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum PseudoKeyword {
        Discard = "_",
        Where = "where",
        From = "from",

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
