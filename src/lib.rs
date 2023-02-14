/* local mod */
pub mod cli;
pub mod error;

/* crates use */
#[cfg(feature = "parallel")]
use rayon::prelude::*;

/* local use */

pub struct Filter {
    solid: pcon::solid::Solid,
    ratio: f64,
    length: usize,
}

impl Filter {
    pub fn new(solid: pcon::solid::Solid, ratio: f64, length: usize) -> Self {
        Filter {
            solid,
            ratio,
            length,
        }
    }
}

#[cfg(feature = "parallel")]
impl Filter {
    pub fn filter_fasta<W>(
        &self,
        input: Box<dyn std::io::BufRead>,
        output: W,
        record_buffer: u64,
    ) -> error::Result<()>
    where
        W: std::io::Write,
    {
        let mut reader = noodles::fasta::Reader::new(input);
        let mut iter = reader.records();
        let mut records = Vec::with_capacity(record_buffer as usize);

        let mut writer = noodles::fasta::Writer::new(output);

        let mut end = true;
        while end {
            log::info!("Start populate buffer");
            end = populate_buffer(&mut iter, &mut records, record_buffer);
            log::info!("End populate buffer {}", records.len());

            log::info!("Start perform filter of records");
            let filter: Vec<&noodles::fasta::Record> = records
                .par_iter()
                .map(|record| {
                    if record.sequence().len() > self.solid.k() as usize
                        || record.sequence().len() > self.length
                    {
                        let mut nb_kmer = 0;
                        let mut nb_valid = 0;

                        for cano in cocktail::tokenizer::Canonical::new(
                            record.sequence().as_ref(),
                            self.solid.k(),
                        ) {
                            nb_kmer += 1;

                            if self.solid.get_canonic(cano) {
                                nb_valid += 1;
                            }
                        }

                        let r = (nb_valid as f64) / (nb_kmer as f64);

                        if r >= self.ratio {
                            Some(record)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .flatten()
                .collect();
            log::info!("End perform filter of records");

            log::info!("Start write correct records");
            for record in filter {
                writer.write_record(record)?;
            }
            log::info!("Start write correct records");
        }

        Ok(())
    }
}

#[cfg(feature = "parallel")]
/// Populate record buffer with content of iterator
pub fn populate_buffer(
    iter: &mut noodles::fasta::reader::Records<'_, Box<dyn std::io::BufRead>>,
    records: &mut Vec<noodles::fasta::Record>,
    record_buffer: u64,
) -> bool {
    records.clear();

    for i in 0..record_buffer {
        if let Some(Ok(record)) = iter.next() {
            records.push(record);
        } else {
            records.truncate(i as usize);
            return false;
        }
    }

    true
}

#[cfg(not(feature = "parallel"))]
impl Filter {
    pub fn filter_fasta<R, W>(&self, input: R, output: W) -> error::Result<()>
    where
        R: std::io::BufRead,
        W: std::io::Write,
    {
        let mut reader = noodles::fasta::Reader::new(input);
        let mut records = reader.records();

        let mut writer = noodles::fasta::Writer::new(output);

        while let Some(Ok(record)) = records.next() {
            if record.sequence().len() > self.solid.k() as usize
                || record.sequence().len() > self.length
            {
                let mut nb_kmer = 0;
                let mut nb_valid = 0;

                for cano in
                    cocktail::tokenizer::Canonical::new(record.sequence().as_ref(), self.solid.k())
                {
                    nb_kmer += 1;

                    if self.solid.get_canonic(cano) {
                        nb_valid += 1;
                    }
                }

                let r = (nb_valid as f64) / (nb_kmer as f64);

                if r >= self.ratio {
                    writer.write_record(&record)?;
                }
            }
        }

        Ok(())
    }
}
