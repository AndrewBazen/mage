use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub enum BuiltinValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<String>),
    None,
}

impl std::fmt::Display for BuiltinValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BuiltinValue::String(s) => write!(f, "{}", s),
            BuiltinValue::Number(n) => write!(f, "{}", n),
            BuiltinValue::Boolean(b) => write!(f, "{}", b),
            BuiltinValue::Array(arr) => write!(f, "[{}]", arr.join(", ")),
            BuiltinValue::None => write!(f, ""),
        }
    }
}

pub fn call_builtin(name: &str, args: Vec<String>) -> Result<BuiltinValue, String> {
    match name {
        // System Information
        "platform" => Ok(BuiltinValue::String(detect_platform())),
        "architecture" => Ok(BuiltinValue::String(detect_architecture())),
        "home_directory" => Ok(BuiltinValue::String(get_home_directory())),
        "current_directory" => Ok(BuiltinValue::String(get_current_directory())),
        
        // File System Operations
        "file_exists" => {
            if args.len() != 1 {
                return Err("file_exists() requires exactly 1 argument: path".to_string());
            }
            Ok(BuiltinValue::Boolean(Path::new(&args[0]).exists()))
        },
        "directory_exists" => {
            if args.len() != 1 {
                return Err("directory_exists() requires exactly 1 argument: path".to_string());
            }
            Ok(BuiltinValue::Boolean(Path::new(&args[0]).is_dir()))
        },
        "ensure_directory" => {
            if args.len() != 1 {
                return Err("ensure_directory() requires exactly 1 argument: path".to_string());
            }
            ensure_directory(&args[0])
        },
        "copy_file" => {
            if args.len() != 2 {
                return Err("copy_file() requires exactly 2 arguments: source, destination".to_string());
            }
            copy_file(&args[0], &args[1])
        },
        "write_file" => {
            if args.len() != 2 {
                return Err("write_file() requires exactly 2 arguments: path, content".to_string());
            }
            write_file(&args[0], &args[1])
        },
        "remove_file" => {
            if args.len() != 1 {
                return Err("remove_file() requires exactly 1 argument: path".to_string());
            }
            remove_file(&args[0])
        },
        "remove_directory" => {
            if args.len() != 1 {
                return Err("remove_directory() requires exactly 1 argument: path".to_string());
            }
            remove_directory(&args[0])
        },
        "symlink" => {
            if args.len() != 2 {
                return Err("symlink() requires exactly 2 arguments: source, target".to_string());
            }
            create_symlink(&args[0], &args[1])
        },
        "make_executable" => {
            if args.len() != 1 {
                return Err("make_executable() requires exactly 1 argument: path".to_string());
            }
            make_executable(&args[0])
        },
        "is_executable" => {
            if args.len() != 1 {
                return Err("is_executable() requires exactly 1 argument: path".to_string());
            }
            Ok(BuiltinValue::Boolean(is_executable(&args[0])))
        },
        
        // Package Management
        "detect_package_managers" => Ok(BuiltinValue::Array(detect_package_managers())),
        "get_primary_package_manager" => Ok(BuiltinValue::String(get_primary_package_manager())),
        "package_manager_available" => {
            if args.len() != 1 {
                return Err("package_manager_available() requires exactly 1 argument: manager_name".to_string());
            }
            Ok(BuiltinValue::Boolean(package_manager_available(&args[0])))
        },
        "install_package" => {
            if args.len() != 1 {
                return Err("install_package() requires exactly 1 argument: package_name".to_string());
            }
            install_package(&args[0])
        },
        "package_installed" => {
            if args.len() != 1 {
                return Err("package_installed() requires exactly 1 argument: package_name".to_string());
            }
            Ok(BuiltinValue::Boolean(package_installed(&args[0])))
        },
        
        // Package Project Management
        "package_init" => {
            if args.len() != 1 {
                return Err("package_init() requires exactly 1 argument: project_name".to_string());
            }
            package_init(&args[0])
        },
        "package_add" => {
            if args.len() < 2 || args.len() > 3 {
                return Err("package_add() requires 2-3 arguments: package_name, version, [--dev]".to_string());
            }
            let is_dev = args.len() == 3 && args[2] == "--dev";
            package_add(&args[0], &args[1], is_dev)
        },
        "package_remove" => {
            if args.len() != 1 {
                return Err("package_remove() requires exactly 1 argument: package_name".to_string());
            }
            package_remove(&args[0])
        },
        "package_install" => {
            let dev = args.get(0).map(|s| s == "--dev").unwrap_or(false);
            package_install_deps(dev)
        },
        "package_list" => {
            Ok(BuiltinValue::String(package_list()))
        },
        "package_info" => {
            if args.len() != 1 {
                return Err("package_info() requires exactly 1 argument: package_name".to_string());
            }
            package_info(&args[0])
        },
        
        // Network Operations  
        "download" => {
            if args.len() != 2 {
                return Err("download() requires exactly 2 arguments: url, path".to_string());
            }
            download_file(&args[0], &args[1])
        },
        "search_package" => {
            if args.len() != 1 {
                return Err("search_package() requires exactly 1 argument: package_name".to_string());
            }
            let pm = get_primary_package_manager();
            match search_for_package(&args[0], &pm) {
                Some(found_name) => Ok(BuiltinValue::String(found_name)),
                None => Ok(BuiltinValue::String(format!("Package '{}' not found", args[0]))),
            }
        },
        "list_packages" => {
            if args.len() != 1 {
                return Err("list_packages() requires exactly 1 argument: package_name".to_string());
            }
            let pm = get_primary_package_manager();
            let matches = search_for_packages(&args[0], &pm);
            
            if matches.is_empty() {
                Ok(BuiltinValue::String(format!("No packages found matching '{}'", args[0])))
            } else {
                let mut result = format!("Found {} packages matching '{}':\n", matches.len(), args[0]);
                for (i, (name, id)) in matches.iter().enumerate() {
                    result.push_str(&format!("  {}: {} ({})\n", i + 1, name, id));
                }
                Ok(BuiltinValue::String(result))
            }
        },
        
        // Environment
        "env_var" => {
            if args.is_empty() || args.len() > 2 {
                return Err("env_var() requires 1 or 2 arguments: name, [default]".to_string());
            }
            let default = if args.len() == 2 { Some(&args[1]) } else { None };
            Ok(BuiltinValue::String(get_env_var(&args[0], default.map(|s| s.as_str()))))
        },
        
        _ => Err(format!("Unknown builtin function: {}", name)),
    }
}

