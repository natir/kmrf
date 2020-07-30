# kmrf 🧬 💻

kmrf (Kmer Ratio Filter), pronounced Amere Fine. Estimate read error rate with ratio between 'solid' kmer and all kmer.

## Plot ratio against real error rate

### Reads produce with [Badread](https://github.com/rrwick/Badread)

Need pandas, altair package install in your python environment.

```
cargo run --release -- -i {badread_reads.fasta} -o /dev/null -s {pcon_solid_kmer_file} -vvv 2>&1 | grep "INFO" | cut -d$' ' -f 4 > out.csv
./real_error2ratio.py -i {out.csv} -f {badread_reads.fasta} -o out.{png|html}
```


