extern crate tree_sitter;
use tree_sitter::{Parser, Language};

fn main() {
  let mut parser = Parser::new();

  extern "C" { fn tree_sitter_rust() -> Language; }

  let language = unsafe { tree_sitter_rust() };
  parser.set_language(language).unwrap();

  let source_code = "fn test() {}";
  println!("source: {}", source_code);
  let tree = parser.parse_str(source_code, None).unwrap();
  let root_node = tree.root_node();
  println!("AST: {}", root_node.to_sexp());

  let mut indent = 0;
  print_node(&root_node, source_code, indent);
  let mut tree_cursor = root_node.walk();
  let mut level = 0;
  let mut child_printed = false;
  while true {
    if tree_cursor.goto_next_sibling() {
      child_printed = false;
      print_node(&tree_cursor.node(), source_code, indent);
      if tree_cursor.goto_first_child() {
        indent +=1;
        level += 1;
        print_node(&tree_cursor.node(), source_code, indent);
      }
    } else if !child_printed && tree_cursor.goto_first_child() {
      indent +=1;
      print_node(&tree_cursor.node(), source_code, indent);
    } else {
        if level > 0 {
          tree_cursor.goto_parent();
          child_printed = true;
          level -= 1;
          indent -=1;
        } else {
          break;
        }
    }
  }
}

fn print_node(node: &tree_sitter::Node, source_code: &str, indent: usize) {
  let separator = "  ".repeat(indent);

  println!("{}- node: {}", separator, node.to_sexp());
  println!("{}  kind: {}", separator, node.kind());
  println!("{}  start: {}, {}", separator, node.start_position().row, node.start_position().column);
  println!("{}  end: {}, {}", separator, node.end_position().row, node.end_position().column);
  let chunk:String = source_code.chars().skip(node.start_byte()).take(node.end_byte()-node.start_byte()).collect();
  println!("{}  content: {}", separator, chunk);
}
