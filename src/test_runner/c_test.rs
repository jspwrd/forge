use std::path::{Path, PathBuf};

use anyhow::Result;
use glob::glob;

use crate::util::output::{self, OutputConfig};
use crate::util::process;

pub fn discover_tests(project_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut tests = Vec::new();
    let test_dir = project_dir.join("tests");

    if !test_dir.exists() {
        return Ok(tests);
    }

    let patterns = [
        test_dir.join("test_*.c").to_string_lossy().to_string(),
        test_dir.join("*_test.c").to_string_lossy().to_string(),
    ];

    for pattern in &patterns {
        for entry in glob(pattern)? {
            let path = entry?;
            if path.is_file() {
                tests.push(path);
            }
        }
    }

    tests.sort();
    tests.dedup();
    Ok(tests)
}

pub fn run_c_tests(project_dir: &Path, compiler: &str, output: &OutputConfig) -> Result<bool> {
    let tests = discover_tests(project_dir)?;

    if tests.is_empty() {
        output::warning("no C test files found");
        return Ok(true);
    }

    let bin_dir = project_dir.join("bin").join("tests");
    crate::util::fs::ensure_dir(&bin_dir)?;

    let mut all_pass = true;
    let src_dir = project_dir.join("src");

    for test_file in &tests {
        let test_name = test_file.file_stem().unwrap_or_default().to_string_lossy();

        let test_bin = bin_dir.join(&*test_name);

        output::status("Compiling", &format!("test '{test_name}'"));

        // Find all .c source files that aren't main.c (to link with tests)
        let mut sources: Vec<String> = vec![test_file.to_string_lossy().to_string()];

        // Add non-main source files
        let src_pattern = src_dir.join("**/*.c").to_string_lossy().to_string();
        for entry in glob(&src_pattern)? {
            let path = entry?;
            if path.is_file() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name != "main.c" {
                    sources.push(path.to_string_lossy().to_string());
                }
            }
        }

        let mut args: Vec<&str> = vec!["-o", test_bin.to_str().unwrap_or("")];
        let src_refs: Vec<&str> = sources.iter().map(|s| s.as_str()).collect();
        args.extend(src_refs);
        args.push("-I");
        args.push(src_dir.to_str().unwrap_or("src"));

        let compile_result = process::run_command(compiler, &args, Some(project_dir), None)?;

        if !compile_result.status.success() {
            output::error(&format!(
                "test '{test_name}' failed to compile:\n{}",
                compile_result.stderr.trim()
            ));
            all_pass = false;
            continue;
        }

        output::status("Running", &format!("test '{test_name}'"));

        let run_result = process::run_command(
            test_bin.to_str().unwrap_or(""),
            &[],
            Some(project_dir),
            None,
        )?;

        if run_result.status.success() {
            output::status("Passed", &format!("test '{test_name}'"));
            output::verbose(output, run_result.stdout.trim());
        } else {
            output::error(&format!("test '{test_name}' FAILED"));
            eprint!("{}", run_result.stdout);
            eprint!("{}", run_result.stderr);
            all_pass = false;
        }
    }

    Ok(all_pass)
}
