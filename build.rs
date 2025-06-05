use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["tree-sitter-mage", "src"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree-sitter-mage");
    
    println!("cargo:rerun-if-changed=tree-sitter-mage/src/parser.c");
    println!("cargo:rerun-if-changed=tree-sitter-mage/src/tree_sitter");
} 