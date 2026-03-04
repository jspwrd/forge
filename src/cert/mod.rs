mod openssl;

use anyhow::Result;

use crate::util::output::OutputConfig;

pub fn generate(cn: &str, org: &str, out: &str, output: &OutputConfig) -> Result<()> {
    openssl::generate_ca(cn, org, out, output)
}

pub fn self_sign(cn: &str, days: u32, out: &str, output: &OutputConfig) -> Result<()> {
    openssl::self_sign(cn, days, out, output)
}

pub fn inspect(path: &str, output: &OutputConfig) -> Result<()> {
    openssl::inspect_cert(path, output)
}
