# 🧙‍♂️ Mage - A Magical Cross-Platform Automation Language ✨

Mage is a powerful, cross-platform scripting language designed to make automation magical. Unlike traditional shell scripts that are platform-specific and error-prone, Mage provides a unified, expressive syntax with built-in cross-platform functions.

## 🌟 Why Mage?

- **🔮 Magical Syntax**: Intuitive, fantasy-themed keywords make code readable and fun
- **🌍 Cross-Platform**: Write once, run everywhere (Windows, macOS, Linux)
- **🔧 Built-in Functions**: No more shell command wrappers - native functions for common tasks
- **🛡️ Error Handling**: Robust error handling and edge case management
- **📦 Package Management**: Intelligent package manager detection and installation
- **🎯 Type Smart**: Automatic type detection and conversion

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/your-username/mage.git
cd mage
cargo build --release
```

### Your First Spell

```mage
# Variables (conjure)
conjure name = "World"
conjure count = 3

# Output with interpolation (incant)
incant "Hello, ${name}!"

# Loops (chant for, recite foreach)
chant i from 1 to count {
    incant "Spell ${i} cast!"
}

# Built-in cross-platform functions (cast)
cast platform()
cast ensure_directory("magical-folder")
```

## 📚 Language Reference

### 🔤 Variables & Data Types

```mage
# Variable declaration
conjure message = "Hello, Mage!"
conjure number = 42
conjure flag = true
conjure pi = 3.14159

# String interpolation
conjure greeting = "Welcome, ${name}! You are ${age} years old."
incant "${greeting}"
```

### ➕ Expressions & Arithmetic

```mage
conjure a = 10
conjure b = 3

conjure sum = (a + b)        # Addition
conjure diff = (a - b)       # Subtraction  
conjure product = (a * b)    # Multiplication
conjure quotient = (a / b)   # Division
conjure remainder = (a % b)  # Modulo

incant "Sum: ${sum}, Product: ${product}"
```

### 🔀 Control Flow

```mage
# Conditional logic (scry/morph/lest = if/else-if/else)
conjure score = 85

scry score >= 90 {
    incant "Excellent!"
} morph score >= 80 {
    incant "Good job!"
} morph score >= 70 {
    incant "Not bad!"
} lest {
    incant "Keep trying!"
}
```

### 🔄 Loops

```mage
# For loops (chant from/to/step)
chant i from 1 to 6 {
    incant "Count: ${i}"
}

# For loops with step
chant i from 0 to 11 step 2 {
    incant "Even: ${i}"
}

# Backward loops
chant i from 5 to 1 step -1 {
    incant "Countdown: ${i}"
}

# Foreach loops (recite from)
conjure fruits = "apple,banana,orange"
recite fruit from fruits {
    incant "Fruit: ${fruit}"
}

# Numeric iteration
conjure count = 5
recite i from count {
    incant "Number: ${i}"
}

# While loops (channel)
conjure counter = 0
channel counter < 3 {
    incant "Counter: ${counter}"
    conjure counter = (counter + 1)
}
```

### 🎯 Functions

```mage
# Function definition (enchant)
enchant greet(name, title) {
    incant "Hello, ${title} ${name}!"
}

enchant calculate_area(length, width) {
    conjure area = (length * width)
    incant "Area: ${area} square units"
}

# Function calls (cast)
cast greet("Alice", "Dr.")
cast calculate_area(10, 5)
```

### 🔤 String Features

```mage
# Escape sequences
conjure text = "Line 1\nLine 2\tTabbed\nQuote: \"Hello\"\nPath: C:\\folder"
incant "${text}"

# String concatenation
conjure first = "Hello"
conjure second = "World"
conjure combined = (first + " " + second)
incant "${combined}"
```

## 🔧 Built-in Functions

### 💻 System Information

```mage
cast platform()           # Get OS (windows/macos/linux)
cast architecture()       # Get architecture (x86_64/aarch64)
cast home_directory()     # Get user home directory
cast current_directory()  # Get current working directory
cast env_var("PATH")      # Get environment variable
```

### 📦 Package Management

```mage
# Detect available package managers
cast detect_package_managers()

# Check if package is installed
cast package_installed("git")

# Install packages (with interactive selection for multiple matches)
cast install_package("python")

# Search for packages
cast search_package("text-editor")

# List installed packages
cast list_packages()
```

### 📁 File Operations

```mage
# Directory operations
cast ensure_directory("my-project")
cast remove_directory("temp-folder")

# File operations
cast write_file("config.txt", "key=value\nmode=production")
cast copy_file("source.txt", "backup.txt")
cast remove_file("temp.txt")
cast make_executable("script.sh")

