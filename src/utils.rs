//! # Utils
//!
//! This module contains utility functions for the crate.

use rand::Rng;
/// Return a random nucleotide from ACTG values.
///
/// # Examples
///
/// ```
/// use rand::Rng;
/// let dna: &[u8] = b"ACGT";
/// let dna_char = seq_generator::utils::dna_returner(dna);
/// assert!(dna.contains(&(dna_char as u8)));
/// ```
pub fn dna_returner(dna: &[u8]) -> char {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..dna.len());
    dna[idx] as char
}
