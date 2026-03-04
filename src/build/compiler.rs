use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};

pub fn compile_object(
    compiler: &str,
    source: &Path,
    output: &Path,
    flags: &[String],
    includes: &[String],
) -> Result<()> {
    let mut cmd = Command::new(compiler);
    cmd.arg("-c");

    for flag in flags {
        cmd.arg(flag);
    }

    for inc in includes {
        cmd.arg(format!("-I{inc}"));
    }

    cmd.arg("-o").arg(output).arg(source);

    let result = cmd
        .output()
        .with_context(|| format!("failed to execute compiler '{compiler}'"))?;

    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        bail!(
            "compilation of {} failed:\n{}",
            source.display(),
            stderr.trim()
        );
    }

    Ok(())
}

pub fn link(
    compiler: &str,
    objects: &[PathBuf],
    output: &Path,
    flags: &[String],
    libs: &[String],
) -> Result<()> {
    let mut cmd = Command::new(compiler);

    for flag in flags {
        cmd.arg(flag);
    }

    cmd.arg("-o").arg(output);

    for obj in objects {
        cmd.arg(obj);
    }

    for lib in libs {
        cmd.arg(format!("-l{lib}"));
    }

    let result = cmd
        .output()
        .context("failed to execute linker")?;

    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        bail!("linking failed:\n{}", stderr.trim());
    }

    Ok(())
}

pub fn strip_binary(path: &Path) -> Result<()> {
    let result = Command::new("strip")
        .arg(path)
        .output()
        .context("failed to execute 'strip'")?;

    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        bail!("strip failed:\n{}", stderr.trim());
    }

    Ok(())
}

pub fn object_path(obj_dir: &Path, source: &Path, project_dir: &Path) -> PathBuf {
    let relative = source
        .strip_prefix(project_dir)
        .unwrap_or(source);
    let obj_name = relative.to_string_lossy().replace(['/', '\\'], "_");
    obj_dir.join(format!("{obj_name}.o"))
}
