mod compiler;
mod context;
mod dependency;
mod incremental;
mod toolchain;

use std::path::Path;

use anyhow::Result;

use crate::cli::build_cmd::BuildArgs;
use crate::manifest;
use crate::util::output::{self, OutputConfig};

pub fn execute_build(args: BuildArgs, output: &OutputConfig) -> Result<()> {
    let manifest = manifest::load_from_cwd()?;
    let project_dir = manifest::manifest_dir()?;
    let bin_dir = project_dir.join("bin");

    crate::util::fs::ensure_dir(&bin_dir)?;

    if args.all_targets {
        build_all_targets(&manifest, &project_dir, &bin_dir, &args, output)?;
    } else if let Some(ref target_name) = args.target {
        build_single_target(
            &manifest,
            target_name,
            &project_dir,
            &bin_dir,
            &args,
            output,
        )?;
    } else {
        build_native(&manifest, &project_dir, &bin_dir, &args, output)?;
    }

    Ok(())
}

fn build_all_targets(
    manifest: &manifest::types::ForgeManifest,
    project_dir: &Path,
    bin_dir: &Path,
    args: &BuildArgs,
    output: &OutputConfig,
) -> Result<()> {
    let mut success = 0;
    let mut failed = 0;

    for (name, target) in &manifest.targets {
        if !target.enabled {
            output::status("Skipping", &format!("disabled target '{name}'"));
            continue;
        }

        output::status("Building", &format!("target '{name}'"));
        let ctx = context::BuildContext::new(manifest, Some((name, target)), args.release)?;

        match do_build(
            &ctx,
            project_dir,
            bin_dir,
            &manifest.project.name,
            Some(name),
            args,
            output,
        ) {
            Ok(()) => {
                success += 1;
                output::status("Finished", &format!("target '{name}'"));
            }
            Err(e) => {
                failed += 1;
                output::error(&format!("target '{name}': {e}"));
            }
        }
    }

    output::status(
        "Summary",
        &format!("Compiled {success} targets ({failed} failed)"),
    );

    if failed > 0 {
        anyhow::bail!("{failed} target(s) failed to build");
    }

    Ok(())
}

fn build_single_target(
    manifest: &manifest::types::ForgeManifest,
    target_name: &str,
    project_dir: &Path,
    bin_dir: &Path,
    args: &BuildArgs,
    output: &OutputConfig,
) -> Result<()> {
    let target = manifest
        .targets
        .get(target_name)
        .ok_or_else(|| anyhow::anyhow!("unknown target '{target_name}'"))?;

    output::status("Building", &format!("target '{target_name}'"));
    let ctx = context::BuildContext::new(manifest, Some((target_name, target)), args.release)?;
    do_build(
        &ctx,
        project_dir,
        bin_dir,
        &manifest.project.name,
        Some(target_name),
        args,
        output,
    )?;
    output::status("Finished", &format!("target '{target_name}'"));
    Ok(())
}

fn build_native(
    manifest: &manifest::types::ForgeManifest,
    project_dir: &Path,
    bin_dir: &Path,
    args: &BuildArgs,
    output: &OutputConfig,
) -> Result<()> {
    output::status("Building", &format!("project '{}'", manifest.project.name));
    let ctx = context::BuildContext::new(manifest, None, args.release)?;
    do_build(
        &ctx,
        project_dir,
        bin_dir,
        &manifest.project.name,
        None,
        args,
        output,
    )?;
    output::status("Finished", &format!("project '{}'", manifest.project.name));
    Ok(())
}

fn do_build(
    ctx: &context::BuildContext,
    project_dir: &Path,
    bin_dir: &Path,
    project_name: &str,
    target_name: Option<&str>,
    args: &BuildArgs,
    output: &OutputConfig,
) -> Result<()> {
    let obj_subdir = target_name.unwrap_or("native");
    let obj_dir = bin_dir.join("obj").join(obj_subdir);
    crate::util::fs::ensure_dir(&obj_dir)?;

    // Discover sources
    let sources = dependency::discover_sources(project_dir, &ctx.sources, &ctx.excludes)?;
    output::verbose(output, &format!("found {} source files", sources.len()));

    if sources.is_empty() {
        anyhow::bail!("no source files found matching patterns");
    }

    // Load incremental cache
    let cache_path = bin_dir.join(".forge-cache");
    let mut cache = incremental::BuildCache::load(&cache_path);

    // Compile each source
    let mut objects = Vec::new();
    let mut compiled = 0usize;
    let mut skipped = 0usize;

    for source in &sources {
        let obj_path = compiler::object_path(&obj_dir, source, project_dir);
        crate::util::fs::ensure_dir(obj_path.parent().expect("object path has parent"))?;

        let cache_key = incremental::cache_key(source, &ctx.all_flags())?;

        if !args.force && cache.is_up_to_date(&cache_key) {
            skipped += 1;
            objects.push(obj_path);
            continue;
        }

        output::verbose(output, &format!("compiling {}", source.display()));
        compiler::compile_object(
            &ctx.compiler,
            source,
            &obj_path,
            &ctx.all_flags(),
            &ctx.includes,
        )?;
        cache.update(&cache_key, source)?;
        compiled += 1;
        objects.push(obj_path);
    }

    output::status(
        "Compiled",
        &format!("{compiled} files ({skipped} unchanged)"),
    );

    // Link
    let output_name = match target_name {
        Some(t) => format!("{project_name}-{t}"),
        None => project_name.to_string(),
    };
    let output_path = bin_dir.join(&output_name);

    output::verbose(
        output,
        &format!("linking {} -> {}", objects.len(), output_path.display()),
    );
    compiler::link(
        &ctx.compiler,
        &objects,
        &output_path,
        &ctx.link_flags(),
        &ctx.link_libs,
    )?;

    // Strip if configured
    if ctx.strip {
        output::verbose(output, "stripping binary");
        compiler::strip_binary(&output_path)?;
    }

    // Save cache
    cache.save(&cache_path)?;

    // Log
    let log_path = bin_dir.join("build.log");
    let log_entry = format!(
        "[{}] built {} ({} compiled, {} skipped)\n",
        chrono_lite(),
        output_name,
        compiled,
        skipped,
    );
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?
        .write_all(log_entry.as_bytes())?;

    output::status("Produced", &output_path.display().to_string());
    Ok(())
}

fn chrono_lite() -> String {
    // Simple timestamp without pulling in chrono
    use std::time::SystemTime;
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", d.as_secs())
}

use std::io::Write;
