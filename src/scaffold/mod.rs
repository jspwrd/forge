mod templates;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use anyhow::{Result, bail};

use crate::util::fs::ensure_dir;
use crate::util::output::{self, OutputConfig};

pub fn create_project(name: &str, output: &OutputConfig) -> Result<()> {
    let root = Path::new(name);

    if root.exists() {
        bail!("directory '{}' already exists", name);
    }

    output::status("Creating", &format!("project '{name}'"));

    // Create directory structure
    let dirs = [
        "",
        "src",
        "src/platform",
        "src/crypto",
        "scripts",
        "vendor",
        "targets",
        "certs",
        "bin",
        "tests",
    ];

    for dir in &dirs {
        ensure_dir(&root.join(dir))?;
    }

    // Write files
    write_file(root, "forge.toml", &templates::forge_toml(name))?;
    write_file(root, "src/main.c", &templates::main_c(name))?;
    write_file(root, "src/config.h", templates::config_h())?;
    write_file(root, "src/comms.h", templates::comms_h())?;
    write_file(root, "src/comms.c", templates::comms_c())?;
    write_file(root, "src/platform/platform.h", templates::platform_h())?;
    write_file(root, "src/crypto/crypto.h", templates::crypto_h())?;

    write_script(root, "scripts/deploy.sh", templates::deploy_sh())?;
    write_script(root, "scripts/cleanup.sh", templates::cleanup_sh())?;
    write_script(root, "scripts/listener.sh", templates::listener_sh())?;

    write_file(root, "tests/test_comms.c", templates::test_comms_c())?;
    write_script(root, "tests/test_scripts.sh", templates::test_scripts_sh())?;

    write_file(root, ".gitignore", templates::gitignore())?;
    write_file(root, "README.md", &templates::readme(name))?;

    output::status("Created", &format!("project '{name}' with forge.toml"));
    output::verbose(output, &format!("project root: {}", root.display()));

    Ok(())
}

pub fn init_project(output: &OutputConfig) -> Result<()> {
    let manifest_path = Path::new("forge.toml");

    if manifest_path.exists() {
        bail!("forge.toml already exists in current directory");
    }

    let name = std::env::current_dir()?
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "project".to_string());

    output::status("Initializing", &format!("forge project '{name}'"));

    fs::write(manifest_path, templates::forge_toml(&name))?;

    // Create directories that don't exist
    let dirs = [
        "src", "scripts", "vendor", "targets", "certs", "bin", "tests",
    ];
    for dir in &dirs {
        let path = Path::new(dir);
        if !path.exists() {
            ensure_dir(path)?;
        }
    }

    output::status("Initialized", &format!("project '{name}'"));
    output::verbose(output, "created forge.toml and project directories");

    Ok(())
}

fn write_file(root: &Path, rel_path: &str, content: &str) -> Result<()> {
    let path = root.join(rel_path);
    fs::write(&path, content)?;
    Ok(())
}

fn write_script(root: &Path, rel_path: &str, content: &str) -> Result<()> {
    let path = root.join(rel_path);
    fs::write(&path, content)?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;
    Ok(())
}
