use std::io::{self, Write};

/// Error type replacing process::exit() calls in the interpreter.
#[derive(Debug, Clone)]
pub enum InterpreterError {
    /// A `curse` statement (user-initiated error exit)
    Curse(String),
    /// A shell command returned a non-zero exit code
    CommandFailed(i32),
    /// A shell command failed to execute
    CommandError(String),
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InterpreterError::Curse(msg) => write!(f, "CURSE: {}", msg),
            InterpreterError::CommandFailed(code) => write!(f, "Command failed with exit code {}", code),
            InterpreterError::CommandError(msg) => write!(f, "Command error: {}", msg),
        }
    }
}

/// Controls where interpreter output goes.
///
/// - `Direct`: prints to real stdout/stderr (CLI, scripts)
/// - `Buffered`: captures into vectors (TUI, testing)
pub struct OutputCollector {
    mode: OutputMode,
}

enum OutputMode {
    Direct,
    Buffered {
        stdout_buf: Vec<String>,
        stderr_buf: Vec<String>,
    },
}

impl OutputCollector {
    /// Create a collector that prints directly to stdout/stderr.
    pub fn direct() -> Self {
        Self {
            mode: OutputMode::Direct,
        }
    }

    /// Create a collector that captures output into buffers.
    pub fn buffered() -> Self {
        Self {
            mode: OutputMode::Buffered {
                stdout_buf: Vec::new(),
                stderr_buf: Vec::new(),
            },
        }
    }

    /// Returns true if this collector is in buffered mode.
    pub fn is_buffered(&self) -> bool {
        matches!(self.mode, OutputMode::Buffered { .. })
    }

    pub fn println(&mut self, msg: &str) {
        match &mut self.mode {
            OutputMode::Direct => {
                println!("{}", msg);
                io::stdout().flush().ok();
            }
            OutputMode::Buffered { stdout_buf, .. } => {
                stdout_buf.push(msg.to_string());
            }
        }
    }

    pub fn print(&mut self, msg: &str) {
        match &mut self.mode {
            OutputMode::Direct => {
                print!("{}", msg);
                io::stdout().flush().ok();
            }
            OutputMode::Buffered { stdout_buf, .. } => {
                // Append to last line or create new one
                if let Some(last) = stdout_buf.last_mut() {
                    last.push_str(msg);
                } else {
                    stdout_buf.push(msg.to_string());
                }
            }
        }
    }

    pub fn eprintln(&mut self, msg: &str) {
        match &mut self.mode {
            OutputMode::Direct => {
                eprintln!("{}", msg);
            }
            OutputMode::Buffered { stderr_buf, .. } => {
                stderr_buf.push(msg.to_string());
            }
        }
    }

    pub fn eprint(&mut self, msg: &str) {
        match &mut self.mode {
            OutputMode::Direct => {
                eprint!("{}", msg);
            }
            OutputMode::Buffered { stderr_buf, .. } => {
                if let Some(last) = stderr_buf.last_mut() {
                    last.push_str(msg);
                } else {
                    stderr_buf.push(msg.to_string());
                }
            }
        }
    }

    /// Take all captured stdout lines, leaving the buffer empty.
    pub fn take_stdout(&mut self) -> Vec<String> {
        match &mut self.mode {
            OutputMode::Direct => Vec::new(),
            OutputMode::Buffered { stdout_buf, .. } => std::mem::take(stdout_buf),
        }
    }

    /// Take all captured stderr lines, leaving the buffer empty.
    pub fn take_stderr(&mut self) -> Vec<String> {
        match &mut self.mode {
            OutputMode::Direct => Vec::new(),
            OutputMode::Buffered { stderr_buf, .. } => std::mem::take(stderr_buf),
        }
    }
}
