use tree_sitter::Language;

extern "C" {
    fn tree_sitter_mage() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_mage() }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let language = super::language();
        assert!(language.node_kind_count() > 0);
    }
} 