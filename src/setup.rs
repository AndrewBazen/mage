use dirs::home_dir;
use serde_json::Value;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;

fn log(message: &str, dry_run: bool) -> io::Result<()> {
    if dry_run {
        println!("[dry-run] {}", message);
        return Ok(());
    }
    let log_path = home_dir().unwrap().join(".mage/setup.log");
    fs::create_dir_all(log_path.parent().unwrap())?;
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_path)?;
    writeln!(file, "{}", message)?;
    Ok(())
}

#[derive(clap::Parser)]
pub struct SetupOptions {
    /// Run setup without making changes
    #[arg(long)]
    pub dry_run: bool,
}

pub fn setup_from_cli(options: &SetupOptions) -> io::Result<()> {
    setup_with_dry_run(options.dry_run)
}

pub fn setup() -> io::Result<()> {
    setup_with_dry_run(false)
}

pub fn setup_with_dry_run(dry_run: bool) -> io::Result<()> {
    println!(
        "🧙 Running Mage setup{}...",
        if dry_run { " (dry-run)" } else { "" }
    );
    log("Mage setup started...", dry_run)?;

    if Command::new("tree-sitter")
        .arg("--version")
        .output()
        .is_err()
    {
        let msg = "❌ tree-sitter CLI is not installed.";
        eprintln!("{}", msg);
        log(msg, dry_run)?;
        return Ok(());
    }
    log("✅ tree-sitter CLI is available", dry_run)?;

    let config_path = home_dir()
        .expect("Could not determine home directory")
        .join(".tree-sitter/config.json");

    if !config_path.exists() {
        println!("📦 Initializing tree-sitter config...");
        log("📦 Running tree-sitter init-config", dry_run)?;
        if !dry_run {
            Command::new("tree-sitter").arg("init-config").status()?;
        }
    }

    let grammar_path = std::env::current_dir()?.join("tree-sitter-mage");
    let config_json = fs::read_to_string(&config_path)?;
    let mut config: Value = serde_json::from_str(&config_json)?;

    let dirs = config
        .get_mut("parser-directories")
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| io::Error::other("Invalid config.json format"))?;

    let grammar_str = grammar_path.to_string_lossy().to_string();
    if !dirs.iter().any(|v| v.as_str() == Some(&grammar_str)) {
        dirs.push(Value::String(grammar_str.clone()));
        println!("✨ Added {} to parser-directories", grammar_str);
        log(
            &format!("✨ Added {} to parser-directories", grammar_str),
            dry_run,
        )?;
    }

    if !dry_run {
        fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
    }
    log("✅ Tree-sitter config updated", dry_run)?;

    let ftdetect_path = home_dir().unwrap().join(".config/nvim/ftdetect/mage.vim");
    if !ftdetect_path.exists() {
        if !dry_run {
            fs::create_dir_all(ftdetect_path.parent().unwrap())?;
            fs::write(
                &ftdetect_path,
                "au BufRead,BufNewFile *.mage set filetype=mage\n",
            )?;
        }
        println!("🔮 Wrote filetype detection to ~/.config/nvim/ftdetect/mage.vim");
        log("🔮 Created mage.vim filetype detection", dry_run)?;
    }

    let ts_config_path = home_dir()
        .unwrap()
        .join(".config/nvim/lua/plugins/treesitter.lua");
    if ts_config_path.exists() {
        let mut existing = fs::read_to_string(&ts_config_path)?;
        if !existing.contains("mage") {
            let insertion = "\n-- Mage Treesitter Config\nlocal parser_config = require('nvim-treesitter.parsers').get_parser_configs()\nparser_config.mage = {\n  install_info = {\n    url = '~/projects/mage/tree-sitter-mage',\n    files = { 'src/parser.c' },\n    branch = 'main',\n  },\n  filetype = 'mage',\n}\n";
            existing.push_str(insertion);
            if !dry_run {
                fs::write(&ts_config_path, existing)?;
            }
            println!("🧩 Appended Mage config to treesitter.lua");
            log(
                "🧩 Mage parser_config injected into treesitter.lua",
                dry_run,
            )?;
        }
    }

    let vscode_path = home_dir()
        .unwrap()
        .join(".vscode/extensions/mage-lang/syntaxes/mage.tmLanguage.json");
    if !vscode_path.exists() {
        println!("⚠️ VS Code integration not found, consider creating a grammar extension for it.");
        log("⚠️ VS Code syntax grammar not found", dry_run)?;
    }

    println!("✅ Mage setup complete!");
    log("✅ Mage setup completed successfully", dry_run)?;
    Ok(())
}
