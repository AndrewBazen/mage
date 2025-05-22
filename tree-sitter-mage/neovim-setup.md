# Setting up Mage in Neovim

## Using LazyVim

Create a file in your LazyVim configuration directory at `lua/plugins/mage.lua` with this content:

```lua
return {
  -- TreeSitter configuration for Mage language
  {
    "nvim-treesitter/nvim-treesitter",
    opts = function(_, opts)
      -- Add parser configuration
      local parser_config = require("nvim-treesitter.parsers").get_parser_configs()
      parser_config.mage = {
        install_info = {
          url = "~/projects/mage", -- Local path to Mage repository
          files = {"src/parser.c"}, -- Corrected path to parser.c
          location = "tree-sitter-mage",
          branch = "main", -- Specify the correct branch name
          requires_generate_from_grammar = false,
          generate_requires_npm = false,
        },
        filetype = "mage",
      }
      
      -- Add mage to the list of parsers to install
      if type(opts.ensure_installed) == "table" then
        table.insert(opts.ensure_installed, "mage")
      end
    end,
  },
  
  -- Filetype and syntax configuration for Mage
  {
    "neovim/nvim-lspconfig", -- Using this as a base plugin that loads by default
    event = "BufReadPre *.mage",
    config = function()
      -- Register the filetype
      vim.filetype.add({
        extension = {
          mage = "mage",
        },
      })
    end,
  },
}
```

