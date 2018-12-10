extern crate cc;

use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let mut config = cc::Build::new();
    let root_path: PathBuf = ["vendor", "tree-sitter"].iter().collect();

    config
        .define("UTF8PROC_STATIC", "")
        .flag_if_supported("-std=c99")
        .flag_if_supported("-Wno-unused-parameter")
        .include(root_path.join("src"))
        .include(root_path.join("include"))
        .include(root_path.join("externals").join("utf8proc"))
        .file(root_path.join("src").join("runtime").join("runtime.c"));

    if env::var("RUST_TREE_SITTER_TEST").is_ok() {
        let paths = fs::read_dir("fixtures").unwrap();

        for path in paths {
            let mut parser_dir: PathBuf = ["fixtures", path.unwrap().file_name().to_str().unwrap(), "src"].iter().collect();
            if parser_dir.join("parser.c").exists() {
               config.file(parser_dir.join("parser.c"));
            }
            if parser_dir.join("scanner.c").exists() {
               config.file(parser_dir.join("scanner.c"));
            }
//            if parser_dir.join("scanner.cc").exists() {
//               config.file(parser_dir.join("scanner.cc"));
//            }
        }
    }

    config.compile("tree-sitter-runtime");
}
