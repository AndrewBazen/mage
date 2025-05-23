use tree_sitter::{Parser, Language, Tree};
use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};
use std::collections::HashMap;
use std::sync::{Once, Mutex};

// Highlight groups corresponding to queries
static HIGHLIGHT_NAMES: &[&str] = &[
    "keyword", "operator", "variable", "variable.declaration", "variable.parameter",
    "function", "function.call", "string", "string.escape", "string.special",
    "number", "constant.builtin", "comment", "comment.block", "punctuation.delimiter",
];

// HTML colors for highlight groups
static HTML_COLORS: &[&[u8]] = &[
    b"#569CD6", b"#D4D4D4", b"#9CDCFE", b"#4EC9B0", b"#9CDCFE",
    b"#DCDCAA", b"#DCDCAA", b"#CE9178", b"#D7BA7D", b"#569CD6",
    b"#B5CEA8", b"#569CD6", b"#6A9955", b"#6A9955", b"#D4D4D4",
];

// Singleton loader for tree-sitter language
static LANGUAGE_INIT: Once = Once::new();
static LANGUAGE: Mutex<Option<Language>> = Mutex::new(None);

#[link(name = "tree-sitter-mage")]
unsafe extern "C" {
    fn tree_sitter_mage() -> Language;
}

pub fn language() -> Language {
    LANGUAGE_INIT.call_once(|| {
        let lang = unsafe { tree_sitter_mage() };
        if let Ok(mut guard) = LANGUAGE.lock() {
            *guard = Some(lang);
        }
    });
    let guard = LANGUAGE.lock().unwrap();
    guard.as_ref().expect("tree-sitter-mage not initialized").clone()
}

pub fn is_tree_sitter_available() -> bool {
    LANGUAGE.lock().ok().is_some() || unsafe {
        std::panic::catch_unwind(|| tree_sitter_mage()).is_ok()
    }
}

pub fn parse(source: &str) -> Option<Tree> {
    let lang = language();
    let mut parser = Parser::new();
    parser.set_language(&lang).ok()?;
    parser.parse(source, None)
}

pub fn highlight_html(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lang = language();
    let query = include_str!("../tree-sitter-mage/queries/highlights.scm");

    let mut config = HighlightConfiguration::new(lang, query, "", "", "")?;
    config.configure(HIGHLIGHT_NAMES);

    let mut highlighter = Highlighter::new();
    let highlights = highlighter.highlight(&config, source.as_bytes(), None, |_| None)?;

    let mut renderer = HtmlRenderer::new();
    renderer.render(highlights, source.as_bytes(), &|h, _| {
        HTML_COLORS.get(h.0).copied().unwrap_or(b"#D4D4D4");
    })?;
    Ok(String::from_utf8(renderer.html)?)
}

pub fn get_ast_string(source: &str) -> String {
    parse(source)
        .map(|tree| tree.root_node().to_sexp().to_string())
        .unwrap_or_else(|| "Failed to parse".to_string())
}

#[derive(Default)]
pub struct TerminalColors {
    color_map: HashMap<&'static str, &'static str>,
}


impl TerminalColors {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("keyword", "\x1b[34m");
        map.insert("function", "\x1b[33m");
        map.insert("function.call", "\x1b[33m");
        map.insert("string", "\x1b[31m");
        map.insert("number", "\x1b[32m");
        map.insert("comment", "\x1b[90m");
        map.insert("variable.declaration", "\x1b[36m");
        map.insert("variable.parameter", "\x1b[36m");
        TerminalColors { color_map: map }
    }

    pub fn get_color(&self, highlight: &str) -> &'static str {
        self.color_map.get(highlight).copied().unwrap_or("\x1b[0m")
    }

    pub fn reset() -> &'static str {
        "\x1b[0m"
    }
}
