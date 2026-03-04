use std::path::Path;

use anyhow::{Result, bail};

use crate::util::output::{self, OutputConfig};
use crate::util::process;

pub fn generate_ca(cn: &str, org: &str, out: &str, output: &OutputConfig) -> Result<()> {
    if which::which("openssl").is_err() {
        bail!("openssl not found in PATH");
    }

    let out_path = Path::new(out);
    if let Some(parent) = out_path.parent() {
        crate::util::fs::ensure_dir(parent)?;
    }

    let key_path = format!("{}.key", out.trim_end_matches(".pem").trim_end_matches(".crt"));

    output::status("Generating", &format!("CA certificate (CN={cn}, O={org})"));

    let subject = format!("/CN={cn}/O={org}");
    process::run_command_checked(
        "openssl",
        &[
            "req", "-x509", "-newkey", "rsa:4096", "-keyout", &key_path,
            "-out", out, "-days", "3650", "-nodes",
            "-subj", &subject,
        ],
        None,
        None,
    )?;

    output::status("Generated", &format!("CA cert: {out}, key: {key_path}"));
    output::verbose(output, &format!("subject: {subject}"));

    Ok(())
}

pub fn self_sign(cn: &str, days: u32, out: &str, output: &OutputConfig) -> Result<()> {
    if which::which("openssl").is_err() {
        bail!("openssl not found in PATH");
    }

    let out_path = Path::new(out);
    if let Some(parent) = out_path.parent() {
        crate::util::fs::ensure_dir(parent)?;
    }

    let key_path = format!("{}.key", out.trim_end_matches(".pem").trim_end_matches(".crt"));
    let days_str = days.to_string();

    output::status("Generating", &format!("self-signed certificate (CN={cn}, {days} days)"));

    let subject = format!("/CN={cn}");
    process::run_command_checked(
        "openssl",
        &[
            "req", "-x509", "-newkey", "rsa:2048", "-keyout", &key_path,
            "-out", out, "-days", &days_str, "-nodes",
            "-subj", &subject,
        ],
        None,
        None,
    )?;

    output::status("Generated", &format!("cert: {out}, key: {key_path}"));
    output::verbose(output, &format!("subject: {subject}, validity: {days} days"));

    Ok(())
}

pub fn inspect_cert(path: &str, _output: &OutputConfig) -> Result<()> {
    if which::which("openssl").is_err() {
        bail!("openssl not found in PATH");
    }

    if !Path::new(path).exists() {
        bail!("certificate file not found: {path}");
    }

    let result = process::run_command_checked(
        "openssl",
        &["x509", "-in", path, "-text", "-noout"],
        None,
        None,
    )?;

    println!("{}", result.stdout);

    Ok(())
}
