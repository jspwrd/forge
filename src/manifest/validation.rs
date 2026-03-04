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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::types::*;
    use std::collections::BTreeMap;

    fn minimal_manifest() -> ForgeManifest {
        ForgeManifest {
            project: ProjectConfig {
                name: "test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
            },
            build: BuildConfig::default(),
            targets: BTreeMap::new(),
            dependencies: BTreeMap::new(),
            patch: BTreeMap::new(),
            validate: ValidateConfig::default(),
            scripts: BTreeMap::new(),
            package: PackageConfig::default(),
        }
    }

    #[test]
    fn valid_minimal_manifest() {
        assert!(validate_manifest(&minimal_manifest()).is_ok());
    }

    #[test]
    fn empty_project_name_rejected() {
        let mut m = minimal_manifest();
        m.project.name = String::new();
        let err = validate_manifest(&m).unwrap_err();
        assert!(err.to_string().contains("project.name must not be empty"));
    }

    #[test]
    fn empty_project_version_rejected() {
        let mut m = minimal_manifest();
        m.project.version = String::new();
        let err = validate_manifest(&m).unwrap_err();
        assert!(
            err.to_string()
                .contains("project.version must not be empty")
        );
    }

    #[test]
    fn invalid_patch_type_rejected() {
        let mut m = minimal_manifest();
        m.patch.insert(
            "field1".to_string(),
            PatchFieldDef {
                sentinel: "SENT".to_string(),
                r#type: "float".to_string(),
                size: 4,
            },
        );
        let err = validate_manifest(&m).unwrap_err();
        assert!(err.to_string().contains("invalid type 'float'"));
    }

    #[test]
    fn valid_patch_types_accepted() {
        for ty in &["ipv4", "u8", "u16", "u32", "string"] {
            let mut m = minimal_manifest();
            m.patch.insert(
                "f".to_string(),
                PatchFieldDef {
                    sentinel: "SENT".to_string(),
                    r#type: ty.to_string(),
                    size: 4,
                },
            );
            assert!(validate_manifest(&m).is_ok(), "type '{ty}' should be valid");
        }
    }

    #[test]
    fn empty_sentinel_rejected() {
        let mut m = minimal_manifest();
        m.patch.insert(
            "f".to_string(),
            PatchFieldDef {
                sentinel: String::new(),
                r#type: "u8".to_string(),
                size: 1,
            },
        );
        let err = validate_manifest(&m).unwrap_err();
        assert!(err.to_string().contains("empty sentinel"));
    }

    #[test]
    fn zero_size_patch_rejected() {
        let mut m = minimal_manifest();
        m.patch.insert(
            "f".to_string(),
            PatchFieldDef {
                sentinel: "SENT".to_string(),
                r#type: "u8".to_string(),
                size: 0,
            },
        );
        let err = validate_manifest(&m).unwrap_err();
        assert!(err.to_string().contains("zero size"));
    }

    #[test]
    fn target_without_compiler_rejected() {
        let mut m = minimal_manifest();
        m.targets.insert(
            "bad".to_string(),
            TargetConfig {
                cc: None,
                cxx: None,
                flags: vec![],
                sources: vec![],
                exclude: vec![],
                link: vec![],
                sysroot: None,
                enabled: true,
            },
        );
        let err = validate_manifest(&m).unwrap_err();
        assert!(err.to_string().contains("must specify at least one of"));
    }

    #[test]
    fn invalid_package_format_rejected() {
        let mut m = minimal_manifest();
        m.package.formats = vec!["zip".to_string()];
        let err = validate_manifest(&m).unwrap_err();
        assert!(err.to_string().contains("invalid package format 'zip'"));
    }

    #[test]
    fn invalid_compiler_rejected() {
        let mut m = minimal_manifest();
        m.build.compiler = Some("msvc".to_string());
        let err = validate_manifest(&m).unwrap_err();
        assert!(err.to_string().contains("invalid compiler 'msvc'"));
    }
}
