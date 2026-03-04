mod c_test;
mod shellcheck;

use anyhow::{Result, bail};

use crate::manifest;
use crate::util::output::{self, OutputConfig};

pub fn run_tests(output: &OutputConfig) -> Result<()> {
    let manifest = manifest::load_from_cwd()?;
    let project_dir = manifest::manifest_dir()?;

    output::status("Testing", &format!("project '{}'", manifest.project.name));

    let mut all_pass = true;

    // Run shellcheck on scripts
    if !manifest.scripts.is_empty() {
        output::status("Running", "shellcheck on scripts");
        let scripts: Vec<(String, String)> = manifest
            .scripts
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let pass = shellcheck::run_shellcheck(&scripts, &project_dir, output)?;
        if !pass {
            all_pass = false;
        }
    }

    // Run C tests
    let compiler = manifest
        .build
        .compiler
        .as_deref()
        .unwrap_or("gcc");
    output::status("Running", "C tests");
    let pass = c_test::run_c_tests(&project_dir, compiler, output)?;
    if !pass {
        all_pass = false;
    }

    if all_pass {
        output::status("Result", "all tests passed");
        Ok(())
    } else {
        bail!("some tests failed");
    }
}
