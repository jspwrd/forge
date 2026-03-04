use anyhow::Result;
use clap::Parser;

use crate::test_runner;
use crate::util::output::OutputConfig;

#[derive(Parser)]
pub struct TestArgs;

pub fn execute(_args: TestArgs, output: &OutputConfig) -> Result<()> {
    test_runner::run_tests(output)
}
