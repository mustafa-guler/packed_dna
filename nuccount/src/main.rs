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

use structopt::StructOpt;
use dna::packed::PackedDna;

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

    // Convert the DNA sequence to PackedDna
    let packed_dna = match <PackedDna as std::str::FromStr>::from_str(&dna) {
        Ok(packed_dna) => packed_dna,
        Err(error) => {
            //Error: failed to parse nucleotide from X - failed char
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    println!("Input: {}", &dna);

    // Count the nucleotides
    let mut nucleotide_counts = [0; 4];
    for nucleotide in packed_dna.0 {
        nucleotide_counts[nucleotide as usize] += 1;
    }

    // Print the nucleotide counts
    let nucleotides = ['A', 'C', 'G', 'T'];
    for (nucleotide, count) in nucleotides.iter().zip(nucleotide_counts.iter()) {
        println!("{}: {}", nucleotide, count);
    }
}



