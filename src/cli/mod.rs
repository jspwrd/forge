pub mod build_cmd;
pub mod cert_cmd;
pub mod clean_cmd;
pub mod config_cmd;
pub mod init_cmd;
pub mod new_cmd;
pub mod package_cmd;
pub mod patch_cmd;
pub mod scripts_cmd;
pub mod targets_cmd;
pub mod test_cmd;
pub mod uninstall_cmd;
pub mod update_cmd;
pub mod validate_cmd;

use clap::{Parser, Subcommand};

use crate::util::output::OutputConfig;

#[derive(Parser)]
#[command(
    name = "forge",
    version,
    about = "Build system for C/C++ cross-compilation"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[command(flatten)]
    pub global: GlobalOpts,
}

#[derive(Parser, Debug)]
pub struct GlobalOpts {
    /// Enable verbose output
    #[arg(long, short, global = true)]
    pub verbose: bool,

    /// Suppress non-error output
    #[arg(long, short, global = true)]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

impl GlobalOpts {
    pub fn output_config(&self) -> OutputConfig {
        OutputConfig {
            verbose: self.verbose,
            quiet: self.quiet,
            no_color: self.no_color,
        }
    }
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a new forge project
    New(new_cmd::NewArgs),
    /// Initialize forge in an existing directory
    Init(init_cmd::InitArgs),
    /// Build the project
    Build(build_cmd::BuildArgs),
    /// Patch compiled binaries with operator values
    Patch(patch_cmd::PatchArgs),
    /// Validate compiled binaries
    Validate(validate_cmd::ValidateArgs),
    /// Clean build artifacts
    Clean(clean_cmd::CleanArgs),
    /// Run tests
    Test(test_cmd::TestArgs),
    /// Run project scripts
    Scripts(scripts_cmd::ScriptsArgs),
    /// List available build targets
    Targets(targets_cmd::TargetsArgs),
    /// Show or manage configuration
    Config(config_cmd::ConfigArgs),
    /// Manage certificates
    Cert(cert_cmd::CertArgs),
    /// Package build artifacts
    Package(package_cmd::PackageArgs),
    /// Update forge to the latest version
    Update(update_cmd::UpdateArgs),
    /// Uninstall forge from your system
    Uninstall(uninstall_cmd::UninstallArgs),
}

pub fn dispatch(cli: Cli) -> anyhow::Result<()> {
    let output = cli.global.output_config();
    output.apply();

    match cli.command {
        Command::New(args) => new_cmd::execute(args, &output),
        Command::Init(args) => init_cmd::execute(args, &output),
        Command::Build(args) => build_cmd::execute(args, &output),
        Command::Patch(args) => patch_cmd::execute(args, &output),
        Command::Validate(args) => validate_cmd::execute(args, &output),
        Command::Clean(args) => clean_cmd::execute(args, &output),
        Command::Test(args) => test_cmd::execute(args, &output),
        Command::Scripts(args) => scripts_cmd::execute(args, &output),
        Command::Targets(args) => targets_cmd::execute(args, &output),
        Command::Config(args) => config_cmd::execute(args, &output),
        Command::Cert(args) => cert_cmd::execute(args, &output),
        Command::Package(args) => package_cmd::execute(args, &output),
        Command::Update(args) => update_cmd::execute(args, &output),
        Command::Uninstall(args) => uninstall_cmd::execute(args, &output),
    }
}
