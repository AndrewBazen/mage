# Mage Language Assets

This directory contains assets for the Mage language project, including icons, images, and other branding materials.

## 🎨 Icon Usage

### **mage64.png**
The primary Mage language icon (64x64 pixels) featuring a magical wizard hat design.

**Used in:**
- VSCode extension file association
- Documentation website favicon and branding
- GitHub repository social preview
- Release assets and packaging

### **Icon Locations**

1. **Source**: `assets/icons/mage64.png`
2. **VSCode Extension**: `editor-support/vscode/icons/mage64.png` (copied by CI)
3. **Documentation Site**: `site/assets/icons/mage64.png` (copied by CI)

### **Automated Integration**

The icon is automatically copied to appropriate locations by GitHub Actions:

- **CI Workflow**: Tests VSCode extension with icon
- **Documentation Workflow**: Copies icon to website and adds favicon
- **VSCode Extension Workflow**: Packages extension with icon
- **Release Workflow**: Includes icon in release assets

### **File Association**

The icon is used for:
- `.mage` script files
- `mage.toml` project manifests  
- `mage.lock` lock files

### **Usage in VSCode**

When the Mage VSCode extension is installed:
1. `.mage` files will display the wizard hat icon
2. The extension itself uses the icon in the marketplace
3. File explorer shows the icon for Mage-related files

### **Documentation Website**

The icon appears as:
- Browser favicon (tab icon)
- Site header branding
- Social media preview image
- OpenGraph meta image

### **Customization**

To use a different icon:
1. Replace `assets/icons/mage64.png` with your icon (64x64 PNG recommended)
2. Ensure the file maintains the same name and format
3. GitHub Actions will automatically use the new icon in all locations

## 🎯 Design Guidelines

The Mage icon should:
- Be recognizable at small sizes (16x16 to 64x64)
- Work well on both light and dark backgrounds
- Represent the magical/wizard theme of the language
- Be professional and clean for development tools

## 📝 Attribution

Icon design and implementation by Andrew Bazen for the Mage language project. 