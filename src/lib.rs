#![warn(missing_docs)]
//! # Random DNA sequence generator
//!
//! This program generates a random DNA sequence of a given length and a given number of mutations.

/// Return a random DNA sequence of a given length and the mutations.
use itertools::Itertools;
use rand::Rng;
use std::{collections::HashMap, fs, io::Write};

pub mod cli;
pub mod utils;

/// Return a random DNA sequence of a given length and the mutations.
///
/// # Examples
/// ```
/// use seq_generator::dna_generator;
/// let records_length = 100;
/// let records_number = 10;
/// let n_mutations = 3;
/// let variation_len = 2;
/// let n_bias = 2;
/// let result = dna_generator(records_length, records_number, n_mutations, variation_len, n_bias);
/// assert!(result.is_ok());
/// ```
pub fn dna_generator(
    records_length: usize,
    records_number: usize,
    n_mutations: i64,
    variation_len: i64,
    n_bias: i64,
) -> Result<(), ()> {
    // Define parameters
    let dna: &[u8] = b"ACGT";
    let mut rng = rand::thread_rng();
    // File name. First for the reference sequence
    let mut file_name = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("reference_{}.fasta", records_length))
        .unwrap();

    // Generate the reference sequence
    let mut seq_ref: String = (0..records_length)
        .map(|_| {
            let idx = rng.gen_range(0..dna.len());
            dna[idx] as char
        })
        .collect();

    // Write the reference sequence to the file in FASTA format
    file_name.write_all(b">reference\n").unwrap();
    file_name.write_all(seq_ref.as_bytes()).unwrap();

    // Generate the mutations in a HashMap
    let mut mutations_hashmap: HashMap<usize, char> = HashMap::new();
    for _i in 0..n_mutations {
        mutations_hashmap.insert(rng.gen_range(0..records_length), utils::dna_returner(dna));
    }

    // Replace the nucleotides in the reference sequence with the mutations
    mutations_hashmap.iter_mut().for_each(|(k, v)| {
        if seq_ref.as_bytes()[*k] == *v as u8 {
            let mut_value = utils::dna_returner(dna);
            *v = mut_value;
            seq_ref.replace_range(*k..*k + 1, v.to_string().as_str());
        }
        seq_ref.replace_range(*k..*k + 1, &String::from(v.to_string()));
    });

    // File name. Second for the mutations
    file_name = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("mutation_{}.txt", records_length))
        .unwrap();

    // Write the mutations to the file in sorted order
    for k in mutations_hashmap.keys().sorted() {
        file_name
            .write_all(format!("{}:{}\n", k + 1, mutations_hashmap[&k]).as_bytes())
            .unwrap();
    }

    // File name. Third for the mutated sequences
    file_name = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("sequences_{}.fasta", records_length))
        .unwrap();

    // Generate the mutated sequences with variations and biases
    for i in 0..records_number {
        let mut variation_length = rng.gen_range(0..variation_len);
        let begin_variations: String = (0..variation_length)
            .map(|_| {
                let idx = rng.gen_range(0..dna.len());
                dna[idx] as char
            })
            .collect();
        variation_length = rng.gen_range(0..variation_len);
        let end_variations: String = (0..variation_length)
            .map(|_| {
                let idx = rng.gen_range(0..dna.len());
                dna[idx] as char
            })
            .collect();

        let mut seq_mut: String = format!("{begin_variations}{seq_ref}{end_variations}");

        let n_bias_seq = rng.gen_range(0..n_bias);
        let mut bias_hashmap: HashMap<usize, char> = HashMap::new();
        for _i in 0..n_bias_seq {
            bias_hashmap.insert(rng.gen_range(0..seq_mut.len()), utils::dna_returner(dna));
        }

        for (k, v) in bias_hashmap.iter() {
            seq_mut.replace_range(*k..*k + 1, &String::from(v.to_string()));
        }
        file_name
            .write_all(format!(">sequence_{}\n", i + 1).as_bytes())
            .unwrap();
        file_name
            .write_all(format!("{}\n", seq_mut).as_bytes())
            .unwrap();
    }
    Ok(())
}
