# Install Mage language support for Mason and Neovim
# Focused on providing LSP integration with optional TreeSitter support

# Get Neovim data directory
$nvimDataPath = Join-Path $env:LOCALAPPDATA "nvim-data"
if (-not (Test-Path $nvimDataPath)) {
    $nvimDataPath = Join-Path $env:LOCALAPPDATA "nvim"
}

# Get Neovim config directory
$nvimConfigPath = Join-Path $env:LOCALAPPDATA "nvim"
$nvimLuaPath = Join-Path $nvimConfigPath "lua"

# Get key Mason directory
$masonPackagesPath = Join-Path $nvimDataPath "mason\packages"
$mageLspPath = Join-Path $masonPackagesPath "mage-lsp"

# Create Mason package directory
if (-not (Test-Path $mageLspPath)) {
    New-Item -Path $mageLspPath -ItemType Directory -Force | Out-Null
    Write-Host "Created Mason package directory: $mageLspPath" -ForegroundColor Green
}

# Ask if TreeSitter support should be installed
$installTreeSitter = Read-Host "Do you want to install TreeSitter support? (y/n)"
if ($installTreeSitter -eq "y") {
    # Install TreeSitter parser
    $nvimParserPath = Join-Path $nvimDataPath "site\parser"
    if (-not (Test-Path $nvimParserPath)) {
        New-Item -Path $nvimParserPath -ItemType Directory -Force | Out-Null
    }
    
    # Copy parser library
    $parserSoPath = Join-Path $PSScriptRoot "libtree-sitter-mage.so"
    if (Test-Path $parserSoPath) {
        Copy-Item -Path $parserSoPath -Destination (Join-Path $nvimParserPath "mage.so") -Force
        Write-Host "Copied parser library to $((Join-Path $nvimParserPath "mage.so"))" -ForegroundColor Green
    } else {
        Write-Host "Parser library not found at $parserSoPath" -ForegroundColor Yellow
    }
    
    # Copy query files
    $queriesDir = Join-Path $mageLspPath "queries\mage"
    if (-not (Test-Path $queriesDir)) {
        New-Item -Path $queriesDir -ItemType Directory -Force | Out-Null
    }
    
    $queryFiles = Get-ChildItem -Path (Join-Path $PSScriptRoot "queries") -Filter "*.scm"
    foreach ($file in $queryFiles) {
        Copy-Item -Path $file.FullName -Destination (Join-Path $queriesDir $file.Name) -Force
        Write-Host "Copied query file $($file.Name) to $((Join-Path $queriesDir $file.Name))" -ForegroundColor Green
    }
    
    # Install the query fix
    $fixFile = Join-Path $PSScriptRoot "fix-neovim-queries.lua"
    $targetFixFile = Join-Path $nvimLuaPath "fix-neovim-queries.lua"
    Copy-Item -Path $fixFile -Destination $targetFixFile -Force
    Write-Host "Copied fix-neovim-queries.lua to $targetFixFile" -ForegroundColor Green
}

# Create a minimal package.json for the LSP
$packageJsonContent = @'
{
  "name": "mage-lsp",
  "version": "0.1.0",
  "description": "Language Server for Mage language",
  "main": "server.js",
  "scripts": {
    "start": "node server.js"
  },
  "dependencies": {
    "vscode-languageserver": "^8.0.2",
    "vscode-languageserver-textdocument": "^1.0.8"
  }
}
'@

$packageJsonPath = Join-Path $mageLspPath "package.json"
Set-Content -Path $packageJsonPath -Value $packageJsonContent
Write-Host "Created package.json at $packageJsonPath" -ForegroundColor Green

# Create a basic server.js file
$serverJsContent = @'
const {
  createConnection,
  TextDocuments,
  ProposedFeatures,
  CompletionItemKind
} = require('vscode-languageserver');

const { TextDocument } = require('vscode-languageserver-textdocument');

// Create a connection for the server
const connection = createConnection(ProposedFeatures.all);

