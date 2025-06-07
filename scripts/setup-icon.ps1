# Setup Mage Icon Script
# This script helps you properly place your mage64.png icon file

Write-Host "🔮 Mage Icon Setup Script" -ForegroundColor Magenta
Write-Host "=========================" -ForegroundColor Magenta
Write-Host ""

# Check if assets/icons directory exists
if (-not (Test-Path "assets\icons")) {
    Write-Host "Creating assets\icons directory..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Path "assets\icons" -Force | Out-Null
}

# Check if icon file exists and is valid
$iconPath = "assets\icons\mage64.png"
if (Test-Path $iconPath) {
    $iconSize = (Get-Item $iconPath).Length
    if ($iconSize -lt 1000) {
        Write-Host "WARNING: Current icon file appears to be a placeholder ($iconSize bytes)" -ForegroundColor Yellow
        Write-Host "         Please replace it with your actual PNG icon file." -ForegroundColor Yellow
    } else {
        Write-Host "SUCCESS: Icon file exists and looks valid ($iconSize bytes)" -ForegroundColor Green
    }
} else {
    Write-Host "❌ Icon file not found at: $iconPath" -ForegroundColor Red
}

Write-Host ""
Write-Host "📋 Instructions:" -ForegroundColor Cyan
Write-Host "1. Place your mage64.png icon file in: assets\icons\mage64.png" -ForegroundColor White
Write-Host "2. Ensure it's a valid PNG file (64x64 pixels recommended)" -ForegroundColor White
Write-Host "3. Commit and push - GitHub Actions will handle the rest!" -ForegroundColor White
Write-Host ""

# Check for other potential issues
Write-Host "🔍 Checking for other setup issues..." -ForegroundColor Cyan

# Check if examples exist
if (-not (Test-Path "examples")) {
    Write-Host "⚠️  Examples directory not found" -ForegroundColor Yellow
} else {
    $mageFiles = Get-ChildItem -Path "examples" -Filter "*.mage" -Recurse
    Write-Host "✅ Found $($mageFiles.Count) .mage example files" -ForegroundColor Green
}

# Check if required example files exist
$requiredExamples = @(
    "examples\package-workflow-demo.mage",
    "examples\web-project-setup.mage", 
    "examples\example-mage.toml",
    "examples\PACKAGE-WORKFLOWS.md"
)

foreach ($example in $requiredExamples) {
    if (Test-Path $example) {
        Write-Host "✅ $example exists" -ForegroundColor Green
    } else {
        Write-Host "❌ $example missing (GitHub Actions may fail)" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "🚀 After placing your icon:" -ForegroundColor Cyan
Write-Host "   git add assets\icons\mage64.png" -ForegroundColor White
Write-Host "   git commit -m `"Add mage icon`"" -ForegroundColor White
Write-Host "   git push" -ForegroundColor White
Write-Host ""
Write-Host "The icon will automatically be used in:" -ForegroundColor Green
Write-Host "• Documentation website favicon and branding" -ForegroundColor White
Write-Host "• GitHub social preview" -ForegroundColor White  
Write-Host "• VSCode extension (via separate repo)" -ForegroundColor White
Write-Host "• Release assets" -ForegroundColor White 