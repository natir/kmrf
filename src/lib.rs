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

/* local mod */
pub mod cli;
pub mod error;

/* crates use */
use anyhow::{anyhow, Context, Result};
use rayon::prelude::*;

/* local use */
use error::*;

pub fn run_filter(
    inputs: Vec<String>,
    outputs: Vec<String>,
    solid: pcon::solid::Solid,
    threshold: f64,
    record_buffer_len: usize,
) -> Result<()> {
    for (input, output) in inputs.iter().zip(outputs) {
        log::info!("Read file {} write in {}", input, output);

        let reader = bio::io::fasta::Reader::new(std::io::BufReader::new(
            std::fs::File::open(input)
                .with_context(|| Error::CantOpenFile)
                .with_context(|| anyhow!("File {}", input.clone()))?,
        ));

        let mut write = bio::io::fasta::Writer::new(std::io::BufWriter::new(
            std::fs::File::create(&output)
                .with_context(|| Error::CantCreateFile)
                .with_context(|| anyhow!("File {}", output.clone()))?,
        ));

        let mut iter = reader.records();
        let mut records = Vec::with_capacity(record_buffer_len);

        let mut end = false;
        loop {
            for _ in 0..record_buffer_len {
                if let Some(Ok(record)) = iter.next() {
                    records.push(record);
                } else {
                    end = true;
                    break;
                }
            }

            log::info!("Buffer len: {}", records.len());

            let keeped: Vec<_> = records
                .par_iter()
                .filter_map(|record| {
                    if record.seq().len() < solid.k as usize {
                        return None;
                    }

                    let mut nb_kmer = 0;
                    let mut nb_valid = 0;

                    for cano in cocktail::tokenizer::Canonical::new(record.seq(), solid.k) {
                        nb_kmer += 1;

                        if solid.get_canonic(cano) {
                            nb_valid += 1;
                        }
                    }

                    let ratio = (nb_valid as f64) / (nb_kmer as f64);

                    if ratio >= threshold {
                        Some(record)
                    } else {
                        None
                    }
                })
                .collect();

            for record in keeped {
                write
                    .write_record(&record)
                    .with_context(|| Error::ErrorDurringWrite)
                    .with_context(|| anyhow!("File {}", output.clone()))?
            }

            records.clear();

            if end {
                break;
            }
        }
    }

    Ok(())
}

/// Set the number of threads use by count step
pub fn set_nb_threads(nb_threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(nb_threads)
        .build_global()
        .unwrap();
}