The LazyVim config directory is typically:
- Linux/macOS: `$HOME/.config/nvim/`
- Windows: `%LOCALAPPDATA%\nvim\`

## Using Other Neovim Setups

For other Neovim configurations, ensure you:

1. Add the Mage TreeSitter parser configuration to your init.lua or equivalent:
   ```lua
   local parser_config = require("nvim-treesitter.parsers").get_parser_configs()
   parser_config.mage = {
     install_info = {
       url = "~/projects/mage", -- Local path to Mage repository
       files = {"src/parser.c"}, -- Corrected path to parser.c
       location = "tree-sitter-mage",
       branch = "main", -- Specify the correct branch name
     },
     filetype = "mage",
   }
   ```

2. Register the .mage filetype:
   ```lua
   vim.filetype.add({
     extension = {
       mage = "mage",
     },
   })
   ```

3. Install the parser from within Neovim:
   ```
   :TSInstall mage
   ```

## Troubleshooting

### Common Issues

1. **"pathspec 'master' did not match any file(s) known to git"**
   - The repository doesn't have a 'master' branch. Make sure to set `branch = "main"` or whatever branch your repository uses in the install_info.

2. **Compilation errors**
   - Make sure the repository has the following structure:
     - `src/parser.c` - Generated parser code
     - `src/tree_sitter/parser.h` - Tree-sitter header
     - `bindings/node/index.js` - Node.js bindings
     - `binding.gyp` - Build configuration
     - `tree-sitter.json` - Tree-sitter configuration with proper metadata

3. **Parser generation issues**
   - If you're developing the grammar, run these commands to regenerate the parser:
     ```
     npm install
     npx tree-sitter generate
     ```

4. **Missing files after installation**
   - Run `:checkhealth nvim-treesitter` in Neovim to check for issues
   - Make sure all required Node.js packages are installed: `npm install nan`

5. **"No such file or directory" during compilation**
   - Double-check the `files` array in your parser configuration. The paths should be relative to the repository root, not to the tree-sitter-mage directory.
   - If using a local repository, ensure the absolute path is correct.
   
6. **TreeSitter query parsing errors**
   - Some Neovim versions have issues with syntax in TreeSitter query files. Try completely removing the query files first, then add minimal queries after the parser is working.
   - In some cases, you may need to change the parser name entirely to avoid cached configurations.
   - Verify your highlights.scm file doesn't use complex syntax like lists in square brackets. 

7. **Timestamp error in parser.c file**
   - If you encounter an error like `expected '=', ',', ';', 'asm' or '__attribute__' before numeric constant` with a timestamp in parser.c, this is due to the parser generator incorrectly adding a timestamp to the code.
   - Solution: Use the clean version of the parser:
     ```
     cp src/clean-parser.c src/parser.c
     gcc -o src/parser.o -c src/parser.c -I src/tree_sitter -fPIC
     gcc -shared -o parser.so src/parser.o
     ```
   - This will create a shared library that can be loaded by Neovim.

8. **"Failed to load symbol tree_sitter_mage" error**
   - This happens when there's a naming mismatch between what Neovim is looking for and what your parser exports.
   - Neovim looks for a function called `tree_sitter_mage` but your code might be exporting `tree_sitter_mage2`.
   - Solution: Make sure all these files use the same name:
     - In `src/parser.c`: The exported function should be `tree_sitter_mage(void)`
     - In `bindings/node/binding.cc`: Use `tree_sitter_mage()` and `NODE_MODULE(tree_sitter_mage_binding, Init)`
     - In `tree-sitter.json`: Use `"language-name": "mage"`
     - In `package.json`: Use `"name": "tree-sitter-mage"`
     - In `binding.gyp`: Use `"target_name": "tree_sitter_mage_binding"`
     - In Neovim configuration: Use `parser_config.mage` and `:TSInstall mage`

9. **Dealing with persistent caching issues**
   - If Neovim is still loading cached versions of the parser despite your changes:
     1. Clear the build directory: `Remove-Item -Path ./build -Recurse -Force`
     2. Rename any old parser files: `Rename-Item -Path "src/parser-mage2.c" -NewName "parser-mage2.c.bak"`
     3. Rebuild the parser completely: 
        ```
        gcc -o src/parser.o -c src/parser.c -I src/tree_sitter -fPIC
        gcc -shared -o libtree-sitter-mage.so src/parser.o
        ```
     4. Create a parsers directory in your Neovim config: `New-Item -Path "$env:LOCALAPPDATA\nvim\parsers" -ItemType Directory -Force`
     5. Copy the parser to the Neovim parsers directory with the correct name: `Copy-Item -Path ".\libtree-sitter-mage.so" -Destination "$env:LOCALAPPDATA\nvim\parsers\mage.so"`
     6. Update your Neovim configuration to append the parsers directory to the runtime path:
        ```lua
        -- Point to the manually compiled parser
        local parser_path = vim.fn.stdpath("config") .. "/parsers"
        vim.opt.runtimepath:append(parser_path)
        ```
     7. In Neovim, completely clear the parser cache with `:TSUninstall mage` followed by `:TSInstall mage`
   - This approach ensures that Neovim uses your manually compiled parser instead of trying to recompile it. 

10. **Neovim crashes when editing Mage files**
   - If Neovim crashes when typing in Mage files, the parser may have bugs or compatibility issues
   - Solutions:
     1. Simplify the highlights.scm file first:
        ```scm
        ;; Keywords
        "conjure" @keyword
        "incant" @keyword
        "curse" @keyword
        "evoke" @keyword
        "enchant" @keyword
        "cast" @keyword
        "if" @keyword
        "else" @keyword
        "loop" @keyword

        ;; Punctuation
        "(" @punctuation.delimiter
        ")" @punctuation.delimiter
        "{" @punctuation.delimiter
        "}" @punctuation.delimiter
        ";" @punctuation.delimiter
        "," @punctuation.delimiter

        ;; Literals
        (string) @string
        (number) @number
        ```
     2. If it still crashes, try disabling TreeSitter for Mage files temporarily:
        ```lua
        -- In your Neovim config
        vim.treesitter.start = function(bufnr, lang)
          if lang == "mage" then
            return nil  -- Disable TreeSitter for Mage files
          end
          -- Call the original start function for other languages
          return require('vim.treesitter').start(bufnr, lang)
        end
        ```
     3. As a last resort, try a minimal parser that does basic tokenization without complex parsing

   - **Recommended solution**: Use the provided workaround file:
     1. Copy `mage-crash-workaround.lua` to your Neovim config directory
     2. Add `require('mage-crash-workaround')` to your init.lua
     3. This provides a toggle command `:MageToggleTreeSitter` to enable/disable TreeSitter for Mage files
     4. It also sets up a fallback syntax highlighting scheme when TreeSitter is disabled