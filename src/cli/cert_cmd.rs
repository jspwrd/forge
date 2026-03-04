use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cert;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct CertArgs {
    #[command(subcommand)]
    pub action: CertAction,
}

#[derive(Subcommand)]
pub enum CertAction {
    /// Generate a CA certificate
    Generate {
        /// Common Name
        #[arg(long)]
        cn: String,
        /// Organization
        #[arg(long)]
        org: String,
        /// Output path
        #[arg(long)]
        out: String,
    },
    /// Generate a self-signed certificate
    SelfSign {
        /// Common Name
        #[arg(long)]
        cn: String,
        /// Validity in days
        #[arg(long, default_value = "365")]
        days: u32,
        /// Output path
        #[arg(long)]
        out: String,
    },
    /// Inspect a certificate
    Inspect {
        /// Path to certificate file
        path: String,
    },
}

pub fn execute(args: CertArgs, output: &OutputConfig) -> Result<()> {
    match args.action {
        CertAction::Generate { cn, org, out } => cert::generate(&cn, &org, &out, output),
        CertAction::SelfSign { cn, days, out } => cert::self_sign(&cn, days, &out, output),
        CertAction::Inspect { path } => cert::inspect(&path, output),
    }
}
