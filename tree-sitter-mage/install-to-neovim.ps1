# Install Tree-sitter Mage parser to Neovim
# This script helps install the Mage parser and query files to the correct Neovim locations

# Get Neovim data directory
$nvimDataPath = Join-Path $env:LOCALAPPDATA "nvim-data"
if (-not (Test-Path $nvimDataPath)) {
    $nvimDataPath = Join-Path $env:LOCALAPPDATA "nvim"
}

# Find nvim-treesitter installation
$treesitterDir = ""
$possiblePaths = @(
    (Join-Path $nvimDataPath "site\pack\packer\start\nvim-treesitter"),
    (Join-Path $nvimDataPath "lazy\nvim-treesitter"),
    (Join-Path $nvimDataPath "site\pack\*\start\nvim-treesitter")
)

foreach ($path in $possiblePaths) {
    $resolved = Resolve-Path $path -ErrorAction SilentlyContinue
    if ($resolved) {
        $treesitterDir = $resolved.Path
        break
    }
}

if (-not $treesitterDir) {
    Write-Host "Could not find nvim-treesitter installation. Please specify the path manually." -ForegroundColor Red
    exit 1
}

Write-Host "Found nvim-treesitter at: $treesitterDir" -ForegroundColor Green

# Create required directories
$parserDir = Join-Path $treesitterDir "parser"
$queryDir = Join-Path $treesitterDir "queries\mage"

if (-not (Test-Path $parserDir)) {
    New-Item -Path $parserDir -ItemType Directory -Force | Out-Null
}
if (-not (Test-Path $queryDir)) {
    New-Item -Path $queryDir -ItemType Directory -Force | Out-Null
}

# Copy parser library
$parserSoPath = Join-Path $PSScriptRoot "libtree-sitter-mage.so"
if (Test-Path $parserSoPath) {
    Copy-Item -Path $parserSoPath -Destination (Join-Path $parserDir "mage.so") -Force
    Write-Host "Copied parser library to $((Join-Path $parserDir "mage.so"))" -ForegroundColor Green
} else {
    Write-Host "Parser library not found at $parserSoPath" -ForegroundColor Yellow
}

# Copy query files
$queryFiles = Get-ChildItem -Path (Join-Path $PSScriptRoot "queries") -Filter "*.scm"
foreach ($file in $queryFiles) {
    Copy-Item -Path $file.FullName -Destination (Join-Path $queryDir $file.Name) -Force
    Write-Host "Copied query file $($file.Name) to $((Join-Path $queryDir $file.Name))" -ForegroundColor Green
}

Write-Host "`nInstallation complete. Please restart Neovim for changes to take effect." -ForegroundColor Cyan
Write-Host "You may need to run :TSInstall mage or add mage to your configured parsers in init.lua" -ForegroundColor Cyan 