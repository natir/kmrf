/* std use */
use std::io::Read as _;

/* crate use */

/* local use */
use crate::error;

#[derive(clap::Parser, Debug)]
#[clap(
    version = "0.1",
    author = "Pierre Marijon <pierre@marijon.fr>",
    about = "KMRF: Kmer based Read Filter"
)]
#[clap(propagate_version = true)]
pub struct Command {
    /* Generic argument */
    #[cfg(feature = "parallel")]
    /// Number of theard use 0 use all avaible core, default value 0
    #[clap(short = 't', long = "threads")]
    threads: Option<usize>,

    /// Silence all output
    #[clap(short = 'q', long = "quiet")]
    quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[clap(short = 'v', long = "verbosity", action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Timestamp (sec, ms, ns, none)
    #[clap(short = 'T', long = "timestamp")]
    ts: Option<stderrlog::Timestamp>,

    /* specific argument */
    /// Path to solidity bitfield produce by pcon
    #[clap(short = 's', long = "solidity")]
    pub solidity: Option<std::path::PathBuf>,

    /// Path to fasta file to be correct
    #[clap(short = 'i', long = "inputs")]
    pub inputs: Option<Vec<std::path::PathBuf>>,

    /// Path where corrected read was write in fasta format
    #[clap(short = 'o', long = "outputs")]
    pub outputs: Option<std::path::PathBuf>,

    /// If you want choose the minimum abundance you can set this parameter
    #[clap(short = 'a', long = "abundance")]
    pub abundance: Option<pcon::CountTypeNoAtomic>,

    /// If a ratio of correct kmer on all kmer is lower than this threshold read is filter out, default 0.8
    #[clap(short = 'r', long = "ratio")]
    pub ratio: Option<f64>,

    /// If a read have length lower than this threshold read is filter out, default 1000
    #[clap(short = 'l', long = "min-length")]
    pub length: Option<usize>,

    /// Kmer length if you didn't provide solidity path you must give a kmer length
    #[clap(short = 'k', long = "kmer-size")]
    pub kmer_size: Option<u8>,

    /// Number of sequence record load in buffer, default 8192
    #[clap(short = 'b', long = "record_buffer")]
    pub record_buffer: Option<u64>,
}

impl Command {
    #[cfg(feature = "parallel")]
    /// Get number of thread
    pub fn threads(&self) -> usize {
        self.threads.unwrap_or(0)
    }

    /// Get verbosity level
    pub fn verbosity(&self) -> usize {
        self.verbosity as usize
    }

    /// Get quiet
    pub fn quiet(&self) -> bool {
        self.quiet
    }

    /// Get timestamp granularity
    pub fn timestamp(&self) -> stderrlog::Timestamp {
        self.ts.unwrap_or(stderrlog::Timestamp::Off)
    }

    /// Get inputs
    pub fn input(&self) -> error::Result<Box<dyn std::io::BufRead>> {
        match &self.inputs {
            None => Ok(Box::new(std::io::stdin().lock())),
            Some(paths) => {
                let mut handle: Box<dyn std::io::Read> = Box::new(std::io::Cursor::new(vec![]));

                for path in paths {
                    let (file, _compression) =
                        niffler::get_reader(Box::new(std::fs::File::open(path)?))?;
                    handle = Box::new(handle.chain(file));
                }

                Ok(Box::new(std::io::BufReader::new(handle)))
            }
        }
    }

    /// Get output
    pub fn output(&self) -> error::Result<Box<dyn std::io::Write + std::marker::Send>> {
        match &self.outputs {
            None => Ok(Box::new(std::io::BufWriter::new(std::io::stdout()))),
            Some(path) => create(path),
        }
    }

    /// Get abundance
    pub fn abundance(&self) -> pcon::CountTypeNoAtomic {
        self.abundance.unwrap_or(0)
    }

    /// Get ratio
    pub fn ratio(&self) -> f64 {
        self.ratio.unwrap_or(0.8)
    }

    /// Get length
    pub fn length(&self) -> usize {
        self.length.unwrap_or(1000)
    }

    /// Get record_buffer
    pub fn record_buffer(&self) -> u64 {
        self.record_buffer.unwrap_or(8192)
    }

    /// Read or compute solidity
    pub fn solidity(&self) -> error::Result<pcon::solid::Solid> {
        if let Some(solidity_path) = &self.solidity {
            log::info!("Load solidity file");
            pcon::solid::Solid::from_path(solidity_path)
        } else if let Some(kmer_size) = self.kmer_size {
            let mut counter = pcon::counter::Counter::<pcon::CountType>::new(kmer_size);

            log::info!("Start count kmer from input");
            counter.count_fasta(self.input()?, self.record_buffer());
            log::info!("End count kmer from input");

            log::info!("Start build spectrum from count");
            #[cfg(feature = "parallel")]
            let spectrum = pcon::spectrum::Spectrum::from_count(&counter.raw_noatomic());
            #[cfg(not(feature = "parallel"))]
            let spectrum = pcon::spectrum::Spectrum::from_count(&counter.raw());
            log::info!("End build spectrum from count");

            let abundance = if let Some(abundance) = self.abundance {
                abundance
            } else {
                log::info!("Start search threshold");
                let value = spectrum
                    .get_threshold(pcon::spectrum::ThresholdMethod::FirstMinimum, 0.0)
                    .ok_or(error::Error::CantComputeAbundance)?;
                log::info!("End search threshold");

                value
            };

            #[cfg(feature = "parallel")]
            let solid =
                pcon::solid::Solid::from_count(kmer_size, counter.raw_noatomic(), abundance);
            #[cfg(not(feature = "parallel"))]
            let solid = pcon::solid::Solid::from_count(kmer_size, counter.raw(), abundance);

            Ok(solid)
        } else {
            Err(error::Error::NoSolidityNoKmer.into())
        }
    }
}

fn create<P>(path: P) -> error::Result<Box<dyn std::io::Write + std::marker::Send>>
where
    P: std::convert::AsRef<std::path::Path>,
{
    let file = std::fs::File::create(path)?;
    let buffer = std::io::BufWriter::new(file);
    let boxed = Box::new(buffer);

    Ok(boxed)
}
