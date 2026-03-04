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
