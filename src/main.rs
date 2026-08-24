use pressure_shader_language::parser::{
    diagnostic::Diagnostics, parse_tree::symbol::SymbolPath, token::Tokenizer,
};

fn main() {
    let contents = std::fs::read_to_string("test.psi").unwrap();

    let mut tokenizer = Tokenizer::new(&contents, Some("test.psi"));
    let mut diagnostics = Diagnostics::new();

    let path = SymbolPath::try_parse(&mut tokenizer, &mut diagnostics, false).unwrap();

    println!("{:?}", path);

    for diagnostic in diagnostics.into_iter() {
        println!("{}", diagnostic);
    }
}
