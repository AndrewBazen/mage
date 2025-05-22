use tree_sitter::{Parser, Language, Tree};
use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};
use std::collections::HashMap;
use std::sync::{Once, Mutex, LazyLock};

// Define highlight names that correspond to our queries
static HIGHLIGHT_NAMES: &[&str] = &[
    "keyword",
    "operator",
    "variable",
    "variable.declaration",
    "variable.parameter",
    "function",
    "function.call",
    "string",
    "string.escape",
    "string.special",
    "number",
    "constant.builtin",
    "comment",
    "comment.block",
    "punctuation.delimiter",
];

// Colors for HTML rendering
static HTML_COLORS: &[&[u8]] = &[
    b"#569CD6", // keyword - blue
    b"#D4D4D4", // operator - white
    b"#9CDCFE", // variable - light blue
    b"#4EC9B0", // variable.declaration - teal 
    b"#9CDCFE", // variable.parameter - light blue
    b"#DCDCAA", // function - yellow
    b"#DCDCAA", // function.call - yellow
    b"#CE9178", // string - orange
    b"#D7BA7D", // string.escape - gold
    b"#569CD6", // string.special - blue
    b"#B5CEA8", // number - light green
    b"#569CD6", // constant.builtin - blue
    b"#6A9955", // comment - green
    b"#6A9955", // comment.block - green
    b"#D4D4D4", // punctuation.delimiter - white
];

// Singleton pattern for tree-sitter language
static LANGUAGE_INIT: Once = Once::new();
static LANGUAGE: Mutex<Option<Language>> = Mutex::new(None);

// External tree-sitter language function
#[link(name = "tree-sitter-mage")]
unsafe extern "C" {
    fn tree_sitter_mage() -> Language;
}

// Get tree-sitter language
pub fn language() -> Language {
    LANGUAGE_INIT.call_once(|| {
        let lang = unsafe { tree_sitter_mage() };
        if let Ok(mut guard) = LANGUAGE.lock() {
            *guard = Some(lang);
        }
    });
    
    LANGUAGE.lock()
        .map(|guard| guard.unwrap_or_else(|| panic!("Failed to load tree-sitter language")))
        .unwrap_or_else(|_| panic!("Failed to acquire lock on language"))
}

// Check if tree-sitter is available
pub fn is_tree_sitter_available() -> bool {
    if let Ok(guard) = LANGUAGE.lock() {
        if guard.is_some() {
            return true;
        }
    }
    
    // Try to load the language
    unsafe {
        match std::panic::catch_unwind(|| tree_sitter_mage()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

// Parse Mage code
pub fn parse(source: &str) -> Option<Tree> {
    let lang = language();
    let mut parser = Parser::new();
    parser.set_language(lang).ok()?;
    parser.parse(source, None)
}

// Highlight Mage code and return HTML
pub fn highlight_html(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lang = language();
    
    // Load the query from file if it exists, otherwise use a default
    let highlight_query = include_str!("../tree-sitter-mage/queries/highlights.scm");
    
    let mut config = HighlightConfiguration::new(
        lang,
        highlight_query,
        "",  // No injections query for now
        "",  // No locals query for now
    )?;
    config.configure(HIGHLIGHT_NAMES);
    
    let mut highlighter = Highlighter::new();
    let highlights = highlighter.highlight(
        &config,
        source.as_bytes(),
        None,
        |_| None,
    )?;
    
    let mut renderer = HtmlRenderer::new();
    renderer.render(
        highlights,
        source.as_bytes(),
        &|highlight| HTML_COLORS[highlight.0],
    )?;
    
    Ok(String::from_utf8(renderer.html)?)
}

// Get AST as string (for debugging)
pub fn get_ast_string(source: &str) -> String {
    if let Some(tree) = parse(source) {
        format!("{}", tree.root_node().to_sexp())
    } else {
        "Failed to parse".to_string()
    }
}

// Terminal ANSI color codes for the REPL
pub struct TerminalColors {
    color_map: HashMap<&'static str, &'static str>,
}

impl TerminalColors {
    pub fn new() -> Self {
        let mut color_map = HashMap::new();
        color_map.insert("keyword", "\x1b[34m");      // Blue
        color_map.insert("function", "\x1b[33m");     // Yellow
        color_map.insert("function.call", "\x1b[33m"); // Yellow
        color_map.insert("string", "\x1b[31m");       // Red
        color_map.insert("number", "\x1b[32m");       // Green
        color_map.insert("comment", "\x1b[90m");      // Bright black
        color_map.insert("variable.declaration", "\x1b[36m"); // Cyan
        color_map.insert("variable.parameter", "\x1b[36m");   // Cyan
        
        TerminalColors { color_map }
    }
    
    pub fn get_color(&self, highlight_type: &str) -> &'static str {
        self.color_map.get(highlight_type).unwrap_or(&"\x1b[0m")
    }
    
    pub fn reset() -> &'static str {
        "\x1b[0m"
    }
} 