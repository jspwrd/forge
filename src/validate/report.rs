use colored::Colorize;

use super::checks::CheckResult;

pub struct ValidationReport {
    pub results: Vec<(String, CheckResult)>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add(&mut self, name: String, result: CheckResult) {
        self.results.push((name, result));
    }

    pub fn has_failures(&self) -> bool {
        self.results.iter().any(|(_, r)| r.is_fail())
    }

    pub fn warning_count(&self) -> usize {
        self.results.iter().filter(|(_, r)| r.is_warn()).count()
    }

    pub fn print(&self) {
        for (name, result) in &self.results {
            let dots = ".".repeat(40_usize.saturating_sub(name.len()));
            let status = match result {
                CheckResult::Pass => "PASS".green().bold().to_string(),
                CheckResult::Fail(msg) => format!("{} ({})", "FAIL".red().bold(), msg),
                CheckResult::Warn(msg) => format!("{} ({})", "WARN".yellow().bold(), msg),
            };
            eprintln!("   Checking {} {} {}", name, dots, status);
        }

        let warnings = self.warning_count();
        let overall = if self.has_failures() {
            "FAIL".red().bold().to_string()
        } else {
            "PASS".green().bold().to_string()
        };

        eprintln!();
        if warnings > 0 {
            eprintln!("   Result: {} ({} warning(s))", overall, warnings);
        } else {
            eprintln!("   Result: {}", overall);
        }
    }
}
