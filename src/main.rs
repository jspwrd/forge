use clap::Parser;

use forge::cli::{Cli, dispatch};
use forge::error::ForgeError;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = dispatch(cli) {
        let code = if let Some(forge_err) = e.downcast_ref::<ForgeError>() {
            forge::util::output::error(&format!("{forge_err}"));
            forge_err.exit_code()
        } else {
            forge::util::output::error(&format!("{e:#}"));
            // Check for config-related errors
            let msg = format!("{e:#}");
            if msg.contains("forge.toml") || msg.contains("manifest") || msg.contains("parse") {
                2
            } else {
                1
            }
        };
        std::process::exit(code);
    }
}
