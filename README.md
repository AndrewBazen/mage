# Mage Language

Mage is a cross-platform, magic-themed scripting language built in Rust. It is designed to be expressive, extensible, and fun, with a syntax inspired by spells and incantations.

## Features

- **Variables:**
  - `conjure name = "Gandalf"`
- **String Interpolation:**
  - Supports `$var` and `${var}` in strings, with escaping (e.g., `\$`, `\{`, `\\`).
- **Output:**
  - `incant "Hello, $name!"`
- **Error Handling:**
  - `curse "Something went wrong!"` (prints error and exits)
- **Run Shell Commands:**
  - `evoke "ls -la"` (cross-platform: supports bash, zsh, fish, sh, or cmd)
- **Comments:**
  - Single-line: `# This is a comment`
  - Multi-line: 
    ```
    ##
    # This is a multi-line comment
    # Another line
    ##
    ```
- **Control Flow:**
  - If: 
    ```
    if name == "Mage" {
        incant "Welcome, $name!"
    }
    ```
  - Loop (fixed 3 times):
    ```
    loop {
        incant "Looping!"
    }
    ```
- **Functions:**
  - Define: 
    ```
    enchant greet(name) {
        incant "Hello, $name!"
    }
    ```
  - Call: 
    ```
    cast greet("Mage")
    ```
- **Syntax Highlighting:**
  - Tree-sitter grammar for precise syntax highlighting
  - Colored REPL with syntax highlighting
  - Export highlighted code to HTML

## Cross-Platform Shell Support
- On Unix: uses `MAGE_SHELL` env var, then `SHELL`, then tries `bash`, `zsh`, `fish`, `sh`.
- On Windows: uses `cmd`.
- Override with command-line flag: `--shell powershell`
- Override in script with directive: `#!shell: powershell`
- Configure in `.mageconfig` file: `shell=powershell`

## CLI Commands

```
mage [SCRIPT]               Run a script directly
mage run <SCRIPT>           Run a script
mage repl                   Start interactive REPL mode
mage init                   Create a .mageconfig file
mage highlight <SCRIPT>     Test tree-sitter highlighting
mage --shell <SHELL>        Override shell for script execution
mage --help                 Show help information
```

## Configuration
Create a `.mageconfig` file in your project directory with:
```
# Override default shell
shell=powershell

# Custom options
project_name=My Mage Project
```

## Example Script
```mage
conjure name = "Mage"
incant "Welcome, $name!"
##
# This is a multi-line comment
# Another line
##
if name == "Mage" {
    incant "You are the archmage!"
}
enchant greet(who) {
    incant "Greetings, $who!"
}
cast greet("Gandalf")
evoke "echo Hello from the shell!"
```

## Installation & Usage
1. **Clone and build:**
    ```sh
    git clone <repo-url>
    cd mage
    cargo build --release
    ```
2. **Run a script:**
    ```sh
    cargo run -- run path/to/script.mage
    ```
3. **Start REPL mode:**
    ```sh
    cargo run -- repl
    ```
4. **Override shell:**
    ```sh
    cargo run -- run path/to/script.mage --shell powershell
    ```
5. **Generate highlighted HTML:**
    ```sh
    cargo run -- highlight path/to/script.mage
    ```

## Editor Integration

### VSCode
The tree-sitter grammar can be used with VSCode:

1. Install the "Tree Sitter Queries" extension
2. Create a `.vscode/settings.json` file with:
```json
{
  "treeSitterQueries.extension": "mage",
  "treeSitterQueries.grammar": "tree-sitter-mage"
}
```

### Other Editors
For other editors that support tree-sitter (like Neovim, Emacs), copy the `tree-sitter-mage` directory to your editor's tree-sitter grammar location.

## Testing
Run all tests:
```sh
cargo test
```

## License
MIT 