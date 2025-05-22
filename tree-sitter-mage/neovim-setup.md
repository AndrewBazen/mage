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
          url = "https://github.com/andrewbazen/mage", -- Replace with the actual repo URL
          files = {"tree-sitter-mage/src/parser.c"},
          location = "tree-sitter-mage",
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
       url = "https://github.com/andrewbazen/mage", -- Replace with the actual repo URL
       files = {"tree-sitter-mage/src/parser.c"},
       location = "tree-sitter-mage",
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