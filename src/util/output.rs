use colored::Colorize;

#[derive(Debug, Clone, Default)]
pub struct OutputConfig {
    pub verbose: bool,
    pub quiet: bool,
    pub no_color: bool,
}

impl OutputConfig {
    pub fn apply(&self) {
        if self.no_color {
            colored::control::set_override(false);
        }
    }
}

pub fn status(label: &str, message: &str) {
    eprintln!("{:>12} {}", label.green().bold(), message);
}

pub fn warning(message: &str) {
    eprintln!("{}: {}", "warning".yellow().bold(), message);
}

pub fn error(message: &str) {
    eprintln!("{}: {}", "error".red().bold(), message);
}

pub fn verbose(config: &OutputConfig, message: &str) {
    if config.verbose {
        eprintln!("{}: {}", "debug".cyan(), message);
    }
}
