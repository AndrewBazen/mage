# Mage - A Cross-Platform Automation Language

Mage is a cross-platform scripting language designed for automation. Unlike traditional shell scripts that are platform-specific and error-prone, Mage provides a unified, expressive syntax with built-in cross-platform functions.

## Why Mage?

- **Cross-Platform**: Write once, run everywhere (Windows, macOS, Linux)
- **Readable Syntax**: Intuitive keywords make code readable and maintainable
- **Built-in Functions**: Native functions for common tasks - no shell command wrappers
- **Error Handling**: Try-catch style error handling with `invoke`/`seal`
- **Package Management**: Intelligent package manager detection and installation
- **Type Smart**: Automatic type detection and conversion

## Project Structure

```
mage/
├── crates/
│   ├── mage-core/       # Core interpreter, parser, builtins (library)
│   ├── mage-cli/        # Command-line interface and REPL
│   └── mage-tui/        # Terminal UI shell (in development)
├── tests/               # Test .mage files
├── examples/            # Example scripts
├── editor-support/      # VS Code/Cursor extension
└── assets/              # Icons and images
```

## Quick Start

### Installation

```bash
git clone https://github.com/AndrewBazen/mage.git
cd mage
cargo build --release
```

### Run a Script

```bash
./target/release/mage script.mage
```

### Start the REPL

```bash
./target/release/mage
```

### Your First Spell

```mage
# Variables
conjure name = "World"
conjure count = 3

# Output with interpolation
incant "Hello, ${name}!"

# Loops
chant i from 1 to count {
    incant "Spell ${i} cast!"
}

# Built-in cross-platform functions
conjure os = cast platform()
incant "Running on: ${os}"
```

## Language Reference

### Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `conjure` | Variable declaration | `conjure name = "Alice"` |
| `incant` | Output/print | `incant "Hello, ${name}!"` |
| `evoke` | Execute shell command | `evoke "ls -la"` |
| `scry` | If condition | `scry x > 10 { ... }` |
| `morph` | Else-if condition | `morph x < 5 { ... }` |
| `lest` | Else | `lest { ... }` |
| `chant` | For loop | `chant i from 1 to 10 { ... }` |
| `recite` | Foreach loop | `recite item from list { ... }` |
| `channel` | While loop | `channel condition { ... }` |
| `loop` | Infinite loop | `loop { ... }` |
| `enchant` | Function definition | `enchant func(param) { ... }` |
| `cast` | Function call | `cast my_function("arg")` |
| `bestow` | Return value | `bestow result` |
| `invoke` | Try block | `invoke { ... } seal { ... }` |
| `seal` | Catch block | `seal (err) { incant err }` |
| `summon` | Throw error | `summon "Something went wrong!"` |
| `dispel` | Break loop | `dispel` |
| `portal` | Continue loop | `portal` |
| `curse` | Exit program | `curse "Fatal error"` |

### Variables & Data Types

```mage
conjure message = "Hello, Mage!"
conjure number = 42
conjure flag = true
conjure pi = 3.14159

# String interpolation
conjure greeting = "Welcome, ${name}!"
incant greeting
```

### Control Flow

```mage
conjure score = 85

scry score >= 90 {
    incant "Excellent!"
} morph score >= 80 {
    incant "Good job!"
} lest {
    incant "Keep trying!"
}
```

### Loops

```mage
# For loop
chant i from 1 to 5 {
    incant "Count: ${i}"
}

# For loop with step
chant i from 0 to 10 step 2 {
    incant "Even: ${i}"
}

# While loop
conjure counter = 0
channel counter < 3 {
    incant "Counter: ${counter}"
    conjure counter = counter + 1
}

# Infinite loop with break
loop {
    incant "Running..."
    scry should_stop == true {
        dispel
    }
}
```

### Functions

```mage
enchant greet(name) {
    incant "Hello, ${name}!"
}

enchant add(a, b) {
    bestow a + b
}

cast greet("Alice")
conjure sum = cast add(5, 3)
```

### Error Handling

```mage
invoke {
    # Code that might fail
    summon "Something went wrong!"
} seal (err) {
    incant "Caught error: ${err}"
}
```

### String Methods

```mage
conjure name = "hello"
conjure upper = name.upper()    # "HELLO"
conjure lower = name.lower()    # "hello"
conjure len = name.len()        # 5
```

## Built-in Functions

### System Information

```mage
cast platform()           # windows/macos/linux
cast architecture()       # x86_64/aarch64
cast home_directory()     # User home directory
cast current_directory()  # Current working directory
cast env_var("PATH")      # Environment variable
```

### File Operations

```mage
cast ensure_directory("my-project")
cast write_file("config.txt", "key=value")
cast read_file("config.txt")
cast copy_file("source.txt", "backup.txt")
cast remove_file("temp.txt")
cast file_exists("config.txt")
cast make_executable("script.sh")
```

### Package Management

```mage
cast detect_package_managers()
cast package_installed("git")
cast install_package("python")
cast search_package("editor")
```

## Development

### Building

```bash
# Build all crates
cargo build --workspace

# Build release
cargo build --workspace --release

# Run CLI
cargo run -p mage-cli -- script.mage

# Run TUI (in development)
cargo run -p mage-tui

# Run tests
cargo test --workspace
```

### Project Crates

- **mage-core**: Core library with interpreter, parser, and built-in functions
- **mage-cli**: Command-line interface with REPL and syntax highlighting
- **mage-tui**: Terminal UI shell with visual interface (in development)

### Editor Support

VS Code/Cursor extension available in `editor-support/vscode/`:

```powershell
# Install in Cursor
Copy-Item -Recurse -Force editor-support\vscode\* $env:USERPROFILE\.cursor\extensions\mage-lang\
```

Features:
- Syntax highlighting
- Code snippets
- File icons

## Roadmap

### In Progress

- [ ] **TUI Shell** - Terminal UI with:
  - Split-pane layout (output + context panel)
  - Context-aware command suggestions (like which-key)
  - File browser panel
  - Git integration panel
  - Customizable themes and layouts

### Planned

- [ ] Spellbooks - Command metadata files for contextual help
- [ ] Plugin system for extending built-in functions
- [ ] Language server protocol (LSP) support
- [ ] More string/list/map methods
- [ ] Module/import system

## Examples

See the `examples/` directory:

- `system-info.mage` - System diagnostics
- `package-workflow-demo.mage` - Package management
- `cross-platform-dotfiles.mage` - Dotfiles setup
- `project-generator.mage` - Project scaffolding

## Contributing

Contributions welcome! Please open an issue or submit a PR.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

Built with Rust
