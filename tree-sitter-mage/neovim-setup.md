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
      parser_config.mage2 = {
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
        table.insert(opts.ensure_installed, "mage2")
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
   parser_config.mage2 = {
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
   :TSInstall mage2
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