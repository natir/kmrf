/*
Copyright (c) 2020 Pierre Marijon <pierre.marijon@hhu.de>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

/* crate use */
use anyhow::{anyhow, Context, Result};
use log::Level;

/* local use */
use crate::error::*;

#[derive(clap::Parser, Debug)]
#[clap(
    version = "0.1",
    author = "Pierre Marijon <pierre.marijon@hhu.de>",
    about = "KMRF: Kmer based Read Filter"
)]
pub struct Command {
    /// solidity bitfield produce by pcon
    #[clap(short = 's', long = "solidity")]
    pub solidity: Option<String>,

    /// fasta file to be correct
    #[clap(short = 'i', long = "inputs")]
    pub inputs: Vec<String>,

    /// path where corrected read was write
    #[clap(short = 'o', long = "outputs")]
    pub outputs: Vec<String>,

    /// if you want choose the minimum abundance you can set this parameter
    #[clap(short = 'a', long = "abundance")]
    pub abundance: Option<u8>,

    /// if a ratio of correct kmer on all kmer is lower than this threshold read is filter out, default 0.8
    #[clap(short = 'r', long = "ratio")]
    pub ratio: Option<f64>,

    /// if a read have length lower than this threshold read is filter out, default 1000
    #[clap(short = 'l', long = "min-length")]
    pub length: Option<usize>,

    /// kmer length if you didn't provide solidity path you must give a kmer length
    #[clap(short = 'k', long = "kmer")]
    pub kmer: Option<u8>,

    /// Number of thread use by br, 0 use all avaible core, default value 0
    #[clap(short = 't', long = "threads")]
    pub threads: Option<usize>,

    /// Number of sequence record load in buffer, default 8192
    #[clap(short = 'b', long = "record_buffer")]
    pub record_buffer: Option<usize>,

    /// verbosity level also control by environment variable BR_LOG if flag is set BR_LOG value is ignored
    #[clap(short = 'v', long = "verbosity", parse(from_occurrences))]
    pub verbosity: i8,
}

pub fn i82level(level: i8) -> Option<Level> {
    match level {
        std::i8::MIN..=0 => None,
        1 => Some(log::Level::Error),
        2 => Some(log::Level::Warn),
        3 => Some(log::Level::Info),
        4 => Some(log::Level::Debug),
        5..=std::i8::MAX => Some(log::Level::Trace),
    }
}

pub fn read_or_compute_solidity(
    solidity_path: Option<String>,
    kmer: Option<u8>,
    inputs: &[String],
    record_buffer_len: usize,
    abundance: Option<u8>,
) -> Result<pcon::solid::Solid> {
    if let Some(solidity_path) = solidity_path {
        let solidity_reader = std::io::BufReader::new(
            std::fs::File::open(&solidity_path)
                .with_context(|| Error::CantOpenFile)
                .with_context(|| anyhow!("File {:?}", solidity_path.clone()))?,
        );

        log::info!("Load solidity file");
        Ok(pcon::solid::Solid::deserialize(solidity_reader)?)
    } else if let Some(kmer) = kmer {
        let mut counter = pcon::counter::Counter::new(kmer);

        log::info!("Start count kmer from input");
        for input in inputs {
            let fasta = std::io::BufReader::new(
                std::fs::File::open(&input)
                    .with_context(|| Error::CantOpenFile)
                    .with_context(|| anyhow!("File {:?}", input.clone()))?,
            );

            counter.count_fasta(fasta, record_buffer_len);
        }
        log::info!("End count kmer from input");

        log::info!("Start build spectrum from count");
        let spectrum = pcon::spectrum::Spectrum::from_counter(&counter);
        log::info!("End build spectrum from count");

        let abun = if let Some(a) = abundance {
            a
        } else {
            log::info!("Start search threshold");
            let abundance = spectrum
                .get_threshold(pcon::spectrum::ThresholdMethod::FirstMinimum, 0.0)
                .ok_or(Error::CantComputeAbundance)?;

            spectrum
                .write_histogram(std::io::stdout(), Some(abundance))
                .with_context(|| anyhow!("Error durring write of kmer histograme"))?;
            println!("If this curve seems bad or minimum abundance choose (marked by *) not apopriate set parameter -a");
            log::info!("End search threshold");
            abundance as u8
        };

        Ok(pcon::solid::Solid::from_counter(&counter, abun))
    } else {
        Err(anyhow!(Error::NoSolidityNoKmer))
    }
}
