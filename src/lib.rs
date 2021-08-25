mod ast;
mod dot;
mod graph;
mod parser;
mod language;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn generate(code: String, func: String, curved: bool) -> String {
    use web_tree_sitter_sys;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    JsFuture::from(web_tree_sitter_sys::Parser::init()).await;
    let res = parser::parse(code.as_bytes(), Some(String::from(func))).await;
    if let Err(res) = res {
        return res.to_string();
    }
    let (ast_vec, maxid) = res.unwrap();
    let res = graph::from_ast(ast_vec, maxid);
    if let Err(res) = res {
        return res.to_string();
    }
    let graph = res.unwrap();
    let res = dot::from_graph(&graph, curved);
    if let Err(res) = res {
        return res.to_string();
    }
    let dot = res.unwrap();
    dot
}