// Create a text document manager
const documents = new TextDocuments(TextDocument);

// Basic keywords and completions for Mage
const mageKeywords = [
  { label: 'conjure', kind: CompletionItemKind.Keyword },
  { label: 'incant', kind: CompletionItemKind.Keyword },
  { label: 'curse', kind: CompletionItemKind.Keyword },
  { label: 'evoke', kind: CompletionItemKind.Keyword },
  { label: 'enchant', kind: CompletionItemKind.Keyword },
  { label: 'cast', kind: CompletionItemKind.Keyword },
  { label: 'if', kind: CompletionItemKind.Keyword },
  { label: 'else', kind: CompletionItemKind.Keyword },
  { label: 'loop', kind: CompletionItemKind.Keyword },
];

// Handle completion requests
connection.onCompletion(() => {
  return mageKeywords;
});

// Make the documents manager listen to connection events
documents.listen(connection);

// Start the server
connection.listen();
'@

$serverJsPath = Join-Path $mageLspPath "server.js"
Set-Content -Path $serverJsPath -Value $serverJsContent
Write-Host "Created server.js at $serverJsPath" -ForegroundColor Green

# Create a config for Mason
$configJsonContent = @'
{
  "name": "mage-lsp",
  "languages": ["mage"],
  "commands": {
    "start": ["node", "server.js"]
  }
}
'@

$configJsonPath = Join-Path $mageLspPath "mason-registry.json"
Set-Content -Path $configJsonPath -Value $configJsonContent
Write-Host "Created mason-registry.json at $configJsonPath" -ForegroundColor Green

# Create an LSP config file example
$lspConfigContent = @'
-- Configuration for Mage Language Server in Neovim
-- Add this to your init.lua or lspconfig setup

local nvim_lsp = require('lspconfig')

-- Register the mage language server
nvim_lsp.mage_lsp = {
  default_config = {
    cmd = {'node', vim.fn.stdpath('data') .. '/mason/packages/mage-lsp/server.js'},
    filetypes = {'mage'},
    root_dir = function(fname)
      return nvim_lsp.util.find_git_ancestor(fname) or vim.fn.getcwd()
    end,
    settings = {}
  }
}

-- Setup the language server
nvim_lsp.mage_lsp.setup {
  capabilities = require('cmp_nvim_lsp').default_capabilities(),
  on_attach = function(client, bufnr)
    -- Add key mappings here
    vim.api.nvim_buf_set_keymap(bufnr, 'n', 'gd', '<cmd>lua vim.lsp.buf.definition()<CR>', {noremap = true})
    vim.api.nvim_buf_set_keymap(bufnr, 'n', 'K', '<cmd>lua vim.lsp.buf.hover()<CR>', {noremap = true})
    -- Add more mappings as needed
  end
}

-- Add filetype detection
vim.filetype.add({
  extension = {
    mage = "mage",
  },
})
'@

$lspConfigPath = Join-Path $nvimLuaPath "mage-lsp-config.lua"
Set-Content -Path $lspConfigPath -Value $lspConfigContent
Write-Host "Created LSP config example at $lspConfigPath" -ForegroundColor Green

# Print instructions
Write-Host "`nInstallation complete." -ForegroundColor Cyan
Write-Host "The Mage language server is now installed in Mason." -ForegroundColor Cyan

Write-Host "`nTo use with LSP, add this to your Neovim init.lua:" -ForegroundColor Cyan
Write-Host "    require('mage-lsp-config')" -ForegroundColor Yellow

if ($installTreeSitter -eq "y") {
    Write-Host "`nTreeSitter support was installed. Add this to your init.lua if you encounter issues:" -ForegroundColor Cyan
    Write-Host "    require('fix-neovim-queries')" -ForegroundColor Yellow
} else {
    Write-Host "`nTreeSitter support was NOT installed. Your setup will use basic syntax highlighting." -ForegroundColor Cyan
} 