use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub dependencies: HashMap<String, PackageDependency>,
    pub dev_dependencies: HashMap<String, PackageDependency>,
    pub scripts: HashMap<String, String>,
    pub keywords: Vec<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDependency {
    pub version: String,
    pub source: PackageSource,
    pub optional: bool,
    pub platform: Option<String>, // "windows", "macos", "linux"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageSource {
    Registry(String), // Official package manager
    Git { url: String, rev: Option<String> },
    Path(String), // Local path
    Url(String),  // Direct download
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageLock {
    pub version: String,
    pub packages: HashMap<String, LockedPackage>,
    pub metadata: LockMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPackage {
    pub version: String,
    pub source: PackageSource,
    pub checksum: Option<String>,
    pub dependencies: Vec<String>,
    pub resolved_at: String, // timestamp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockMetadata {
    pub generated_at: String,
    pub generator: String,
    pub platform: String,
    pub mage_version: String,
}

pub struct PackageResolver {
    manifest_path: PathBuf,
    lock_path: PathBuf,
    packages_dir: PathBuf,
}

impl PackageResolver {
    pub fn new(project_root: &Path) -> Self {
        Self {
            manifest_path: project_root.join("mage.toml"),
            lock_path: project_root.join("mage.lock"),
            packages_dir: project_root.join(".mage/packages"),
        }
    }

    pub fn init_project(&self, name: &str) -> Result<(), String> {
        if self.manifest_path.exists() {
            return Err("Project already initialized (mage.toml exists)".to_string());
        }

        let manifest = PackageManifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            description: Some(format!("A mage project: {}", name)),
            author: None,
            license: Some("MIT".to_string()),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            scripts: HashMap::from([
                ("setup".to_string(), "mage scripts/setup.mage".to_string()),
                ("build".to_string(), "mage scripts/build.mage".to_string()),
                ("test".to_string(), "mage scripts/test.mage".to_string()),
            ]),
            keywords: vec!["mage".to_string()],
            repository: None,
            homepage: None,
        };

        self.write_manifest(&manifest)?;
        self.create_directory_structure()?;

        Ok(())
    }

    pub fn add_dependency(&self, package: &str, version: &str, dev: bool) -> Result<(), String> {
        let mut manifest = self.read_manifest()?;

        let dependency = PackageDependency {
            version: version.to_string(),
            source: PackageSource::Registry("auto".to_string()),
            optional: false,
            platform: None,
        };

        if dev {
            manifest
                .dev_dependencies
                .insert(package.to_string(), dependency);
        } else {
            manifest
                .dependencies
                .insert(package.to_string(), dependency);
        }

        self.write_manifest(&manifest)?;
        self.resolve_dependencies()?;

        Ok(())
    }

    pub fn remove_dependency(&self, package: &str) -> Result<(), String> {
        let mut manifest = self.read_manifest()?;

        let removed = manifest.dependencies.remove(package).is_some()
            || manifest.dev_dependencies.remove(package).is_some();

        if !removed {
            return Err(format!("Package '{}' not found in dependencies", package));
        }

        self.write_manifest(&manifest)?;
        self.resolve_dependencies()?;

        Ok(())
    }

    pub fn resolve_dependencies(&self) -> Result<(), String> {
        let manifest = self.read_manifest()?;
        let mut resolved_packages = HashMap::new();
        let mut resolution_queue = Vec::new();

        // Add direct dependencies to queue
        for (name, dep) in &manifest.dependencies {
            resolution_queue.push((name.clone(), dep.clone(), false));
        }

        for (name, dep) in &manifest.dev_dependencies {
            resolution_queue.push((name.clone(), dep.clone(), true));
        }

        // Resolve dependencies breadth-first
        let mut visited = HashSet::new();
        while let Some((name, dep, _is_dev)) = resolution_queue.pop() {
            if visited.contains(&name) {
                continue;
            }
            visited.insert(name.clone());

            // Skip platform-specific packages
            if let Some(platform) = &dep.platform
                && platform != std::env::consts::OS
            {
                continue;
            }

            // Resolve package version and source
            let resolved_version = self.resolve_version(&name, &dep)?;
            let checksum = self.calculate_checksum(&name, &resolved_version, &dep.source)?;

            let locked_package = LockedPackage {
                version: resolved_version,
                source: dep.source.clone(),
                checksum: Some(checksum),
                dependencies: Vec::new(), // TODO: Parse transitive dependencies
                resolved_at: chrono::Utc::now().to_rfc3339(),
            };

            resolved_packages.insert(name, locked_package);
        }

        // Create lock file
        let lock = PackageLock {
            version: "1".to_string(),
            packages: resolved_packages,
            metadata: LockMetadata {
                generated_at: chrono::Utc::now().to_rfc3339(),
                generator: "mage".to_string(),
                platform: std::env::consts::OS.to_string(),
                mage_version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        self.write_lock(&lock)?;
        Ok(())
    }

    pub fn install_dependencies(&self, dev: bool) -> Result<(), String> {
        let manifest = self.read_manifest()?;
        let lock = self.read_lock().ok();

        fs::create_dir_all(&self.packages_dir)
            .map_err(|e| format!("Failed to create packages directory: {}", e))?;

        // Install regular dependencies
        for (name, dep) in &manifest.dependencies {
            self.install_package(name, dep, &lock)?;
        }

        // Install dev dependencies if requested
        if dev {
            for (name, dep) in &manifest.dev_dependencies {
                self.install_package(name, dep, &lock)?;
            }
        }

        Ok(())
    }

    fn install_package(
        &self,
        name: &str,
        dep: &PackageDependency,
        _lock: &Option<PackageLock>,
    ) -> Result<(), String> {
        // Check if package is platform-specific
        if let Some(platform) = &dep.platform
            && platform != &std::env::consts::OS.to_string()
        {
            println!(
                "⏭️  Skipping {} (not for platform {})",
                name,
                std::env::consts::OS
            );
            return Ok(());
        }

        println!("📦 Installing {}...", name);

        match &dep.source {
            PackageSource::Registry(manager) => {
                if manager == "auto" {
                    // Use system package manager
                    let _ =
                        crate::builtins::call_builtin("install_package", vec![name.to_string()], &mut crate::output::OutputCollector::direct())
                            .map_err(|e| format!("Failed to install {}: {}", name, e))?;
                } else {
                    // Use specific package manager
                    self.install_with_manager(name, manager)?;
                }
            }
            PackageSource::Git { url, rev } => {
                self.install_from_git(name, url, rev.as_deref())?;
            }
            PackageSource::Path(path) => {
                self.install_from_path(name, path)?;
            }
            PackageSource::Url(url) => {
                self.install_from_url(name, url)?;
            }
        }

        println!("✅ Installed {}", name);
        Ok(())
    }

    fn install_with_manager(&self, package: &str, manager: &str) -> Result<(), String> {
        // Use specific package manager
        use std::process::Command;

        let install_cmd = match manager {
            "npm" => format!("npm install -g {}", package),
            "pip" => format!("pip install {}", package),
            "cargo" => format!("cargo install {}", package),
            "gem" => format!("gem install {}", package),
            _ => return Err(format!("Unsupported package manager: {}", manager)),
        };

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", &install_cmd]).output()
        } else {
            Command::new("sh").args(["-c", &install_cmd]).output()
        };

        match output {
            Ok(output) if output.status.success() => Ok(()),
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(format!("Installation failed: {}", stderr))
            }
            Err(e) => Err(format!("Failed to execute install command: {}", e)),
        }
    }

    fn install_from_git(&self, name: &str, url: &str, rev: Option<&str>) -> Result<(), String> {
        let package_dir = self.packages_dir.join(name);

        // Clone or update repository
        if package_dir.exists() {
            // Update existing repo
            use std::process::Command;
            let output = Command::new("git")
                .args(["pull"])
                .current_dir(&package_dir)
                .output()
                .map_err(|e| format!("Failed to update git repo: {}", e))?;

            if !output.status.success() {
                return Err("Failed to update git repository".to_string());
            }
        } else {
            // Clone new repo
            use std::process::Command;
            let mut args = vec!["clone", url];
            if let Some(rev) = rev {
                args.extend(&["--branch", rev]);
            }
            args.push(package_dir.to_str().unwrap());

            let output = Command::new("git")
                .args(&args)
                .output()
                .map_err(|e| format!("Failed to clone git repo: {}", e))?;

            if !output.status.success() {
                return Err("Failed to clone git repository".to_string());
            }
        }

        // Run package-specific install script if it exists
        let install_script = package_dir.join("install.mage");
        if install_script.exists() {
            use std::process::Command;
            let output = Command::new("mage")
                .arg(install_script)
                .output()
                .map_err(|e| format!("Failed to run install script: {}", e))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Install script failed: {}", stderr));
            }
        }

        Ok(())
    }

    fn install_from_path(&self, name: &str, path: &str) -> Result<(), String> {
        let source_path = Path::new(path);
        let package_dir = self.packages_dir.join(name);

        if !source_path.exists() {
            return Err(format!("Source path does not exist: {}", path));
        }

        // Create symlink to local package
        #[cfg(unix)]
        {
            use std::os::unix::fs;
            fs::symlink(source_path, &package_dir)
                .map_err(|e| format!("Failed to create symlink: {}", e))?;
        }

        #[cfg(windows)]
        {
            use std::os::windows::fs;
            if source_path.is_dir() {
                fs::symlink_dir(source_path, &package_dir)
                    .map_err(|e| format!("Failed to create directory symlink: {}", e))?;
            } else {
                fs::symlink_file(source_path, &package_dir)
                    .map_err(|e| format!("Failed to create file symlink: {}", e))?;
            }
        }

        Ok(())
    }

    fn install_from_url(&self, name: &str, url: &str) -> Result<(), String> {
        let package_dir = self.packages_dir.join(name);
        fs::create_dir_all(&package_dir)
            .map_err(|e| format!("Failed to create package directory: {}", e))?;

        // Download and extract
        let download_path = package_dir.join("download");
        let _ = crate::builtins::call_builtin(
            "download",
            vec![url.to_string(), download_path.to_string_lossy().to_string()],
            &mut crate::output::OutputCollector::direct(),
        )
        .map_err(|e| format!("Failed to download package: {}", e))?;

        // TODO: Handle different archive formats (zip, tar.gz, etc.)
        Ok(())
    }

    fn resolve_version(&self, _name: &str, dep: &PackageDependency) -> Result<String, String> {
        // Simple version resolution - in a real implementation you'd:
        // 1. Parse version constraints (^1.0.0, ~1.2.0, >=1.0.0, etc.)
        // 2. Query package registry for available versions
        // 3. Find best matching version
        // 4. Handle conflicts between dependencies

        // For now, just return the specified version
        Ok(dep.version.clone())
    }

    fn calculate_checksum(
        &self,
        name: &str,
        version: &str,
        _source: &PackageSource,
    ) -> Result<String, String> {
        // In a real implementation, you'd calculate actual checksums
        // For now, return a placeholder
        Ok(format!(
            "sha256:{:x}",
            md5::compute(format!("{}-{}", name, version))
        ))
    }

    pub fn read_manifest(&self) -> Result<PackageManifest, String> {
        let content = fs::read_to_string(&self.manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;

        toml::from_str(&content).map_err(|e| format!("Failed to parse manifest: {}", e))
    }

    fn write_manifest(&self, manifest: &PackageManifest) -> Result<(), String> {
        let content = toml::to_string_pretty(manifest)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;

        fs::write(&self.manifest_path, content)
            .map_err(|e| format!("Failed to write manifest: {}", e))
    }

    fn read_lock(&self) -> Result<PackageLock, String> {
        let content = fs::read_to_string(&self.lock_path)
            .map_err(|e| format!("Failed to read lock file: {}", e))?;

        toml::from_str(&content).map_err(|e| format!("Failed to parse lock file: {}", e))
    }

    fn write_lock(&self, lock: &PackageLock) -> Result<(), String> {
        let content = toml::to_string_pretty(lock)
            .map_err(|e| format!("Failed to serialize lock file: {}", e))?;

        fs::write(&self.lock_path, content).map_err(|e| format!("Failed to write lock file: {}", e))
    }

    fn create_directory_structure(&self) -> Result<(), String> {
        let dirs = [".mage", ".mage/packages", "scripts", "lib", "tests"];

        for dir in &dirs {
            let dir_path = self.manifest_path.parent().unwrap().join(dir);
            fs::create_dir_all(dir_path)
                .map_err(|e| format!("Failed to create directory {}: {}", dir, e))?;
        }

        // Create default scripts
        let script_contents = [
            (
                "scripts/setup.mage",
                include_str!("../templates/setup.mage"),
            ),
            (
                "scripts/build.mage",
                include_str!("../templates/build.mage"),
            ),
            ("scripts/test.mage", include_str!("../templates/test.mage")),
        ];

        for (path, content) in &script_contents {
            let script_path = self.manifest_path.parent().unwrap().join(path);
            if !script_path.exists() {
                fs::write(script_path, content)
                    .map_err(|e| format!("Failed to create script {}: {}", path, e))?;
            }
        }

        Ok(())
    }
}