pub fn is_builtin(name: &str) -> bool {
    matches!(name, 
        "platform" | "architecture" | "home_directory" | "current_directory" |
        "file_exists" | "directory_exists" | "ensure_directory" | "copy_file" | 
        "symlink" | "make_executable" | "is_executable" |
        "write_file" | "remove_file" | "remove_directory" |
        "detect_package_managers" | "get_primary_package_manager" | "package_manager_available" |
        "install_package" | "package_installed" | "search_package" | "list_packages" |
        "package_init" | "package_add" | "package_remove" | "package_install" | "package_list" | "package_info" |
        "download" | "env_var"
    )
}

// System Information Functions
fn detect_platform() -> String {
    std::env::consts::OS.to_string()
}

fn detect_architecture() -> String {
    std::env::consts::ARCH.to_string()
}

fn get_home_directory() -> String {
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .to_string_lossy()
        .to_string()
}

fn get_current_directory() -> String {
    std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .to_string_lossy()
        .to_string()
}

// File System Functions
fn ensure_directory(path: &str) -> Result<BuiltinValue, String> {
    match fs::create_dir_all(path) {
        Ok(()) => Ok(BuiltinValue::Boolean(true)),
        Err(e) => Err(format!("Failed to create directory '{}': {}", path, e)),
    }
}

fn copy_file(source: &str, dest: &str) -> Result<BuiltinValue, String> {
    match fs::copy(source, dest) {
        Ok(_) => Ok(BuiltinValue::Boolean(true)),
        Err(e) => Err(format!("Failed to copy '{}' to '{}': {}", source, dest, e)),
    }
}

