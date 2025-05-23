-- Fix for TreeSitter query parsing errors with Mage language
-- Add this to your init.lua or source it using: require('fix-neovim-queries')

local M = {}

-- Override problematic queries for Mage language
M.setup = function()
  -- Only proceed if TreeSitter is available
  if not pcall(require, 'nvim-treesitter') then
    return
  end

  -- Create a safe highlights query
  local highlights_query = [[
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
  
  ;; Comments
  (comment) @comment
  ]]

  -- Override the query loading function only for Mage language
  local orig_get_query = vim.treesitter.query.get_query
  vim.treesitter.query.get_query = function(lang, query_name)
    if lang == "mage" and query_name == "highlights" then
      -- Intercept highlights query for Mage language and return our safe version
      return vim.treesitter.query.parse_query(lang, highlights_query)
    end
    -- For all other languages/queries, use the original function
    return orig_get_query(lang, query_name)
  end

  -- Special handling for Neo-tree to prevent crashes
  vim.api.nvim_create_autocmd("FileType", {
    pattern = "mage",
    callback = function()
      -- Try to disable conceal which can cause problems with Neo-tree
      vim.wo.conceallevel = 0
      vim.wo.concealcursor = ""
      
      -- Prevent highlighter errors by using pcall
      local orig_highlighter_new = vim.treesitter.highlighter.new
      vim.treesitter.highlighter.new = function(bufnr, opts)
        if vim.bo[bufnr].filetype == "mage" then
          local success, result = pcall(orig_highlighter_new, bufnr, opts)
          if not success then
            vim.notify("TreeSitter highlighting disabled for Mage file due to errors", vim.log.levels.WARN)
            return nil
          end
          return result
        else
          return orig_highlighter_new(bufnr, opts)
        end
      end
    end
  })
  
  -- Add a command to toggle TreeSitter for Mage files
  vim.api.nvim_create_user_command("MageToggleTreeSitter", function()
    vim.g.mage_disable_treesitter = not vim.g.mage_disable_treesitter
    local state = vim.g.mage_disable_treesitter and "disabled" or "enabled"
    vim.notify("TreeSitter for Mage files is now " .. state)
    
    -- Reload the current buffer to apply changes
    local bufnr = vim.api.nvim_get_current_buf()
    if vim.bo[bufnr].filetype == "mage" then
      local winview = vim.fn.winsaveview()
      vim.cmd("e!")
      vim.fn.winrestview(winview)
    end
  end, {})
  
  -- Start with TreeSitter disabled for Mage by default (safer)
  vim.g.mage_disable_treesitter = true
end

-- Call setup function
M.setup()

return M 