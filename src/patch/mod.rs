mod scanner;
pub mod types;
mod writer;

use std::path::Path;

use anyhow::{Result, bail};

use crate::cli::patch_cmd::PatchArgs;
use crate::manifest;
use crate::util::fs::atomic_write;
use crate::util::output::{self, OutputConfig};

pub fn execute_patch(args: PatchArgs, output: &OutputConfig) -> Result<()> {
    let manifest = manifest::load_from_cwd()?;
    let binary_path = Path::new(&args.binary);

    if !binary_path.exists() {
        bail!("binary not found: {}", args.binary);
    }

    if manifest.patch.is_empty() {
        bail!("no [patch] fields defined in forge.toml");
    }

    let mut data = std::fs::read(binary_path)?;
    let original_len = data.len();

    // Parse field=value pairs from trailing args
    let fields = parse_field_args(&args.fields)?;

    if fields.is_empty() {
        bail!("no patch fields specified. Available fields: {}",
            manifest.patch.keys().map(|k| format!("--{}", k.replace('_', "-"))).collect::<Vec<_>>().join(", "));
    }

    for (field_name, value) in &fields {
        // Convert CLI name (callback-ip) to manifest name (callback_ip)
        let manifest_name = field_name.replace('-', "_");

        let field_def = manifest
            .patch
            .get(&manifest_name)
            .ok_or_else(|| anyhow::anyhow!("unknown patch field '{field_name}'"))?;

        output::status("Patching", &format!("{field_name} = {value}"));

        // Encode value
        let encoded = types::encode_value(field_def, value)?;

        // Find sentinel
        let offset = scanner::find_sentinel(&data, &field_def.sentinel)?;
        output::verbose(output, &format!("found sentinel at offset {offset:#x}"));

        // Apply patch
        writer::apply_patch(&mut data, offset, &encoded, field_def.size)?;
    }

    // Write patched binary atomically
    atomic_write(binary_path, &data)?;

    // Verify size unchanged
    assert_eq!(data.len(), original_len, "binary size changed during patching");

    output::status("Patched", &format!("{} ({} fields)", args.binary, fields.len()));

    Ok(())
}

fn parse_field_args(args: &[String]) -> Result<Vec<(String, String)>> {
    let mut fields = Vec::new();
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        if let Some(flag) = arg.strip_prefix("--") {
            // Check for --field=value format
            if let Some((name, value)) = flag.split_once('=') {
                fields.push((name.to_string(), value.to_string()));
            } else {
                // Next arg is the value
                let value = iter
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("missing value for --{flag}"))?;
                fields.push((flag.to_string(), value.clone()));
            }
        } else {
            bail!("unexpected argument: {arg} (expected --field-name value)");
        }
    }

    Ok(fields)
}
