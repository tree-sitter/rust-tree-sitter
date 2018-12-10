extern crate tree_sitter;
use tree_sitter::{Parser, Language};
use std::env;

fn main() {
  let mut parser = Parser::new();

  extern "C" { fn tree_sitter_rust() -> Language; }
  extern "C" { fn tree_sitter_c() -> Language; }
  extern "C" { fn tree_sitter_java() -> Language; }
  extern "C" { fn tree_sitter_javascript() -> Language; }
  extern "C" { fn tree_sitter_python() -> Language; }
  extern "C" { fn tree_sitter_ruby() -> Language; }
  extern "C" { fn tree_sitter_haskell() -> Language; }
  extern "C" { fn tree_sitter_cpp() -> Language; }
  extern "C" { fn tree_sitter_go() -> Language; }
  extern "C" { fn tree_sitter_bash() -> Language; }
  extern "C" { fn tree_sitter_css() -> Language; }
  extern "C" { fn tree_sitter_json() -> Language; }
  extern "C" { fn tree_sitter_php() -> Language; }

  let args: Vec<String> = env::args().collect();

  let mut source_code = "fn test() {}";
  if args.len() > 1 {
    source_code = &args[1];
  }
  println!("source: {}", source_code);

  let mut language_name = "rust";
  if args.len() > 2 {
    language_name = &args[2];
  }
  let mut language = unsafe { tree_sitter_rust() };
  if language_name == "c" {
    language = unsafe { tree_sitter_c() };
//  } else if language_name == "cpp" {
//    language = unsafe { tree_sitter_cpp() };
  } else if language_name == "go" {
    language = unsafe { tree_sitter_go() };
  } else if language_name == "javascript" {
    language = unsafe { tree_sitter_javascript() };
  } else if language_name == "java" {
    language = unsafe { tree_sitter_java() };
//  } else if language_name == "embedded_template" {
//    language = unsafe { tree_sitter_embedded_template() };
//  } else if language_name == "html" {
//    language = unsafe { tree_sitter_html() };
//  } else if language_name == "python" {
//    language = unsafe { tree_sitter_python() };
//  } else if language_name == "ruby" {
//    language = unsafe { tree_sitter_ruby() };
//  } else if language_name == "haskell" {
//    language = unsafe { tree_sitter_haskell() };
//  } else if language_name == "bash" {
//    language = unsafe { tree_sitter_bash() };
//  } else if language_name == "php" {
//    language = unsafe { tree_sitter_php() };
  } else if language_name == "css" {
    language = unsafe { tree_sitter_css() };
  } else if language_name == "json" {
    language = unsafe { tree_sitter_json() };
  } else if language_name == "rust" {
    language = unsafe { tree_sitter_rust() };
  } else {
    language = unsafe { tree_sitter_rust() };
  }
  parser.set_language(language).unwrap();

  let tree = parser.parse_str(&source_code, None).unwrap();
  let root_node = tree.root_node();
  println!("AST: {}", root_node.to_sexp());

  let mut indent = 0;
  print_node(&root_node, &source_code, indent);
  let mut tree_cursor = root_node.walk();
  let mut level = 0;
  let mut child_printed = false;
  loop {
    if tree_cursor.goto_next_sibling() {
      child_printed = false;
      print_node(&tree_cursor.node(), &source_code, indent);
      if tree_cursor.goto_first_child() {
        indent +=1;
        level += 1;
        print_node(&tree_cursor.node(), &source_code, indent);
      }
    } else if !child_printed && tree_cursor.goto_first_child() {
      indent +=1;
      print_node(&tree_cursor.node(), &source_code, indent);
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
