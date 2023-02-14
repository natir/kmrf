/* std use */

/* crate use */
use clap::Parser;

use anyhow::Context as _;

/* project use */
use kmrf::*;

fn main() -> error::Result<()> {
    // parse cli
    let params = cli::Command::parse();

    // Setup logger
    stderrlog::new()
        .module(module_path!())
        .quiet(params.quiet())
        .verbosity(params.verbosity())
        .timestamp(params.timestamp())
        .init()
        .context("stderrlog already create a logger")?;

    #[cfg(feature = "parallel")]
    rayon::ThreadPoolBuilder::new()
        .num_threads(params.threads())
        .build_global()?;

    let solid = params.solidity()?;

    let filter = Filter::new(solid, params.ratio(), params.length());

    log::info!("Start filter reads");
    #[cfg(feature = "parallel")]
    filter.filter_fasta(params.input()?, params.output()?, params.record_buffer())?;
    #[cfg(not(feature = "parallel"))]
    filter.filter_fasta(params.input()?, params.output()?)?;
    log::info!("End filter reads");

    Ok(())
}