# Symlinks (where supported)
cast symlink("target.txt", "link.txt")
```

### 🌐 Network Operations

```mage
# Download files
cast download("https://example.com/file.zip", "downloads/file.zip")
```

## 🎭 Advanced Examples

### Dotfiles Management

```mage
incant "🏠 Setting up dotfiles..."

# Detect system
cast platform()
conjure home = ""
cast home_directory()

# Create config directories
cast ensure_directory("${home}/.config")
cast ensure_directory("${home}/.config/nvim")

# Install essential packages
conjure packages = "git,curl,wget,vim"
recite package from packages {
    scry package_installed(package) {
        incant "✅ ${package} already installed"
    } lest {
        incant "📦 Installing ${package}..."
        cast install_package(package)
    }
}

# Setup configurations
cast write_file("${home}/.gitconfig", "[user]\n\tname = Your Name\n\temail = you@example.com")
cast write_file("${home}/.vimrc", "set number\nset tabstop=4\nsyntax on")

incant "🎉 Dotfiles setup complete!"
```

### Project Setup

```mage
enchant setup_project(project_name, project_type) {
    incant "🚀 Creating ${project_type} project: ${project_name}"
    
    # Create project structure
    cast ensure_directory(project_name)
    cast ensure_directory("${project_name}/src")
    cast ensure_directory("${project_name}/tests")
    
    scry project_type == "rust" {
        cast write_file("${project_name}/Cargo.toml", "[package]\nname = \"${project_name}\"\nversion = \"0.1.0\"")
        cast write_file("${project_name}/src/main.rs", "fn main() {\n    println!(\"Hello, world!\");\n}")
    } morph project_type == "python" {
        cast write_file("${project_name}/requirements.txt", "# Add your dependencies here")
        cast write_file("${project_name}/main.py", "#!/usr/bin/env python3\n\ndef main():\n    print(\"Hello, world!\")\n\nif __name__ == \"__main__\":\n    main()")
    }
    
    # Initialize git
    scry package_installed("git") {
        cast write_file("${project_name}/.gitignore", "target/\n__pycache__/\n.DS_Store")
        incant "✅ Project ${project_name} created successfully!"
    } lest {
        incant "⚠️ Git not found. Install git for version control."
    }
}

# Usage
cast setup_project("my-awesome-app", "rust")
```

## 🎮 Language Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `conjure` | Variable declaration | `conjure name = "Alice"` |
| `incant` | Output/print | `incant "Hello, ${name}!"` |
| `scry` | If condition | `scry x > 10 { ... }` |
| `morph` | Else-if condition | `morph x < 5 { ... }` |
| `lest` | Else | `lest { ... }` |
| `chant` | For loop | `chant i from 1 to 10 { ... }` |
| `recite` | Foreach loop | `recite item from list { ... }` |
| `channel` | While loop | `channel condition { ... }` |
| `enchant` | Function definition | `enchant func(param) { ... }` |
| `cast` | Function call | `cast my_function("arg")` |
| `curse` | Error/exit | `curse "Something went wrong!"` |

## 🔗 Comparison with Other Languages

| Feature | Bash | PowerShell | Python | Mage |
|---------|------|------------|--------|------|
| Cross-platform | ❌ | ⚠️ | ✅ | ✅ |
| Built-in package mgmt | ❌ | ⚠️ | ⚠️ | ✅ |
| Type safety | ❌ | ⚠️ | ✅ | ✅ |
| Readable syntax | ❌ | ⚠️ | ✅ | ✅ |
| No external deps | ✅ | ✅ | ❌ | ✅ |
| File operations | ⚠️ | ✅ | ⚠️ | ✅ |

## 🛠️ Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/your-username/mage.git
cd mage

# Build in debug mode
cargo build

# Build for release
cargo build --release

# Run tests
cargo test

# Install globally
cargo install --path .
```

### VS Code Extension

Install the Mage language extension for syntax highlighting, snippets, and IntelliSense support:

```bash
code --install-extension mage-language-0.1.0.vsix
```

## 📖 Examples

Check out the `examples/` directory for more comprehensive examples:

- `examples/dotfiles-setup.mage` - Complete dotfiles management
- `examples/project-generator.mage` - Multi-language project scaffolding
- `examples/system-info.mage` - System diagnostics and information gathering

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the need for better cross-platform automation
- Built with Rust for performance and safety
- Uses Pest for robust parsing
- Magic-themed syntax for developer joy

---

*Made with ✨ magic ✨ and Rust 🦀* 