fn create_symlink(source: &str, target: &str) -> Result<BuiltinValue, String> {
    #[cfg(target_family = "windows")]
    {
        use std::os::windows::fs;
        match fs::symlink_file(source, target) {
            Ok(()) => Ok(BuiltinValue::Boolean(true)),
            Err(e) => Err(format!("Failed to create symlink from '{}' to '{}': {}", source, target, e)),
        }
    }
    
    #[cfg(not(target_family = "windows"))]
    {
        use std::os::unix::fs;
        match fs::symlink(source, target) {
            Ok(()) => Ok(BuiltinValue::Boolean(true)),
            Err(e) => Err(format!("Failed to create symlink from '{}' to '{}': {}", source, target, e)),
        }
    }
}

fn write_file(path: &str, content: &str) -> Result<BuiltinValue, String> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(path).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directory: {}", e))?;
        }
    }
    
    std::fs::write(path, content)
        .map(|_| BuiltinValue::Boolean(true))
        .map_err(|e| format!("Failed to write file '{}': {}", path, e))
}

fn remove_file(path: &str) -> Result<BuiltinValue, String> {
    if std::path::Path::new(path).exists() {
        std::fs::remove_file(path)
            .map(|_| BuiltinValue::Boolean(true))
            .map_err(|e| format!("Failed to remove file '{}': {}", path, e))
    } else {
        Ok(BuiltinValue::Boolean(false))
    }
}

fn remove_directory(path: &str) -> Result<BuiltinValue, String> {
    if std::path::Path::new(path).exists() {
        std::fs::remove_dir_all(path)
            .map(|_| BuiltinValue::Boolean(true))
            .map_err(|e| format!("Failed to remove directory '{}': {}", path, e))
    } else {
        Ok(BuiltinValue::Boolean(false))
    }
}

fn make_executable(path: &str) -> Result<BuiltinValue, String> {
    #[cfg(target_family = "windows")]
    {
        // On Windows, executability is determined by file extension
        // Just check if the file exists
        if Path::new(path).exists() {
            Ok(BuiltinValue::Boolean(true))
        } else {
            Err(format!("File '{}' does not exist", path))
        }
    }
    
    #[cfg(not(target_family = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        let path = Path::new(path);
        if !path.exists() {
            return Err(format!("File '{}' does not exist", path.display()));
        }
        
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(e) => return Err(format!("Failed to get metadata for '{}': {}", path.display(), e)),
        };
        
        let mut perms = metadata.permissions();
        perms.set_mode(perms.mode() | 0o111); // Add execute permission
        
        match fs::set_permissions(path, perms) {
            Ok(()) => Ok(BuiltinValue::Boolean(true)),
            Err(e) => Err(format!("Failed to make '{}' executable: {}", path.display(), e)),
        }
    }
}

fn is_executable(path: &str) -> bool {
    #[cfg(target_family = "windows")]
    {
        // On Windows, check if it's an executable extension
        let path = Path::new(path);
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(ext.as_str(), "exe" | "bat" | "cmd" | "ps1")
        } else {
            false
        }
    }
    
    #[cfg(not(target_family = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(path) {
            metadata.permissions().mode() & 0o111 != 0
        } else {
            false
        }
    }
}

// Package Management Functions
fn detect_package_managers() -> Vec<String> {
    let mut managers = Vec::new();
    let candidates = vec![
        "apt", "yum", "dnf", "pacman", "zypper", "emerge", // Linux
        "brew", "port", // macOS
        "winget", "choco", "scoop", // Windows
        "pip", "npm", "cargo", "gem", // Language-specific
    ];
    
    for manager in candidates {
        if which::which(manager).is_ok() {
            managers.push(manager.to_string());
        }
    }
    
    managers
}

fn get_primary_package_manager() -> String {
    let platform = detect_platform();
    let available = detect_package_managers();
    
    let priorities = match platform.as_str() {
        "linux" => vec!["apt", "yum", "dnf", "pacman", "zypper", "emerge"],
        "macos" => vec!["brew", "port"],
        "windows" => vec!["winget", "choco", "scoop"],
        _ => vec![],
    };
    
    for pm in priorities {
        if available.contains(&pm.to_string()) {
            return pm.to_string();
        }
    }
    
    available.first().cloned().unwrap_or_else(|| "none".to_string())
}

