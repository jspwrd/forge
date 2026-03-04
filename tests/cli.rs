use predicates::prelude::*;

fn forge_cmd() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("forge").unwrap()
}

#[test]
fn version_flag_prints_version() {
    forge_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("forge"));
}

#[test]
fn help_flag_shows_usage() {
    forge_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Build system for C/C++ cross-compilation",
        ));
}

#[test]
fn no_args_shows_help() {
    forge_cmd()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn build_without_manifest_fails() {
    let dir = tempfile::tempdir().unwrap();
    forge_cmd()
        .arg("build")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("forge.toml"));
}

#[test]
fn new_creates_project() {
    let dir = tempfile::tempdir().unwrap();
    forge_cmd()
        .args(["new", "testproject"])
        .current_dir(dir.path())
        .assert()
        .success();

    let project_dir = dir.path().join("testproject");
    assert!(project_dir.join("forge.toml").exists());
    assert!(project_dir.join("src").is_dir());
}

#[test]
fn init_creates_manifest_in_existing_dir() {
    let dir = tempfile::tempdir().unwrap();
    forge_cmd()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(dir.path().join("forge.toml").exists());
}

#[test]
fn targets_without_manifest_fails() {
    let dir = tempfile::tempdir().unwrap();
    forge_cmd()
        .arg("targets")
        .current_dir(dir.path())
        .assert()
        .failure();
}

#[test]
fn unknown_subcommand_fails() {
    forge_cmd().arg("nonexistent").assert().failure();
}
