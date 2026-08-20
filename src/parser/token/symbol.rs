macro_rules! symbols {
    (
        $(
            $(#[$enum_meta:meta])*
            $vis:vis enum $t:ident {
                $($name:ident = $str:literal),* $(,)?
            }
        )+
    ) => {
        $(
            $(#[$enum_meta])*
            $vis enum $t {
                $($name),*
            }

            impl $t {
                pub fn parse(cursor: &mut $crate::parser::source::SourceCursor) -> Option<Self> {
                    $(
                        if cursor.consume_str($str) {
                            return Some(Self::$name);
                        }
                    )*
                    return None;
                }

                pub fn repr(&self) -> &'static str {
                    return match self {
                        $(Self::$name => $str),*
                    };
                }
            }
        )+
    };
}

symbols! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Symbol {
        // Operators
        RangeTo = "..=",
        Range = "..",

        ShiftLeftAssign = "<<=",
        ShiftRightAssign = ">>=",

        AddAssign = "+=",
        SubtractAssign = "-=",
        MultiplyAssign = "*=",
        DivideAssign = "/=",
        RemainderAssign = "%=",

        Equal = "==",
        NotEqual = "!=",
        LessThanOrEqual = "<=",
        GreaterThanOrEqual = ">=",

        BooleanAnd = "&&",
        BooleanOr = "||",

        BitwiseAndAssign = "&=",
        BitwiseOrAssign = "|=",
        BitwiseXorAssign = "^=",

        ShiftLeft = "<<",
        ShiftRight = ">>",

        Add = "+",
        Subtract = "-",
        Multiply = "*",
        Divide = "/",
        Remainder = "%",

        LessThan = "<",
        GreaterThan = ">",

        BitwiseAnd = "&",
        BitwiseOr = "|",
        BitwiseXor = "^",

        Not = "!",
        BitwiseNot = "~",

        // Access
        DoubleColon = "::",
        Dot = ".",

        // Delimiters
        ParenOpen = "(",
        ParenClose = ")",
        BracketOpen = "[",
        BracketClose = "]",
        BraceOpen = "{",
        BraceClose = "}",

        Comma = ",",
        Semicolon = ";",
        Colon = ":",
        Assign = "=",
    }
}
