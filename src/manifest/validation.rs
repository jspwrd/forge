use anyhow::{Result, bail};

use super::types::ForgeManifest;

pub fn validate_manifest(manifest: &ForgeManifest) -> Result<()> {
    if manifest.project.name.is_empty() {
        bail!("project.name must not be empty");
    }

    if manifest.project.version.is_empty() {
        bail!("project.version must not be empty");
    }

    // Validate patch field types
    let valid_types = ["ipv4", "u8", "u16", "u32", "string"];
    for (name, field) in &manifest.patch {
        if !valid_types.contains(&field.r#type.as_str()) {
            bail!(
                "patch field '{}' has invalid type '{}', expected one of: {}",
                name,
                field.r#type,
                valid_types.join(", ")
            );
        }
        if field.sentinel.is_empty() {
            bail!("patch field '{}' has empty sentinel", name);
        }
        if field.size == 0 {
            bail!("patch field '{}' has zero size", name);
        }
    }

    // Validate target configs
    for (name, target) in &manifest.targets {
        if target.cc.is_none() && target.cxx.is_none() {
            bail!(
                "target '{}' must specify at least one of 'cc' or 'cxx'",
                name
            );
        }
    }

    // Validate package formats
    let valid_formats = ["raw", "dmg", "deb", "msi"];
    for fmt in &manifest.package.formats {
        if !valid_formats.contains(&fmt.as_str()) {
            bail!(
                "invalid package format '{}', expected one of: {}",
                fmt,
                valid_formats.join(", ")
            );
        }
    }

    // Validate build compiler if specified
    if let Some(ref compiler) = manifest.build.compiler {
        let valid_compilers = ["gcc", "clang"];
        if !valid_compilers.contains(&compiler.as_str()) {
            bail!(
                "invalid compiler '{}', expected one of: {}",
                compiler,
                valid_compilers.join(", ")
            );
        }
    }

    Ok(())
}
