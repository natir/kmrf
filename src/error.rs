//! Error struct of project kmrf

/* std use */

/* crate use */
use anyhow;
use thiserror;

/* project use */

/// All error produce by Pcon
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    /// Error in logging system configuration
    #[error(transparent)]
    Log(#[from] log::SetLoggerError),

    /// Error in rayon thread pool build
    #[cfg(feature = "parallel")]
    #[error(transparent)]
    RayonThreadPool(#[from] rayon::ThreadPoolBuildError),

    /// Cost io error
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// Error if cli parameter isn't set propely
    #[error("You must provide a solidity path '-s' or a kmer length '-k'")]
    NoSolidityNoKmer,

    /// Error durring search of minimal abundance
    #[error("Can't compute minimal abundance")]
    CantComputeAbundance,
}

/// Alias of result
pub type Result<T> = anyhow::Result<T>;
