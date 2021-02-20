# kmrf 🧬 💻

[![License](https://img.shields.io/badge/license-MIT-green)](https://github.com/natir/kmrf/blob/master/LICENSE)
![CI](https://github.com/natir/kmrf/workflows/CI/badge.svg)
[![Documentation](https://github.com/natir/kmrf/workflows/Documentation/badge.svg)](https://natir.github.io/kmrf/kmrf)
[![CodeCov](https://codecov.io/gh/natir/kmrf/kmrfanch/master/graph/badge.svg)](https://codecov.io/gh/natir/kmrf)

kmrf (Kmer Ratio Filter), pronounced Amere Fine. Estimate read error rate with ratio between 'solid' kmer and all kmer.

## Plot ratio against real error rate

### Reads produce with [Badread](https://github.com/rrwick/Badread)

Need pandas, altair package install in your python environment.

```
cargo run --release -- -i {badread_reads.fasta} -o /dev/null -s {pcon_solid_kmer_file} -vvvv 2>&1 | grep "DEBUG" | cut -d$' ' -f 4 > {out.csv}
./real_error2ratio.py -i {out.csv} -f {badread_reads.fasta} -o out.{png|html}
```

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.45.0.
