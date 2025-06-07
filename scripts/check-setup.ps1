# Mage Project Setup Checker
Write-Host "Mage Icon Setup Checker" -ForegroundColor Magenta
Write-Host "======================" -ForegroundColor Magenta
Write-Host ""

# Check if icon file exists and is valid
$iconPath = "assets\icons\mage64.png"
if (Test-Path $iconPath) {
    $iconSize = (Get-Item $iconPath).Length
    if ($iconSize -lt 1000) {
        Write-Host "WARNING: Icon file appears to be placeholder text ($iconSize bytes)" -ForegroundColor Yellow
        Write-Host "         Replace with actual PNG icon file" -ForegroundColor Yellow
    } else {
        Write-Host "SUCCESS: Icon file exists ($iconSize bytes)" -ForegroundColor Green
    }
} else {
    Write-Host "ERROR: Icon file not found at $iconPath" -ForegroundColor Red
}

Write-Host ""
Write-Host "Checking required example files..." -ForegroundColor Cyan

# Check for required files that GitHub Actions expect
$requiredFiles = @(
    "examples\package-workflow-demo.mage",
    "examples\web-project-setup.mage", 
    "examples\example-mage.toml",
    "examples\PACKAGE-WORKFLOWS.md",
    "examples\system-info.mage",
    "examples\comprehensive-demo.mage"
)

foreach ($file in $requiredFiles) {
    if (Test-Path $file) {
        Write-Host "SUCCESS: $file exists" -ForegroundColor Green
    } else {
        Write-Host "ERROR: $file missing (GitHub Actions will fail)" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "TO FIX GITHUB ACTIONS ERRORS:" -ForegroundColor Yellow
Write-Host "1. Replace assets\icons\mage64.png with your actual icon" -ForegroundColor White
Write-Host "2. Ensure all required example files exist" -ForegroundColor White
Write-Host "3. Commit and push changes" -ForegroundColor White 