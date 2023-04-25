//! A general-purpose genomics crate for dealing with DNA.

#![warn(missing_docs)]

use std::{convert::TryFrom, fmt::Display, iter::FromIterator, str::FromStr};

// TODO: add a packed module with the PackedDna struct
//
// this struct must have the following:
// 1. A representation that is more memory efficient that simply storing a vector of `Nuc`
// 2. A FromStr implementation (should be case insensitive like the `Nuc` impl)
// 3. A `FromIterator` implementation to construct it from an iterator over `Nuc`s
// 4. A `fn get(&self, idx: usize) -> Nuc` getter for a particular nucleotide
//
// Make sure to unit test and document all elements
// Also, the internal representation of the PackedDna struct should be privately scoped
/// An error that can occur when parsing a nucleotide.

#[derive(Debug, thiserror::Error)]
#[error("failed to parse nucleotide from {0}")]
pub struct ParseNucError<T: Display>(T);

/// A nucleotide
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nuc {
    /// Adenine
    A = 0b00,
    /// Cytosine
    C = 0b01,
    /// Guanine
    G = 0b10,
    /// Thymine
    T = 0b11,
}

impl TryFrom<char> for Nuc {
    type Error = ParseNucError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'T' => Ok(Self::T),
            _ => Err(ParseNucError(value)),
        }
    }
}

///PackedDna struct
#[derive(Debug)]
pub struct PackedDna(Vec<Nuc>);

///implementation for packed dan struct
impl PackedDna {
    ///store the dna string as a vector of nucs
    pub fn new(dna: &[Nuc]) -> Self {
        let mut packed = Vec::new();
        for nuc in dna.iter() {
            packed.push(*nuc);
        }
        PackedDna(packed)
    }
    ///get the packeddna private object at given index
    pub fn get(&self, idx: usize) -> Nuc {
        let characters = &self.0;

        characters[idx]
    }
}
///A FromStr implementation (should be case insensitive like the `Nuc` impl)
impl FromStr for PackedDna {
    type Err = ParseNucError<char>;

    ///take dna string, get characters and the nuc 2-bit version of nucleotide and store it in vector of nucs
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dna = Vec::new();
        for c in s.chars() {
            let nuc = Nuc::try_from(c)?;
            dna.push(nuc);
        }

        Ok(PackedDna::new(&dna))
    }
}

///A `FromIterator` implementation to construct it from an iterator over `Nuc`s
impl FromIterator<Nuc> for PackedDna {
    fn from_iter<I: IntoIterator<Item = Nuc>>(iter: I) -> Self {
        let dna: Vec<Nuc> = iter.into_iter().collect();
        PackedDna::new(&dna)
    }
}

#[cfg(test)]
mod tests {
    // TODO: fill in tests
    use super::*;

    //failing case
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: ParseNucError('U')")]
    fn tryfrom_char() {
        let s = 'U';
        Nuc::try_from(s).unwrap();
    }

    //passing case
    #[test]
    fn tryfrom_char2() {
        let s = 'C';
        assert_eq!(Nuc::try_from(s).unwrap(), Nuc::C);
        let s = 'A';
        assert_eq!(Nuc::try_from(s).unwrap(), Nuc::A);
        let s = 'T';
        assert_eq!(Nuc::try_from(s).unwrap(), Nuc::T);
        let s = 'G';
        assert_eq!(Nuc::try_from(s).unwrap(), Nuc::G);
    }

    //failing case
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: ParseNucError('D')")]
    fn fromstr() {
        let s = "ACGTTDTT";
        PackedDna::from_str(s).unwrap();
    }

    //passing case
    #[test]
    fn fromstr2() {
        let s = "ACGTT";
        let packed = PackedDna::from_str(s).unwrap();
        assert_eq!(packed.get(0), Nuc::A);
        assert_eq!(packed.get(1), Nuc::C);
        assert_eq!(packed.get(2), Nuc::G);
        assert_eq!(packed.get(3), Nuc::T);
    }

    #[test]
    fn fromiter() {
        let dna = vec![Nuc::A, Nuc::C, Nuc::G, Nuc::T];
        let packed: PackedDna = dna.into_iter().collect();
        assert_eq!(packed.get(0), Nuc::A);
        assert_eq!(packed.get(1), Nuc::C);
        assert_eq!(packed.get(2), Nuc::G);
        assert_eq!(packed.get(3), Nuc::T);
    }
}