fn package_manager_available(manager: &str) -> bool {
    which::which(manager).is_ok()
}

fn install_package(package: &str) -> Result<BuiltinValue, String> {
    let pm = get_primary_package_manager();
    if pm == "none" {
        return Err("No package manager available".to_string());
    }
    
    // Search for multiple packages and let user choose
    let package_name = match select_package_interactively(package, &pm) {
        Some(selected) => selected,
        None => {
            // Fallback to single search, then mapping
            search_for_package(package, &pm)
                .unwrap_or_else(|| map_package_name(package, &pm))
        }
    };
    
    let install_cmd = match pm.as_str() {
        "apt" => format!("apt install -y {}", package_name),
        "yum" => format!("yum install -y {}", package_name),
        "dnf" => format!("dnf install -y {}", package_name),
        "pacman" => format!("pacman -S --noconfirm {}", package_name),
        "brew" => format!("brew install {}", package_name),
        "winget" => format!("winget install {}", package_name),
        "choco" => format!("choco install {} -y", package_name),
        "scoop" => format!("scoop install {}", package_name),
        _ => return Err(format!("Unsupported package manager: {}", pm)),
    };
    
    println!("📦 Installing {} using {}...", package_name, pm);
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", &install_cmd]).output()
    } else {
        Command::new("sh").args(&["-c", &install_cmd]).output()
    };
    
    match output {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Successfully installed {}", package_name);
                Ok(BuiltinValue::Boolean(true))
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(format!("Failed to install {}: {}", package_name, stderr))
            }
        }
        Err(e) => Err(format!("Failed to execute install command: {}", e)),
    }
}

fn package_installed(package: &str) -> bool {
    let pm = get_primary_package_manager();
    let package_name = map_package_name(package, &pm);
    
    let check_cmd = match pm.as_str() {
        "apt" => format!("dpkg -l | grep -q '^ii.*{}'", package_name),
        "yum" | "dnf" => format!("rpm -q {}", package_name),
        "pacman" => format!("pacman -Q {}", package_name),
        "brew" => format!("brew list | grep -q {}", package_name),
        "winget" => format!("winget list | findstr {}", package_name),
        "choco" => format!("choco list --local-only | findstr {}", package_name),
        _ => return false,
    };
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", &check_cmd]).output()
    } else {
        Command::new("sh").args(&["-c", &check_cmd]).output()
    };
    
    output.map(|o| o.status.success()).unwrap_or(false)
}

fn search_for_packages(package: &str, manager: &str) -> Vec<(String, String)> {
    // Returns Vec of (name, id) tuples
    let search_cmd = match manager {
        "apt" => format!("apt search '{}' 2>/dev/null", package),
        "yum" => format!("yum search {} 2>/dev/null", package),
        "dnf" => format!("dnf search {} 2>/dev/null", package), 
        "pacman" => format!("pacman -Ss '{}' 2>/dev/null", package),
        "brew" => format!("brew search '{}' 2>/dev/null", package),
        "winget" => format!("winget search {} 2>nul", package),
        "choco" => format!("choco search {} 2>nul", package),
        "scoop" => format!("scoop search {} 2>nul", package),
        _ => return Vec::new(),
    };
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", &search_cmd]).output()
    } else {
        Command::new("sh").args(&["-c", &search_cmd]).output()
    };
    
    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Parse search results based on package manager
            match manager {
                "apt" => parse_apt_search_multiple(&stdout, package),
                "brew" => parse_brew_search_multiple(&stdout, package),
                "winget" => parse_winget_search_multiple(&stdout, package),
                "choco" => parse_choco_search_multiple(&stdout, package),
                "pacman" => parse_pacman_search_multiple(&stdout, package),
                _ => Vec::new(),
            }
        }
        _ => Vec::new(),
    }
}

