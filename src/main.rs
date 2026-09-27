use pawkit_interner::InternString;
use pressure_shader_language::{
    diagnostic::Diagnostics,
    library::builder::LibraryBuilder,
    module_cache::ModuleCache,
    parser::{parse_tree::expr::Expr, token::Tokenizer},
    report::IntoReport,
};

fn main() {
    let library = LibraryBuilder::new()
        .add_root_module(InternString::new("test.psi"))
        .build();

    let contents = InternString::from(std::fs::read_to_string("test.psi").unwrap());

    let mut cache = ModuleCache::new();

    let index = cache
        .insert(InternString::new("test.psi"), contents.clone())
        .unwrap();

    let mut tokenizer = Tokenizer::new(&contents, index);
    let mut diagnostics = Diagnostics::new();

    let path = Expr::try_parse(&mut tokenizer, &mut diagnostics).unwrap();

    println!("{:?}", path);

    for diagnostic in diagnostics.into_iter() {
        println!("{}", diagnostic.into_report().render(&cache));
    }
}
