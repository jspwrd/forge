use anyhow::Result;

use crate::manifest::types::{ForgeManifest, TargetConfig};

pub struct BuildContext {
    pub compiler: String,
    pub sources: Vec<String>,
    pub excludes: Vec<String>,
    pub includes: Vec<String>,
    pub common_flags: Vec<String>,
    pub profile_flags: Vec<String>,
    pub target_flags: Vec<String>,
    pub link_libs: Vec<String>,
    pub strip: bool,
    pub is_static: bool,
    pub sysroot: Option<String>,
}

impl BuildContext {
    pub fn new(
        manifest: &ForgeManifest,
        target: Option<(&str, &TargetConfig)>,
        release: bool,
    ) -> Result<Self> {
        let build = &manifest.build;

        let compiler = if let Some((_, t)) = &target {
            t.cc
                .clone()
                .or_else(|| build.compiler.clone())
                .unwrap_or_else(|| "gcc".to_string())
        } else {
            build.compiler.clone().unwrap_or_else(|| "gcc".to_string())
        };

        let mut sources = build.sources.clone();
        let mut excludes = Vec::new();
        let mut target_flags = Vec::new();
        let mut link_libs = build.link.clone();
        let mut sysroot = None;

        if let Some((_, t)) = &target {
            if !t.sources.is_empty() {
                sources.extend(t.sources.iter().cloned());
            }
            excludes.extend(t.exclude.iter().cloned());
            target_flags.extend(t.flags.iter().cloned());
            link_libs.extend(t.link.iter().cloned());
            sysroot = t.sysroot.clone();
        }

        if sources.is_empty() {
            sources.push("src/**/*.c".to_string());
        }

        let profile_flags = if release {
            build.flags.release.clone()
        } else {
            build.flags.debug.clone()
        };

        let mut common_flags = build.flags.common.clone();
        if let Some(ref std) = build.standard {
            common_flags.push(format!("-std={std}"));
        }

        Ok(Self {
            compiler,
            sources,
            excludes,
            includes: build.includes.clone(),
            common_flags,
            profile_flags,
            target_flags,
            link_libs,
            strip: build.strip,
            is_static: build.r#static,
            sysroot,
        })
    }

    pub fn all_flags(&self) -> Vec<String> {
        let mut flags = self.common_flags.clone();
        flags.extend(self.profile_flags.iter().cloned());
        flags.extend(self.target_flags.iter().cloned());
        if let Some(ref sysroot) = self.sysroot {
            flags.push(format!("--sysroot={sysroot}"));
        }
        flags
    }

    pub fn link_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        if self.is_static {
            flags.push("-static".to_string());
        }
        flags
    }
}
