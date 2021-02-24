# kmrf 🧬 💻

[![License](https://img.shields.io/badge/license-MIT-green)](https://github.com/natir/kmrf/blob/master/LICENSE)
![CI](https://github.com/natir/kmrf/workflows/CI/badge.svg)
[![Documentation](https://github.com/natir/kmrf/workflows/Documentation/badge.svg)](https://natir.github.io/kmrf/kmrf)
[![CodeCov](https://codecov.io/gh/natir/kmrf/kmrfanch/master/graph/badge.svg)](https://codecov.io/gh/natir/kmrf)

kmrf (Kmer Ratio Filter), pronounced Amere Fine. Estimate read error rate with ratio ![|solid kmer| / |total kmer|](https://render.githubusercontent.com/render/math?math=\frac{\left\lVert%20solid%20kmer%20\right\rVert}{\left\lVert%20all%20kmer%20\right\rVert}).

- [Instalation](#instalation)
- [Usage](#usage)
- [Evaluation](#evaluation)
- [Minimum supported Rust version](#minimum-supported-rust-version)
- [Citation](#citation)

## Instalation

If you haven't a rust environment you can use [rustup](https://rustup.rs/) or your package manager.

### With cargo

Recommended solution.

```
cargo install --git https://github.com/natir/kmrf.git
```

### With source

```
git clone https://github.com/natir/kmrf.git
cd kmrf
cargo install --path .
```

## Usage

WIP

## Evaluation

To plot ![|solid kmer| / |total kmer|](https://render.githubusercontent.com/render/math?math=\frac{\left\lVert%20solid%20kmer%20\right\rVert}{\left\lVert%20all%20kmer%20\right\rVert}) against [badread](https://github.com/rrwick/Badread) *real* error rate. You need badread reads in fasta format, pandas, altair package in your python environment and run this in kmrf directory:

```
cargo run --release -- -i {badread_reads.fasta} -o /dev/null -s {pcon_solid_kmer_file} -vvvv 2>&1 | grep "DEBUG" | cut -d$' ' -f 4 > {out.csv}
./script/real_error2ratio.py -i {out.csv} -f {badread_reads.fasta} -o out.{png|html}
```

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.45.0.

## Citation

WIP