fn select_package_interactively(package: &str, manager: &str) -> Option<String> {
    let matches = search_for_packages(package, manager);
    
    if matches.is_empty() {
        return None;
    }
    
    if matches.len() == 1 {
        // Only one match, use it automatically
        return Some(matches[0].1.clone());
    }
    
    // Multiple matches - show selection menu
    println!("🔍 Found {} packages matching '{}':", matches.len(), package);
    println!();
    
    for (i, (name, id)) in matches.iter().enumerate() {
        println!("  {}: {} ({})", i + 1, name, id);
    }
    
    println!();
    print!("Choose a package (1-{}, or 0 to cancel): ", matches.len());
    
    // Flush stdout to ensure prompt appears
    use std::io::{self, Write};
    io::stdout().flush().ok();
    
    // Read user input
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let choice = input.trim().parse::<usize>().unwrap_or(0);
            if choice > 0 && choice <= matches.len() {
                Some(matches[choice - 1].1.clone())
            } else {
                println!("❌ Installation cancelled.");
                None
            }
        }
        Err(_) => {
            println!("❌ Failed to read input. Installation cancelled.");
            None
        }
    }
}

fn search_for_package(package: &str, manager: &str) -> Option<String> {
    let search_cmd = match manager {
        "apt" => format!("apt search '^{}$' 2>/dev/null", package),
        "yum" => format!("yum search {} 2>/dev/null", package),
        "dnf" => format!("dnf search {} 2>/dev/null", package), 
        "pacman" => format!("pacman -Ss '^{}$' 2>/dev/null", package),
        "brew" => format!("brew search '^{}$' 2>/dev/null", package),
        "winget" => format!("winget search {} 2>nul", package),
        "choco" => format!("choco search {} --exact 2>nul", package),
        "scoop" => format!("scoop search {} 2>nul", package),
        _ => return None,
    };
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", &search_cmd]).output()
    } else {
        Command::new("sh").args(&["-c", &search_cmd]).output()
    };
    
    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Parse search results based on package manager
            match manager {
                "apt" => parse_apt_search(&stdout, package),
                "brew" => parse_brew_search(&stdout, package),
                "winget" => parse_winget_search(&stdout, package),
                "choco" => parse_choco_search(&stdout, package),
                "pacman" => parse_pacman_search(&stdout, package),
                _ => {
                    // For other managers, if search succeeded and has output, assume found
                    if !stdout.trim().is_empty() {
                        Some(package.to_string())
                    } else {
                        None
                    }
                }
            }
        }
        _ => None,
    }
}

// Helper functions to parse search output
fn parse_apt_search(output: &str, package: &str) -> Option<String> {
    // apt search shows: "package-name/repo version [installed]"
    for line in output.lines() {
        if line.contains(package) && !line.starts_with("WARNING") {
            if let Some(pkg_name) = line.split('/').next() {
                return Some(pkg_name.to_string());
            }
        }
    }
    None
}

fn parse_brew_search(output: &str, package: &str) -> Option<String> {
    // brew search shows exact matches first
    for line in output.lines() {
        let line = line.trim();
        if line == package || line.starts_with(&format!("{}@", package)) {
            return Some(line.to_string());
        }
    }
    None
}

fn parse_winget_search(output: &str, package: &str) -> Option<String> {
    // winget search shows: "Name Id Version Available Source"
    let lines: Vec<&str> = output.lines().collect();
    let package_lower = package.to_lowercase();
    
    // Priority 1: Single word exact name match
    for line in &lines {
        if !line.starts_with("Name") && !line.starts_with("-") && !line.trim().is_empty() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0];
                let id = parts[1];
                // Exact single-word match
                if name.to_lowercase() == package_lower {
                    return Some(id.to_string());
                }
            }
        }
    }
    
    // Priority 2: ID contains package name
    for line in &lines {
        if !line.starts_with("Name") && !line.starts_with("-") && !line.trim().is_empty() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let id = parts[1].to_lowercase();
                // Check if ID contains the package name (like vim.vim)
                if id.contains(&package_lower) {
                    return Some(parts[1].to_string());
                }
            }
        }
    }
    
    // Priority 3: Multi-word name starts with package name
    for line in &lines {
        if !line.starts_with("Name") && !line.starts_with("-") && !line.trim().is_empty() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0];
                if name.to_lowercase() == package_lower {
                    return Some(parts[1].to_string());
                }
            }
        }
    }
    
    None
}

