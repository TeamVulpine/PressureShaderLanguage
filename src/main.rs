use pressure_shader_language::parser::{
    diagnostic::Diagnostics,
    parse_tree::{
        MismatchHandling, expect_keyword, expect_pseudo_keyword, expect_symbol, path::Path,
    },
    token::{Tokenizer, ident::PseudoKeyword, keyword::Keyword, symbol::Symbol},
};

fn main() {
    let contents = std::fs::read_to_string("test.psi").unwrap();

    let mut tokenizer = Tokenizer::new(&contents, Some("test.psi"));
    let mut diagnostics = Diagnostics::new();

    let path = Path::try_parse(&mut tokenizer, &mut diagnostics).unwrap();

    println!("{:?}", path);

    for diagnostic in diagnostics.into_iter() {
        println!("{}", diagnostic);
    }
}
