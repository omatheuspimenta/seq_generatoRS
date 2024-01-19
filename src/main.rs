///! Main file for the seq_generator binary.
use clap::Parser;

use seq_generator::cli::Args;
use seq_generator::dna_generator;

fn main() {
    (|| {
    let args = Args::parse();

    let n_mutations = (args.mutation_rate * args.records_length as f64) as i64;
    if n_mutations == 0 {
        return Err(eprintln!("Error: --mutation-rate is too low, no mutations will be generated. The mutation rate must be greater than 0.\n \
        Tip: The max number of mutations is obtained by multiplying the length of the sequence by the mutation rate."));
    }
    let n_bias = (args.bias_rate * args.records_length as f64) as i64;
    if n_bias == 0 {
        return Err(eprintln!("Error: --bias-rate is too low, no bias will be generated. The bias rate must be greater than 0.\n \
        Tip: The max number of bias is obtained by multiplying the length of the sequence by the bias rate."));
    }
    let variation_len = (args.variation_rate * args.records_length as f64) as i64;
    if variation_len == 0 {
        return Err(eprintln!("Error: --variation-rate is too low, no variation will be generated. The variation rate must be greater than 0.\n \
        Tip: The max number of variation is obtained by multiplying the length of the sequence by the variation rate."));
    }

    dna_generator(
        args.records_length,
        args.records_number,
        n_mutations,
        variation_len,
        n_bias,
    )
    })()
    .map_err(|()| std::process::exit(1))
    .ok();
}
