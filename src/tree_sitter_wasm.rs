use web_tree_sitter_sys::{SyntaxNode as Node, Parser, Language};
pub use web_tree_sitter_sys as tree_sitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tree-sitter-cpp")]
extern "C" { fn tree_sitter_cpp() -> Language; }

impl Parser {
    pub fn parse(&mut self, text: impl AsRef<[u8]>, old_tree: Option<&Tree>) -> Option<Tree> {
        let bytes = text.as_ref();
        let str = 
        let len = bytes.len();
        self.parse_with_string(
            &mut |i, _| if i < len { &bytes[i..] } else { &[] },
            old_tree,
            None
        )
    }
}

pub fn get_treesitter() -> Parser {
    let mut parser = Parser::new();
    let language = tree_sitter_cpp();
    parser.set_language(language).unwrap();
    return parser;
}