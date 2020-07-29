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
use clap::Clap;
use log::Level;

#[derive(Clap, Debug)]
#[clap(
    version = "0.1",
    author = "Pierre Marijon <pierre.marijon@hhu.de>",
    about = "KmRF: Kmer based Read Filter"
)]
pub struct Command {
    #[clap(
        short = "s",
        long = "solidity",
        about = "solidity bitfield produce by pcon"
    )]
    pub solidity: String,

    #[clap(short = "i", long = "inputs", about = "fasta file to be correct")]
    pub inputs: Vec<String>,

    #[clap(
        short = "o",
        long = "outputs",
        about = "path where corrected read was write"
    )]
    pub outputs: Vec<String>,

    #[clap(
        short = "t",
        long = "threshold",
        about = "if a ratio of correct kmer on all kmer is lower than this threshold read is filter out, default 0.8"
    )]
    pub threshold: Option<f64>,

    #[clap(
        short = "v",
        long = "verbosity",
        parse(from_occurrences),
        about = "verbosity level also control by environment variable BR_LOG if flag is set BR_LOG value is ignored"
    )]
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
