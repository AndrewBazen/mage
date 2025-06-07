# Editor Support for Mage Language

This directory contains editor support files for the Mage language that are designed to be consumed by separate editor extension repositories.

## 📁 Directory Structure

```
editor-support/
└── vscode/
    ├── package.json              # Extension manifest and configuration
    ├── language-configuration.json # Language configuration
    ├── syntaxes/
    │   └── mage.tmLanguage.json  # Syntax highlighting grammar
    ├── snippets/
    │   └── mage.json             # Code snippets
    └── icons/
        └── mage-icon-theme.json  # Icon theme definition
```

## 🔗 VSCode Extension

The VSCode extension for Mage is maintained in a **separate repository** that:

1. **Pulls from this repository**: The extension repo syncs files from `editor-support/vscode/`
2. **Adds build tooling**: Includes webpack, packaging, and publishing workflows  
3. **Manages releases**: Handles version bumping and marketplace publishing
4. **Tests integration**: Runs extension-specific tests and validation

### **Files Synced to Extension Repository:**

- `package.json` - Extension metadata and configuration
- `language-configuration.json` - Bracket matching, commenting, etc.
- `syntaxes/mage.tmLanguage.json` - Syntax highlighting rules
- `snippets/mage.json` - Code completion snippets
- `icons/mage-icon-theme.json` - File icon definitions
- `assets/icons/mage64.png` - Icon file (from main assets directory)

## 🚀 Workflow

1. **Language changes** are made in this repository
2. **Extension repository** pulls changes automatically or manually
3. **Extension is built** and tested in the extension repo
4. **Extension is published** to VS Code Marketplace

## 📝 Updating Editor Support

To update VSCode support:

1. Edit files in `editor-support/vscode/`
2. Commit and push to this repository
3. The extension repository will sync changes
4. Extension repository handles building and publishing

### **Adding New Features:**

- **Keywords**: Update `syntaxes/mage.tmLanguage.json`
- **Snippets**: Add to `snippets/mage.json`  
- **Configuration**: Modify `language-configuration.json`
- **Icons**: Update `icons/mage-icon-theme.json`

## 🎯 Other Editors

Future editor support can be added by creating new directories:

```
editor-support/
├── vscode/     # Visual Studio Code
├── vim/        # Vim/Neovim
├── emacs/      # Emacs
└── sublime/    # Sublime Text
```

Each editor directory should contain the appropriate configuration files for that editor's plugin/extension system.

## 🔧 Validation

The main repository's CI validates that:
- JSON files are well-formed
- TextMate grammar is valid
- Package.json follows VSCode extension schema
- All referenced files exist

The extension repository handles:
- Extension packaging
- Marketplace validation
- Integration testing
- Publishing workflows

---

**Note**: This approach keeps the language definition separate from extension-specific build tooling, making it easier to maintain and support multiple editors. 