use std::path::Path;

// Simple build script that creates stub directories for future tree-sitter integration
fn main() {
    let src_dir = Path::new("tree-sitter-mage").join("src");
    let parser_path = src_dir.join("parser.c");
    
    // Check if parser file exists
    if parser_path.exists() {
        println!("cargo:rerun-if-changed=tree-sitter-mage/grammar.js");
        println!("cargo:rerun-if-changed=tree-sitter-mage/src/parser.c");
        println!("cargo:rerun-if-changed=tree-sitter-mage/src/scanner.c");
        
        // Compile the tree-sitter grammar
        cc::Build::new()
            .include(&src_dir)
            .file(parser_path)
            .compile("tree-sitter-mage");
            
        println!("cargo:rustc-link-lib=tree-sitter-mage");
    } else {
        println!("cargo:warning=Parser file not found at {:?}", parser_path);
        println!("cargo:warning=Run 'cd tree-sitter-mage && tree-sitter generate' to generate it");
    }
} 