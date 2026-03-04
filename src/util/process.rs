use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Output};

use anyhow::{Context, Result, bail};

pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub status: std::process::ExitStatus,
}

pub fn run_command(
    program: &str,
    args: &[&str],
    cwd: Option<&Path>,
    env: Option<&HashMap<String, String>>,
) -> Result<CommandResult> {
    let mut cmd = Command::new(program);
    cmd.args(args);

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    if let Some(env_vars) = env {
        for (k, v) in env_vars {
            cmd.env(k, v);
        }
    }

    let output: Output = cmd
        .output()
        .with_context(|| format!("failed to execute '{}'", program))?;

    let result = CommandResult {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        status: output.status,
    };

    Ok(result)
}

pub fn run_command_checked(
    program: &str,
    args: &[&str],
    cwd: Option<&Path>,
    env: Option<&HashMap<String, String>>,
) -> Result<CommandResult> {
    let result = run_command(program, args, cwd, env)?;
    if !result.status.success() {
        let stderr = result.stderr.trim();
        bail!(
            "'{}' exited with status {}: {}",
            program,
            result.status,
            if stderr.is_empty() {
                "(no output)"
            } else {
                stderr
            }
        );
    }
    Ok(result)
}
