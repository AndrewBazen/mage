# Mage Language Assets

This directory contains assets for the Mage language project, including icons, images, and other branding materials.

## 🎨 Icon Usage

### **mage64.png**
The primary Mage language icon (64x64 pixels) featuring a magical wizard hat design.

**Used in:**
- VSCode extension file association (via separate extension repository)
- Documentation website favicon and branding
- GitHub repository social preview
- Release assets and packaging

### **Icon Locations**

1. **Source**: `assets/icons/mage64.png`
2. **VSCode Extension**: Managed in separate repository that pulls from this repo
3. **Documentation Site**: `site/assets/icons/mage64.png` (copied by CI)

### **Automated Integration**

The icon is automatically used by GitHub Actions:

- **CI Workflow**: Tests core mage functionality
- **Documentation Workflow**: Copies icon to website and adds favicon
- **VSCode Extension**: Handled in separate repository with its own workflow
- **Release Workflow**: Includes icon in release assets

### **File Association**

The icon is used for:
- `.mage` script files
- `mage.toml` project manifests  
- `mage.lock` lock files

### **Usage in VSCode**

The VSCode extension is maintained in a separate repository that:
1. Pulls language files from this repository
2. Builds and publishes the extension independently
3. Uses the icon from this repository for file associations

When the Mage VSCode extension is installed:
- `.mage` files will display the wizard hat icon
- The extension itself uses the icon in the marketplace
- File explorer shows the icon for Mage-related files

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
3. GitHub Actions will automatically use the new icon in documentation
4. The VSCode extension repository will pull the updated icon on its next sync

## 🔗 Related Repositories

- **VSCode Extension**: Separate repository that pulls from `editor-support/vscode/`
- **Main Language**: This repository contains the core language and assets

## 🎯 Design Guidelines

The Mage icon should:
- Be recognizable at small sizes (16x16 to 64x64)
- Work well on both light and dark backgrounds
- Represent the magical/wizard theme of the language
- Be professional and clean for development tools

## 📝 Attribution

Icon design and implementation by Andrew Bazen for the Mage language project. 