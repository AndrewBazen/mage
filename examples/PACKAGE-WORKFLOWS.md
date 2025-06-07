# 📦 Mage Package Workflows

A comprehensive package management system for mage that provides declarative dependency management, reproducible builds, and cross-platform compatibility.

## 🚀 Quick Start

### Initialize a New Project

```bash
# Create a new mage project
mage -c "cast package_init('my-project')"

# Navigate to the project
cd my-project
```

### Add Dependencies

```bash
# Add production dependencies
mage -c "cast package_add('git', 'latest', false)"
mage -c "cast package_add('nodejs', '>=18.0.0', false)"

# Add development dependencies  
mage -c "cast package_add('curl', 'latest', true)"
mage -c "cast package_add('docker', 'latest', true)"
```

### Install Dependencies

```bash
# Install production dependencies
mage -c "cast package_install()"

# Install development dependencies
mage -c "cast package_install('--dev')"
```

## 📋 Core Features

- **📋 Declarative dependency management** - Define dependencies in `mage.toml`
- **🔒 Lock files for reproducible builds** - Pin exact versions in `mage.lock`
- **🌍 Cross-platform package mapping** - Automatic platform detection
- **📦 Multiple package sources** - Registry, git, path, URL support
- **🎯 Platform-specific dependencies** - OS-conditional packages
- **⚡ Parallel dependency installation** - Fast concurrent installs
- **🔍 Intelligent package search** - Interactive package selection

## 🛠️ Built-in Functions

### Project Management

```mage
# Initialize new project
cast package_init("project-name")

# Add dependency
cast package_add("package-name", "version", false)  # production
cast package_add("package-name", "version", true)   # development

# Remove dependency
cast package_remove("package-name")

# List dependencies
cast package_list()

# Get package information
cast package_info("package-name")
```

### Installation

```mage
# Install production dependencies
cast package_install()

# Install development dependencies
cast package_install("--dev")

# Check if package is installed
scry package_installed("git") {
    incant "Git is available"
}
```

## 📁 Project Structure

When you initialize a project, mage creates this structure:

```
my-project/
├── mage.toml           # Project manifest
├── mage.lock           # Dependency lock file (auto-generated)
├── scripts/            # Project scripts
│   ├── setup.mage      # Setup script
│   ├── build.mage      # Build script
│   └── test.mage       # Test script
├── lib/                # Library code
├── tests/              # Test files
└── .mage/              # Package cache and metadata
    └── packages/       # Installed packages
```

## 🌍 Cross-Platform Support

### Automatic Package Mapping

The system automatically maps generic package names to platform-specific ones:

| Generic Name | Linux (apt) | macOS (brew) | Windows (winget) |
|--------------|-------------|--------------|------------------|
| `nodejs`     | `nodejs`    | `node`       | `OpenJS.NodeJS`  |
| `git`        | `git`       | `git`        | `Git.Git`        |
| `python3`    | `python3`   | `python@3.11`| `Python.Python.3`|

### Platform Detection

```mage
# Check current platform
conjure platform = platform()
incant "Running on: $platform"

# Platform-specific logic
scry platform() == "windows" {
    cast package_add("windows-terminal", "latest", false)
} morph platform() == "macos" {
    cast package_add("iterm2", "latest", false)
} morph platform() == "linux" {
    cast package_add("gnome-terminal", "latest", false)
}
```

## 📚 Examples

See the `examples/` directory for complete examples:

- `package-workflow-demo.mage` - Basic workflow demonstration
- `web-project-setup.mage` - Web development project setup
- `example-mage.toml` - Complete manifest example

---

**Happy packaging! 📦✨** 