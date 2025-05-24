use clap::{Parser, Subcommand};
use mage::{run, run_repl, syntax};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about = "🧙 The Mage Scripting Language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Optional script file to run (shorthand for `run <SCRIPT>`)
    #[arg(global = false)]
    script: Option<String>,

    /// Override shell for script execution
    #[arg(long, global = true)]
    shell: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a mage script
    Run {
        /// Script file to run
        file: String,
    },
    /// Start an interactive REPL
    Repl {},
    /// Create a new .mageconfig file in the current directory
    Init {},
    /// Highlight a mage script using tree-sitter (for testing)
    Highlight {
        /// Script file to highlight
        file: String,
    },
    /// format a Mage script
    Format {
        /// Script file to format, or use '-' for stdin
        file: String,

        /// Format in place (overwrite the file)
        #[arg(short, long)]
        inplace: bool,
    },
    Setup {
        #[command(flatten)]
        options: mage::setup::SetupOptions,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { file }) => {
            run_script(file, cli.shell.as_deref());
        }
        Some(Commands::Repl {}) => {
            if let Err(e) = run_repl(cli.shell.as_deref()) {
                eprintln!("❌ {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Format { file, inplace }) => {
            format_script(file, *inplace);
        }
        Some(Commands::Setup { options }) => {
            if let Err(e) = mage::setup::setup_from_cli(options) {
                eprintln!("❌ {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Init {}) => {
            init_config();
        }
        Some(Commands::Highlight { file }) => {
            highlight_script(file);
        }
        None => {
            // If no command but a script is provided, run it
            if let Some(script) = cli.script {
                run_script(&script, cli.shell.as_deref());
            } else {
                // No subcommand and no script, show help
                println!("🧙 Welcome to Mage!");
                println!("Try one of these commands:");
                println!("  mage run file.mage    - Run a script");
                println!("  mage repl             - Start interactive REPL");
                println!("  mage init             - Create .mageconfig file");
                println!("  mage highlight file.mage - Test syntax highlighting");
                println!("  mage --help           - Show help");
            }
        }
    }
}

fn run_script(path: &str, shell: Option<&str>) {
    let code = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("📜 Failed to read spellbook: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = run(&code, shell) {
        eprintln!("❌ {}", e);
        std::process::exit(1);
    }
}

fn highlight_script(path: &str) {
    let code = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("📜 Failed to read spellbook: {}", e);
            std::process::exit(1);
        }
    };

    // Check if tree-sitter is available
    if syntax::is_tree_sitter_available() {
        match syntax::highlight_html(&code) {
            Ok(html) => {
                // Save the HTML to a file
                let output_path = format!("{}.html", path);
                match fs::write(&output_path, html) {
                    Ok(_) => println!("✨ Highlighted HTML saved to {}", output_path),
                    Err(e) => eprintln!("❌ Failed to write HTML: {}", e),
                }
            }
            Err(e) => eprintln!("❌ Failed to highlight: {}", e),
        }
    } else {
        println!("🔍 Tree-sitter syntax highlighting is not yet available.");
        println!(
            "📝 When tree-sitter support is added, this command will generate HTML with syntax highlighting."
        );
        println!("💻 For now, you can use the colored REPL with basic syntax highlighting:");
        println!("   mage repl");
    }
}

fn format_script(path: &str, inplace: bool) {
    use std::io::{self, Read};

    let input = if path == "-" {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    } else {
        match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("📜 Failed to read script: {}", e);
                std::process::exit(1);
            }
        }
    };

    match mage::format(&input) {
        Ok(output) => {
            if inplace && path != "-" {
                if let Err(e) = fs::write(path, output) {
                    eprintln!("❌ Failed to overwrite file: {}", e);
                    std::process::exit(1);
                }
            } else {
                println!("{}", output);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to overwrite file: {}", e);
            std::process::exit(1);
        }
    }
}

fn init_config() {
    let config_path = Path::new(".mageconfig");

    if config_path.exists() {
        println!("⚠️ .mageconfig already exists in the current directory");
        return;
    }

    let config_content = r#"# Mage Configuration File
# Uncomment and modify settings as needed

# Override default shell
# shell=powershell

# Add custom configuration options below
# option_name=value
"#;

    match fs::write(config_path, config_content) {
        Ok(_) => println!("✨ Created .mageconfig in the current directory"),
        Err(e) => eprintln!("❌ Failed to create .mageconfig: {}", e),
    }
}
