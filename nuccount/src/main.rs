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

/// Count the number of occurrences of each nucleotide in the provided DNA.
#[derive(Debug, StructOpt)]
struct Opts {
    /// The DNA sequence for which we should retrieve a nucleotide count.
    ///
    /// It is case insensitive but only nucleotides A, C, G and T are supported.
    #[structopt(short = "d", long, required = true)]
    dna: String
}

fn main() {
    let opts = Opts::from_args();
    let dna = opts.dna;
    println!("Input: {}", &dna);

    let packed_dna = match <dna::packed::PackedDna as dna::packed::FromStr>::from_str(&dna) {
        Ok(p) => p,
        Err(_) => panic!("{} is not a valid string. Input must consist of 
            nucleotide characters only.", dna),
    };

    let mut a_count = 0;
    let mut c_count = 0;
    let mut g_count = 0;
    let mut t_count = 0;

    let mut i = 0;
    let len = packed_dna.size;
    while i < len {
        let n = packed_dna.get(i);
        match n {
            dna::Nuc::A => {a_count += 1; ()},
            dna::Nuc::C => {c_count += 1; ()},
            dna::Nuc::G => {g_count += 1; ()},
            dna::Nuc::T => {t_count += 1; ()},
        }
        i += 1;
    }

    println!("Length: {}", len);
    println!("A: {}", a_count);
    println!("C: {}", c_count);
    println!("G: {}", g_count);
    println!("T: {}", t_count);
    // use FromStr on packed dna
    // iterate thru packed dna, count
    // print out counts
}
