mod ast;
mod dot;
mod graph;
mod parser;

pub fn generate_dot_from_cpp(content: String, func: Option<String>, curved: bool) -> String {
    let content = content.as_bytes();
    let ast_vec = parser::parse(&content, func).unwrap();
    let graph = graph::from_ast(ast_vec).unwrap();
    let dot = dot::from_graph(&graph, curved).unwrap();
    dot
}