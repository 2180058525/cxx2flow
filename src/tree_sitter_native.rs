pub use tree_sitter::{Node, Parser};
pub use tree_sitter;
pub fn get_treesitter() -> Parser {
    let mut parser = Parser::new();
    let language = tree_sitter_cpp::language();
    parser.set_language(language).unwrap();
    return parser;
}