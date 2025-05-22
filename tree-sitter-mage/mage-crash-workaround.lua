-- Mage TreeSitter Crash Workaround
-- Add this to your Neovim config to prevent crashes when editing Mage files

-- Option 1: Disable TreeSitter for Mage files
local orig_treesitter_start = vim.treesitter.start
vim.treesitter.start = function(bufnr, lang)
  if lang == "mage" then
    -- Check if we should enable or disable based on a configuration variable
    if vim.g.mage_disable_treesitter then
      return nil  -- Disable TreeSitter for Mage files
    end
  end
  -- Call the original start function for other languages
  return orig_treesitter_start(bufnr, lang)
end

-- Option 2: Create a command to toggle TreeSitter for Mage files
vim.api.nvim_create_user_command("MageToggleTreeSitter", function()
  vim.g.mage_disable_treesitter = not vim.g.mage_disable_treesitter
  local state = vim.g.mage_disable_treesitter and "disabled" or "enabled"
  vim.notify("TreeSitter for Mage files is now " .. state)
  
  -- Refresh the current buffer if it's a Mage file
  local bufnr = vim.api.nvim_get_current_buf()
  local ft = vim.bo[bufnr].filetype
  if ft == "mage" then
    -- Need to reload the buffer to refresh TreeSitter
    local winview = vim.fn.winsaveview()
    vim.cmd("e!")
    vim.fn.winrestview(winview)
  end
end, {})

-- Set default to disabled if crashes are occurring
vim.g.mage_disable_treesitter = true  -- set to false if you want to try with TreeSitter enabled

-- Add a minimal filetype detection that doesn't depend on TreeSitter
vim.filetype.add({
  extension = {
    mage = "mage",
  },
})

-- Set up minimal syntax highlighting as a fallback when TreeSitter is disabled
vim.api.nvim_create_autocmd("FileType", {
  pattern = "mage",
  callback = function()
    if vim.g.mage_disable_treesitter then
      vim.cmd([[
        syntax keyword mageKeyword conjure incant curse evoke enchant cast if else loop
        syntax match mageOperator "[=+\-*/]"
        syntax match mageDelimiter "[(){},;]"
        syntax region mageString start=/"/ end=/"/
        syntax match mageNumber "\<\d\+\>"
        syntax match mageComment "#.*$"
        
        highlight default link mageKeyword Keyword
        highlight default link mageOperator Operator 
        highlight default link mageDelimiter Delimiter
        highlight default link mageString String
        highlight default link mageNumber Number
        highlight default link mageComment Comment
      ]])
    end
  end
}) 