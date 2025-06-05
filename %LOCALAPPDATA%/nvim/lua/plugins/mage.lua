return {
  -- Mage language configuration
  {
    "nvim-treesitter/nvim-treesitter",
    opts = function(_, opts)
      -- Add the parser configuration
      local parser_config = require("nvim-treesitter.parsers").get_parser_configs()
      
      -- Configure mage parser if it doesn't exist
      if not parser_config.mage then
        parser_config.mage = {
          install_info = {
            -- Using local path instead of Git repo
            url = vim.fn.expand("C:/Users/andre/projects/mage/tree-sitter-mage"),
            files = {"src/parser.c"},
            -- Tell Tree-sitter this isn't a Git repo
            requires_generate_from_grammar = false,
            generate_requires_npm = false,
          },
          filetype = "mage",
        }
      end
      
      -- Don't try to auto-install it
      if type(opts.ensure_installed) == "table" then
        opts.ensure_installed = vim.tbl_filter(
          function(lang) return lang ~= "mage" end,
          opts.ensure_installed
        )
      end
      
      -- Enable Tree-sitter for syntax highlighting
      if opts.highlight then
        opts.highlight.enable = true
      end
    end,
  },
  
  -- File type detection and syntax
  {
    "LazyVim/LazyVim",
    opts = function(_, opts)
      -- Register the mage filetype
      vim.filetype.add({
        extension = {
          mage = "mage",
        },
      })
      
      -- Link the tree-sitter queries directory
      local run_cmd = function(cmd)
        local result = vim.fn.system(cmd)
        if vim.v.shell_error ~= 0 then
          vim.notify("Command failed: " .. cmd .. "\n" .. result, vim.log.levels.ERROR)
          return false
        end
        return true
      end
      
      -- Auto-compile the parser once LazyVim is loaded
      vim.api.nvim_create_autocmd("User", {
        pattern = "LazyVimStarted",
        callback = function()
          -- Create the parser directory
          local parser_dir = vim.fn.stdpath("data") .. "\\parser"
          vim.fn.mkdir(parser_dir, "p")
          
          -- Compile the parser directly
          local source_dir = "C:/Users/andre/projects/mage/tree-sitter-mage"
          local cmd = ('gcc -o "%s\\mage.dll" -shared "%s\\src\\parser.c" -Os -I"%s\\src"'):format(
            parser_dir, source_dir, source_dir
          )
          
          -- Run in the background to avoid blocking startup
          vim.defer_fn(function()
            if run_cmd(cmd) then
              vim.notify("Mage parser compiled successfully!", vim.log.levels.INFO)
            end
          end, 1000)
        end,
        once = true,
      })
    end,
  },
} 