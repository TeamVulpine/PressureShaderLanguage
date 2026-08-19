use pressure_shader_language::parser::{
    diagnostic::Diagnostics,
    parse_tree::{expect_keyword, expect_pseudo_keyword, expect_symbol},
    token::{TokenKind, Tokenizer, ident::PseudoKeyword, keyword::Keyword, symbol::Symbol},
};

fn main() {
    let contents = std::fs::read_to_string("test.psi").unwrap();

    let mut tokenizer = Tokenizer::new(&contents, Some("test.psi"));
    let mut diagnostics = Diagnostics::new();

    expect_symbol(&mut tokenizer, &mut diagnostics, Symbol::Plus, true);
    expect_keyword(&mut tokenizer, &mut diagnostics, Keyword::Let, true);
    expect_pseudo_keyword(
        &mut tokenizer,
        &mut diagnostics,
        PseudoKeyword::Discard,
        true,
    );

    for diagnostic in diagnostics.into_iter() {
        println!("{}", diagnostic);
    }
}
