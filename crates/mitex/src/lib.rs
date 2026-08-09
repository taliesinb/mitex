mod converter;

pub use mitex_parser::command_preludes;
use mitex_parser::parse;
use mitex_parser::parse_without_macro;
pub use mitex_parser::spec::*;

use converter::convert_inner;
pub use converter::convert_node;
pub use converter::LaTeXMode;
pub use mitex_parser::syntax::SyntaxNode;

pub fn convert_text(input: &str, spec: Option<CommandSpec>) -> Result<String, String> {
    convert_inner(input, LaTeXMode::Text, spec, parse)
}

pub fn convert_math(input: &str, spec: Option<CommandSpec>) -> Result<String, String> {
    convert_inner(input, LaTeXMode::Math, spec, parse)
}

/// Convert an already-parsed math (sub)tree to typst code.
pub fn convert_math_node(node: SyntaxNode, spec: Option<CommandSpec>) -> Result<String, String> {
    convert_node(node, LaTeXMode::Math, spec)
}

/// For internal testing
pub fn convert_math_no_macro(input: &str, spec: Option<CommandSpec>) -> Result<String, String> {
    convert_inner(input, LaTeXMode::Math, spec, parse_without_macro)
}
