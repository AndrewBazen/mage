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

      -- Point to the manually compiled parser
      local parser_path = vim.fn.stdpath("config") .. "/parsers"
      vim.opt.runtimepath:append(parser_path)
      
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