fn parse_choco_search(output: &str, package: &str) -> Option<String> {
    // choco search shows: "package-name version [Approved]"
    for line in output.lines() {
        if line.starts_with(package) && line.contains(' ') {
            if let Some(pkg_name) = line.split(' ').next() {
                return Some(pkg_name.to_string());
            }
        }
    }
    None
}

fn parse_pacman_search(output: &str, package: &str) -> Option<String> {
    // pacman -Ss shows: "repo/package-name version"
    for line in output.lines() {
        if line.contains(&format!("/{}", package)) {
            if let Some(pkg_part) = line.split(' ').next() {
                if let Some(pkg_name) = pkg_part.split('/').nth(1) {
                    return Some(pkg_name.to_string());
                }
            }
        }
    }
    None
}

// Multiple result parsing functions
fn parse_winget_search_multiple(output: &str, package: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let package_lower = package.to_lowercase();
    
    for line in output.lines() {
        if !line.starts_with("Name") && !line.starts_with("-") && !line.trim().is_empty() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0];
                let id = parts[1];
                
                // Include if name or ID contains the search term
                if name.to_lowercase().contains(&package_lower) || 
                   id.to_lowercase().contains(&package_lower) {
                    results.push((name.to_string(), id.to_string()));
                }
            }
        }
    }
    
    // Sort by relevance: exact matches first, then contains
    results.sort_by(|a, b| {
        let a_exact = a.0.to_lowercase() == package_lower || a.1.to_lowercase() == package_lower;
        let b_exact = b.0.to_lowercase() == package_lower || b.1.to_lowercase() == package_lower;
        
        match (a_exact, b_exact) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.0.cmp(&b.0),
        }
    });
    
    results.truncate(10); // Limit to 10 results
    results
}

fn parse_apt_search_multiple(output: &str, package: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let package_lower = package.to_lowercase();
    
    for line in output.lines() {
        if !line.starts_with("WARNING") && line.contains('/') {
            if let Some(pkg_name) = line.split('/').next() {
                if pkg_name.to_lowercase().contains(&package_lower) {
                    results.push((pkg_name.to_string(), pkg_name.to_string()));
                }
            }
        }
    }
    
    results.sort();
    results.dedup();
    results.truncate(10);
    results
}

fn parse_brew_search_multiple(output: &str, package: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let package_lower = package.to_lowercase();
    
    for line in output.lines() {
        let line = line.trim();
        if !line.is_empty() && line.to_lowercase().contains(&package_lower) {
            results.push((line.to_string(), line.to_string()));
        }
    }
    
    results.sort();
    results.truncate(10);
    results
}

fn parse_choco_search_multiple(output: &str, package: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let package_lower = package.to_lowercase();
    
    for line in output.lines() {
        if line.contains(' ') && !line.starts_with("Chocolatey") {
            if let Some(pkg_name) = line.split(' ').next() {
                if pkg_name.to_lowercase().contains(&package_lower) {
                    results.push((pkg_name.to_string(), pkg_name.to_string()));
                }
            }
        }
    }
    
    results.sort();
    results.truncate(10);
    results
}

fn parse_pacman_search_multiple(output: &str, package: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let package_lower = package.to_lowercase();
    
    for line in output.lines() {
        if line.contains('/') {
            if let Some(pkg_part) = line.split(' ').next() {
                if let Some(pkg_name) = pkg_part.split('/').nth(1) {
                    if pkg_name.to_lowercase().contains(&package_lower) {
                        results.push((pkg_name.to_string(), pkg_name.to_string()));
                    }
                }
            }
        }
    }
    
    results.sort();
    results.truncate(10);
    results
}

fn map_package_name(package: &str, manager: &str) -> String {
    // Built-in package name mapping
    let mappings = HashMap::from([
        ("nodejs", HashMap::from([
            ("apt", "nodejs"),
            ("yum", "nodejs"),
            ("dnf", "nodejs"),
            ("brew", "node"),
            ("choco", "nodejs"),
            ("winget", "OpenJS.NodeJS"),
        ])),
        ("git", HashMap::from([
            ("apt", "git"),
            ("yum", "git"),
            ("dnf", "git"),
            ("brew", "git"),
            ("choco", "git"),
            ("winget", "Git.Git"),
        ])),
        ("python3", HashMap::from([
            ("apt", "python3"),
            ("yum", "python3"),
            ("dnf", "python3"),
            ("brew", "python@3.11"),
            ("choco", "python3"),
            ("winget", "Python.Python.3"),
        ])),
    ]);
    
    if let Some(pkg_map) = mappings.get(package) {
        if let Some(mapped) = pkg_map.get(manager) {
            return mapped.to_string();
        }
    }
    
    package.to_string()
}

