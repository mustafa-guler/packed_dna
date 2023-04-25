// TODO: implement a nucleotide counter
//
// command line argument parsing has been provided
// you must use the PackedDna struct you previously implemented
// if there is any functionality you would like to add to PackedDna feel free to do so in the DNA
// crate
//
// If run with `nuccount --dna ACGTTT" it should print the following to stdout:
// ```
// Input: ACGTTT
//
// A: 1
// C: 1
// G: 1
// T: 3
// ```
//
// be sure to exit with informative error messages if the input is invalid
use dna::{Nuc, PackedDna};
use std::{convert::TryFrom, iter::FromIterator, str::FromStr};
use structopt::StructOpt;
/// Count the number of occurrences of each nucleotide in the provided DNA.
#[derive(Debug, StructOpt)]
struct Opts {
    /// The DNA sequence for which we should retrieve a nucleotide count.
    ///
    /// It is case insensitive but only nucleotides A, C, G and T are supported.
    #[structopt(short = "d", long, required = true)]
    dna: String,
}

fn main() {
    let opts = Opts::from_args();
    let dna = opts.dna;
    let size = dna.len();

    println!("Input: {}", dna);
    let seq = dna
        .chars()
        .map(Nuc::try_from)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let dna_from_iter = PackedDna::from_iter(seq.clone().into_iter());

    let mut count_a = 0;
    let mut count_c = 0;
    let mut count_g = 0;
    let mut count_t = 0;
    {
        println!("Result from FromIter: ");

        for i in 0..size {
            let a = PackedDna::get(&dna_from_iter, i);
            match a {
                Nuc::A => count_a += 1,
                Nuc::C => count_c += 1,
                Nuc::G => count_g += 1,
                Nuc::T => count_t += 1,
            }
        }

        println!("A: {}", count_a);
        println!("C: {}", count_c);
        println!("G: {}", count_g);
        println!("T: {}", count_t);
    }

    let dna_from_str = PackedDna::from_str(&dna).unwrap();
    {
        println!("Result fromFromStr: ");
        let mut count_a = 0;
        let mut count_c = 0;
        let mut count_g = 0;
        let mut count_t = 0;

        for i in 0..size {
            let a = PackedDna::get(&dna_from_str, i);
            match a {
                Nuc::A => count_a += 1,
                Nuc::C => count_c += 1,
                Nuc::G => count_g += 1,
                Nuc::T => count_t += 1,
            }
        }

        println!("A: {}", count_a);
        println!("C: {}", count_c);
        println!("G: {}", count_g);
        println!("T: {}", count_t);
    }
}
