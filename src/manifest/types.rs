use serde::Deserialize;
use std::collections::BTreeMap;

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct ForgeManifest {
    pub project: ProjectConfig,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub targets: BTreeMap<String, TargetConfig>,
    #[serde(default)]
    pub dependencies: BTreeMap<String, DependencyConfig>,
    #[serde(default)]
    pub patch: BTreeMap<String, PatchFieldDef>,
    #[serde(default)]
    pub validate: ValidateConfig,
    #[serde(default)]
    pub scripts: BTreeMap<String, String>,
    #[serde(default)]
    pub package: PackageConfig,
}

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct BuildConfig {
    pub compiler: Option<String>,
    pub standard: Option<String>,
    #[serde(default)]
    pub r#static: bool,
    #[serde(default)]
    pub strip: bool,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub includes: Vec<String>,
    #[serde(default)]
    pub link: Vec<String>,
    #[serde(default)]
    pub flags: BuildFlags,
}

#[derive(Debug, Default, Deserialize)]
pub struct BuildFlags {
    #[serde(default)]
    pub common: Vec<String>,
    #[serde(default)]
    pub release: Vec<String>,
    #[serde(default)]
    pub debug: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TargetConfig {
    pub cc: Option<String>,
    pub cxx: Option<String>,
    #[serde(default)]
    pub flags: Vec<String>,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub link: Vec<String>,
    pub sysroot: Option<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct PatchFieldDef {
    pub sentinel: String,
    pub r#type: String,
    pub size: usize,
}

#[derive(Debug, Default, Deserialize)]
pub struct ValidateConfig {
    #[serde(default)]
    pub no_debug_symbols: bool,
    #[serde(default)]
    pub no_plaintext_strings: Vec<String>,
    #[serde(default)]
    pub no_compiler_watermarks: bool,
    pub max_binary_size: Option<String>,
    #[serde(default)]
    pub no_rpath: bool,
    #[serde(default)]
    pub no_buildpaths: bool,
}

#[derive(Debug, Deserialize)]
pub struct DependencyConfig {
    pub path: String,
    pub version: Option<String>,
    #[serde(default)]
    pub header_only: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct PackageConfig {
    #[serde(default)]
    pub formats: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_manifest() {
        let toml_str = r#"
[project]
name = "test"
version = "1.0.0"
"#;
        let manifest: ForgeManifest = toml::from_str(toml_str).unwrap();
        assert_eq!(manifest.project.name, "test");
        assert_eq!(manifest.project.version, "1.0.0");
        assert!(manifest.targets.is_empty());
        assert!(manifest.patch.is_empty());
    }

    #[test]
    fn parse_full_manifest() {
        let toml_str = r#"
[project]
name = "myproject"
version = "2.0.0"
description = "A test project"

[build]
compiler = "gcc"
standard = "c11"
static = true
strip = true
sources = ["src/**/*.c"]
includes = ["include"]
link = ["pthread"]

[build.flags]
common = ["-Wall"]
release = ["-O2"]
debug = ["-g"]

[targets.linux-x86]
cc = "gcc"
flags = ["-m32"]

[patch.callback_ip]
sentinel = "AAAA_IP_AAAA"
type = "ipv4"
size = 16

[validate]
no_debug_symbols = true
no_plaintext_strings = ["secret"]
max_binary_size = "1MB"

[package]
formats = ["raw", "deb"]
"#;
        let manifest: ForgeManifest = toml::from_str(toml_str).unwrap();
        assert_eq!(manifest.project.name, "myproject");
        assert_eq!(
            manifest.project.description.as_deref(),
            Some("A test project")
        );
        assert_eq!(manifest.build.compiler.as_deref(), Some("gcc"));
        assert!(manifest.build.r#static);
        assert!(manifest.build.strip);
        assert_eq!(manifest.build.sources, vec!["src/**/*.c"]);
        assert_eq!(manifest.targets.len(), 1);
        assert!(manifest.targets.contains_key("linux-x86"));
        assert_eq!(manifest.patch.len(), 1);
        assert_eq!(manifest.patch["callback_ip"].sentinel, "AAAA_IP_AAAA");
        assert!(manifest.validate.no_debug_symbols);
        assert_eq!(manifest.package.formats, vec!["raw", "deb"]);
    }

    #[test]
    fn target_enabled_defaults_to_true() {
        let toml_str = r#"
[project]
name = "test"
version = "1.0.0"

[targets.linux]
cc = "gcc"
"#;
        let manifest: ForgeManifest = toml::from_str(toml_str).unwrap();
        assert!(manifest.targets["linux"].enabled);
    }

    #[test]
    fn target_can_be_disabled() {
        let toml_str = r#"
[project]
name = "test"
version = "1.0.0"

[targets.linux]
cc = "gcc"
enabled = false
"#;
        let manifest: ForgeManifest = toml::from_str(toml_str).unwrap();
        assert!(!manifest.targets["linux"].enabled);
    }
}
