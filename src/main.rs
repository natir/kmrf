/*
Copyright (c) 2019 Pierre Marijon <pierre.marijon@hhu.de>

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
use anyhow::Result;
use clap::Clap;

use kmrf::*;

fn main() -> Result<()> {
    let params = cli::Command::parse();

    if let Some(level) = cli::i82level(params.verbosity) {
        env_logger::builder()
            .format_timestamp(None)
            .filter_level(level.to_level_filter())
            .init();
    } else {
        env_logger::Builder::from_env("KMRF_LOG")
            .format_timestamp(None)
            .init();
    }

    let ratio = if let Some(val) = params.ratio {
        val
    } else {
        0.9
    };

    let length = if let Some(val) = params.length {
        val
    } else {
        1000
    };

    if let Some(threads) = params.threads {
        log::info!("Set number of threads to {}", threads);

        set_nb_threads(threads);
    }

    let record_buffer = if let Some(len) = params.record_buffer {
        len
    } else {
        8192
    };

    let solid =
        cli::read_or_compute_solidity(params.solidity, params.kmer, &params.inputs, record_buffer)?;

    kmrf::run_filter(
        params.inputs,
        params.outputs,
        solid,
        ratio,
        length,
        record_buffer,
    )?;

    Ok(())
}
