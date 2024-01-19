//! Command line interface

use clap::Parser;

/// Struct for command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of records to be generated
    #[arg(long, default_value = "1000")]
    pub records_number: usize,

    /// Length of each record
    #[arg(long, default_value = "10000")]
    pub records_length: usize,

    /// Mutation rate
    /// Rates of evolutionary change in viruses: patterns and determinants
    /// 'Another relationship between mutation rate and genome size was noted by Drake, who proposed
    /// a ‘universal’ genomic mutation rate in DNA microorganisms of 3.4*10^–3 mutations per genome,
    /// per genomic replication.  
    /// [REFERENCE](https://www.nature.com/articles/nrg2323).
    #[arg(long, default_value_t = 0.0034)]
    pub mutation_rate: f64,

    /// Bias rate
    #[arg(long, default_value_t = 0.0005)]
    pub bias_rate: f64,

    /// Variation rate
    #[arg(long, default_value_t = 0.001)]
    pub variation_rate: f64,
}
