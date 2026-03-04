mod checks;
mod report;

use std::path::Path;

use anyhow::{Result, bail};

use crate::cli::validate_cmd::ValidateArgs;
use crate::manifest;
use crate::util::output::{self, OutputConfig};

use report::ValidationReport;

pub fn execute_validate(args: ValidateArgs, _output: &OutputConfig) -> Result<()> {
    let manifest = manifest::load_from_cwd()?;
    let binary_path = Path::new(&args.binary);

    if !binary_path.exists() {
        bail!("binary not found: {}", args.binary);
    }

    output::status("Validating", &binary_path.display().to_string());

    let config = &manifest.validate;
    let mut report = ValidationReport::new();

    // Run all checks
    report.add(
        "debug symbols".to_string(),
        checks::check_debug_symbols(binary_path, config)?,
    );
    report.add(
        "plaintext strings".to_string(),
        checks::check_plaintext_strings(binary_path, config)?,
    );
    report.add(
        "compiler watermarks".to_string(),
        checks::check_compiler_watermarks(binary_path, config)?,
    );
    report.add(
        "binary size".to_string(),
        checks::check_binary_size(binary_path, config)?,
    );

    // Check unpatched sentinels
    let sentinels: Vec<String> = manifest
        .patch
        .values()
        .map(|f| f.sentinel.clone())
        .collect();
    report.add(
        "unpatched sentinels".to_string(),
        checks::check_unpatched_sentinels(binary_path, &sentinels)?,
    );

    report.add(
        "RPATH/RUNPATH".to_string(),
        checks::check_rpath(binary_path, config)?,
    );
    report.add(
        "build path leakage".to_string(),
        checks::check_buildpaths(binary_path, config)?,
    );

    report.print();

    if report.has_failures() {
        bail!("validation failed");
    }

    Ok(())
}
