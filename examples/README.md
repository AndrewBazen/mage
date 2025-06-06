# 🧙‍♂️ Mage Examples

This directory contains example scripts that demonstrate various features and use cases of the Mage language.

## 🎯 Core Examples

### [`comprehensive-demo.mage`](comprehensive-demo.mage)
**Complete language feature demonstration**
- All Mage language features in one script
- Variables, expressions, control flow, loops, functions
- Built-in functions for system info, package management, file operations
- Perfect for learning the language or testing functionality

### [`system-info.mage`](system-info.mage)
**System information and diagnostics**
- Cross-platform system detection
- Environment variable access
- Hardware and OS information gathering
- Useful for system administration scripts

### [`dotfiles-setup.mage`](dotfiles-setup.mage)
**Modern dotfiles management**
- Cross-platform configuration setup
- Uses built-in functions (no shell commands)
- File operations, directory creation, symlinks
- Demonstrates zero-dependency automation

## 📦 Package Management Examples

### [`interactive-package-demo.mage`](interactive-package-demo.mage)
**Interactive package installation**
- Package manager detection
- Interactive package selection when multiple matches found
- Cross-platform package installation
- User-friendly package management

### [`dynamic-package-search.mage`](dynamic-package-search.mage)
**Dynamic package searching**
- Search packages across different package managers
- Parse package manager output
- Handle multiple package sources
- Advanced package discovery

## 🏗️ Project Setup Examples

### [`project-generator.mage`](project-generator.mage)
**Multi-language project scaffolding**
- Create project structures for different languages
- Template generation and file creation
- Git initialization and configuration
- Automated development environment setup

### [`cross-platform-dotfiles.mage`](cross-platform-dotfiles.mage)
**Advanced cross-platform dotfiles**
- Comprehensive dotfiles management
- OS-specific configurations
- Package installation and setup
- Shell and editor configuration

### [`env-specific-setup.mage`](env-specific-setup.mage)
**Environment-specific setup**
- Development vs production configurations
- Environment variable management
- Conditional setup based on environment
- Configuration templating

## 🔧 Maintenance Examples

### [`dotfiles-maintenance.mage`](dotfiles-maintenance.mage)
**Dotfiles maintenance and updates**
- Update existing configurations
- Backup and restore functionality
- Configuration validation
- Maintenance automation

## 📋 Configuration Files

### [`.mageconfig`](.mageconfig)
**Basic Mage configuration**
- Project-level configuration example
- Shell and environment settings

### [`dotfiles.mageconfig`](dotfiles.mageconfig)
**Dotfiles-specific configuration**
- Configuration for dotfiles management
- Custom variables and settings

## 🚀 Getting Started

1. **Start with the comprehensive demo:**
   ```bash
   mage examples/comprehensive-demo.mage
   ```

2. **Try system information gathering:**
   ```bash
   mage examples/system-info.mage
   ```

3. **Set up dotfiles (safe demo):**
   ```bash
   mage examples/dotfiles-setup.mage
   ```

4. **Explore package management:**
   ```bash
   mage examples/interactive-package-demo.mage
   ```

## 💡 Tips

- All examples are designed to be safe to run
- File operations use temporary directories when possible
- Package installations are often commented out for safety
- Examples demonstrate best practices for cross-platform scripting

## 🤝 Contributing

Feel free to add your own examples! Make sure they:
- Are well-commented and educational
- Use built-in functions when possible
- Work cross-platform
- Include safety measures for destructive operations

---

*Happy scripting with Mage! 🧙‍♂️✨* 