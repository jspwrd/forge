use std::collections::HashMap;
use std::path::Path;

use crate::manifest::types::ForgeManifest;

pub fn build_env(manifest: &ForgeManifest, project_dir: &Path) -> HashMap<String, String> {
    let mut env = HashMap::new();

    env.insert(
        "FORGE_PROJECT_NAME".to_string(),
        manifest.project.name.clone(),
    );
    env.insert(
        "FORGE_PROJECT_VERSION".to_string(),
        manifest.project.version.clone(),
    );
    env.insert(
        "FORGE_PROJECT_DIR".to_string(),
        project_dir.display().to_string(),
    );
    env.insert(
        "FORGE_BIN_DIR".to_string(),
        project_dir.join("bin").display().to_string(),
    );
    env.insert(
        "FORGE_SRC_DIR".to_string(),
        project_dir.join("src").display().to_string(),
    );

    if let Some(ref desc) = manifest.project.description {
        env.insert("FORGE_PROJECT_DESCRIPTION".to_string(), desc.clone());
    }

    env
}
