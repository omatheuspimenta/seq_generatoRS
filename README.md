# seq_generatoRS

`seq_generatoR` is a random DNA sequence generator written in Rust.  

## Install

To install seq_generatoRS, run the following commands:

```bash
git clone git@github.com:omatheuspimenta/seq_generatoRS.git
cd seq_generatoRS
cargo install --path . 
```

#### Tests and Docs


Generate tests and documentation with the following commands:

```bash
cargo doc
cargo test
```

## Usage

`seq_generatoRS` functions as a command-line interface (CLI). 


#### `help`
```bash
seq_generator --help
Generate randon DNA sequences with mutations, variations and bias.

Usage: seq_generator [OPTIONS]

Options:
      --records-number <RECORDS_NUMBER>
          Number of records to be generated [default: 1000]
      --records-length <RECORDS_LENGTH>
          Length of each record [default: 10000]
      --mutation-rate <MUTATION_RATE>
          Mutation rate Rates of evolutionary change in viruses: patterns and determinants Another relationship between mutation rate and genome size was noted by Drake, who proposed a 'universal' genomic mutation rate in DNA microorganisms of 3.4*10^–3 mutations per genome, per genomic replication. [REFERENCE](https://www.nature.com/articles/nrg2323) [default: 0.0034]
      --bias-rate <BIAS_RATE>
          Bias rate [default: 0.0005]
      --variation-rate <VARIATION_RATE>
          Variation rate [default: 0.001]
  -h, --help
          Print help
  -V, --version

```

## Basic example

Simple example: Create 1000 DNA sequences with 10000 nucleotides each, using default settings for mutation, bias, and variation rate in `seq_generatoRS`.

```bash
seq_generator --records-number 1000 --records-length 10000
```

To adjust mutation, bias, or variation rate, ensure values greater than 0 are used, as these parameters represent the maximum value when multiplied by the sequence length in `seq_generatoRS`.

Three files will be generated:

- `mutation_x.txt`: includes information about mutations and their locations.
- `reference_x.fasta`: stores the reference sequence, serving as the basis for mutation insertions.
- `sequences_x.fasta`: encompasses the specified number of sequences (`--records-number`) with mutations, variations, and bias.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