// Network Functions
fn download_file(url: &str, path: &str) -> Result<BuiltinValue, String> {
    // This is a simplified implementation
    // In a real implementation, you'd use a proper HTTP client like reqwest
    let curl_cmd = format!("curl -L '{}' -o '{}'", url, path);
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", &curl_cmd]).output()
    } else {
        Command::new("sh").args(&["-c", &curl_cmd]).output()
    };
    
    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(BuiltinValue::Boolean(true))
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(format!("Failed to download '{}': {}", url, stderr))
            }
        }
        Err(e) => Err(format!("Failed to execute download command: {}", e)),
    }
}

// Environment Functions
fn get_env_var(name: &str, default: Option<&str>) -> String {
    std::env::var(name).unwrap_or_else(|_| {
        default.unwrap_or("").to_string()
    })
}

// Package Project Management Functions
fn package_init(name: &str) -> Result<BuiltinValue, String> {
    use std::env;
    let current_dir = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let resolver = crate::package::PackageResolver::new(&current_dir);
    resolver.init_project(name)?;
    
    println!("✅ Initialized mage project: {}", name);
    println!("📁 Created project structure:");
    println!("   mage.toml        - Project manifest");
    println!("   scripts/         - Project scripts");
    println!("   .mage/           - Package cache");
    
    Ok(BuiltinValue::Boolean(true))
}

fn package_add(package: &str, version: &str, is_dev: bool) -> Result<BuiltinValue, String> {
    use std::env;
    let current_dir = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let resolver = crate::package::PackageResolver::new(&current_dir);
    resolver.add_dependency(package, version, is_dev)?;
    
    let dep_type = if is_dev { "dev dependency" } else { "dependency" };
    println!("✅ Added {} {} @ {}", dep_type, package, version);
    
    Ok(BuiltinValue::Boolean(true))
}

fn package_remove(package: &str) -> Result<BuiltinValue, String> {
    use std::env;
    let current_dir = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let resolver = crate::package::PackageResolver::new(&current_dir);
    resolver.remove_dependency(package)?;
    
    println!("✅ Removed dependency: {}", package);
    
    Ok(BuiltinValue::Boolean(true))
}

fn package_install_deps(dev: bool) -> Result<BuiltinValue, String> {
    use std::env;
    let current_dir = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let resolver = crate::package::PackageResolver::new(&current_dir);
    resolver.install_dependencies(dev)?;
    
    println!("✅ Dependencies installed successfully");
    
    Ok(BuiltinValue::Boolean(true))
}

fn package_list() -> String {
    use std::env;
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return "❌ Failed to get current directory".to_string(),
    };
    
    let resolver = crate::package::PackageResolver::new(&current_dir);
    match resolver.read_manifest() {
        Ok(manifest) => {
            let mut result = format!("📦 {} v{}\n", manifest.name, manifest.version);
            
            if !manifest.dependencies.is_empty() {
                result.push_str("\n🔗 Dependencies:\n");
                for (name, dep) in &manifest.dependencies {
                    result.push_str(&format!("  {} @ {}\n", name, dep.version));
                }
            }
            
            if !manifest.dev_dependencies.is_empty() {
                result.push_str("\n🛠️  Dev Dependencies:\n");
                for (name, dep) in &manifest.dev_dependencies {
                    result.push_str(&format!("  {} @ {}\n", name, dep.version));
                }
            }
            
            result
        }
        Err(e) => format!("❌ Failed to read project manifest: {}", e),
    }
}

fn package_info(package: &str) -> Result<BuiltinValue, String> {
    // This would typically query a package registry
    // For now, provide basic package information
    let info = format!("📦 Package: {}\n🔍 Status: Checking...\n💡 Use search_package() for repository search", package);
    Ok(BuiltinValue::String(info))
} 