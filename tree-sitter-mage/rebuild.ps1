# Script to regenerate the parser and query files

Write-Host "Regenerating parser..."
npx tree-sitter generate

Write-Host "Updating parser timestamp..."
(Get-Item .\src\parser.c).LastWriteTime = Get-Date

Write-Host "Done. Please restart Neovim and run :TSInstall mage" 