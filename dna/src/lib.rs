//! A general-purpose genomics crate for dealing with DNA.

#![warn(missing_docs)]

use std::{convert::TryFrom, fmt::Display, str::FromStr};

/// A nucleotide
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nuc {
    /// Adenine
    A,
    /// Cytosine
    C,
    /// Guanine
    G,
    /// Thymine
    T,
}

/// An error that can occur when parsing a nucleotide.
#[derive(Debug, thiserror::Error)]
#[error("failed to parse nucleotide from {0}")]
pub struct ParseNucError<T: Display>(T);

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

impl FromStr for Nuc {
    type Err = ParseNucError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let upper = s.to_ascii_uppercase();
        match upper.as_str() {
            "A" => Ok(Self::A),
            "C" => Ok(Self::C),
            "G" => Ok(Self::G),
            "T" => Ok(Self::T),
            _ => Err(ParseNucError(upper)),
        }
    }
}

impl TryFrom<u8> for Nuc {
    type Error = ParseNucError<u8>;

    /// Tries to convert a `u8` value to a `Nuc` enum variant.
    ///
    /// # Arguments
    ///
    /// * `value` - The `u8` value to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Nuc` variant if the conversion is successful,
    /// or an `InvalidNucError` if the value is not a valid nucleotide representation.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::A),
            1 => Ok(Self::C),
            2 => Ok(Self::G),
            3 => Ok(Self::T),
            _ => Err(ParseNucError(value)),
        }
    }
}


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

/// A module containing a more memory-efficient representation for DNA.

mod packed {
    use std::convert::TryFrom;
    use std::str::FromStr;
    use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

    #[derive(Debug, PartialEq)]

    /// A more memory-efficient representation for DNA.
    pub struct PackedDna(pub Vec<u8>);


    impl FromStr for PackedDna {
        type Err = crate::ParseNucError<char>;
        /// Converts a string slice to a `PackedDna` instance.
        ///
        /// # Arguments
        ///
        /// * `s` - The string slice to parse.
        ///
        /// # Returns
        ///
        /// A `Result` containing the `PackedDna` instance if the parsing is successful,
        /// or a `ParseNucError` if an error occurs.
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let nucleotides: Result<Vec<crate::Nuc>, _> = s.chars().map(crate::Nuc::try_from).collect();
            nucleotides.map(|nucs: Vec<crate::Nuc>| {
                let packed_nucs: Vec<u8> = nucs.into_iter().map(|n: crate::Nuc| n as u8).collect();
                PackedDna(packed_nucs)
            })
        }
    }

    
    
    
    impl std::iter::FromIterator<crate::Nuc> for PackedDna {
        /// Constructs a `PackedDna` instance from an iterator over `Nuc` values.
        ///
        /// # Arguments
        ///
        /// * `iter` - An iterator over `Nuc` values.
        ///
        /// # Returns
        ///
        /// The constructed `PackedDna` instance.
        fn from_iter<I: IntoIterator<Item = crate::Nuc>>(iter: I) -> Self {
            let nucleotides: Vec<u8> = iter.into_iter().map(|n| n as u8).collect();
            PackedDna(nucleotides)
        }
    }

    impl PackedDna {
        /// Retrieves the nucleotide at the specified index.
        ///
        /// # Arguments
        ///
        /// * `idx` - The index of the nucleotide to retrieve.
        ///
        /// # Panics
        ///
        /// Panics if the index is out of bounds.
        ///
        /// # Returns
        ///
        /// The nucleotide at the specified index.
        pub fn get(&self, idx: usize) -> crate::Nuc {
            let value = self.0[idx];
            crate::Nuc::try_from(value).unwrap()
        }
    }
}







#[cfg(test)]
mod tests {
    // TODO: fill in tests
    use super::*;

    /// Tests the `from_str` function of `PackedDna`.
    #[test]
    fn packed_dna_from_str() {
        // Valid input
        let expected0: packed::PackedDna = packed::PackedDna(vec![0]);
        assert_eq!(packed::PackedDna::from_str("A").unwrap(), expected0);

        let expected1: packed::PackedDna = packed::PackedDna(vec![0, 1, 2, 3]);
        assert_eq!(packed::PackedDna::from_str("ACGT").unwrap(), expected1);
        assert_eq!(packed::PackedDna::from_str("acgt").unwrap(), expected1);
        assert_eq!(packed::PackedDna::from_str("ACgt").unwrap(), expected1);

        let expected2: packed::PackedDna = packed::PackedDna(vec![0, 1, 3, 3, 1, 0]);
        assert_eq!(packed::PackedDna::from_str("ACTTCA").unwrap(), expected2);
        let expected3: packed::PackedDna = packed::PackedDna(vec![2, 2, 2, 3, 1, 0]);
        assert_eq!(packed::PackedDna::from_str("gggtca").unwrap(), expected3);
        

        // Invalid input
        assert!(packed::PackedDna::from_str("XYZ").is_err());
    }

    /// Tests the `from_iter` function of `PackedDna`.
    #[test]
    fn packed_dna_from_iterator() {
        let nucs = vec![Nuc::A, Nuc::C, Nuc::G, Nuc::T];
        let packed_dna: packed::PackedDna = nucs.into_iter().collect();
        assert_eq!(packed_dna.0, vec![0, 1, 2, 3]);
    }

    /// Tests the `get` function of `PackedDna`.
    #[test]
    fn packed_dna_get() {
        let packed_dna = packed::PackedDna(vec![0, 1, 2, 3]);
        assert_eq!(packed_dna.get(0), Nuc::A);
        assert_eq!(packed_dna.get(1), Nuc::C);
        assert_eq!(packed_dna.get(2), Nuc::G);
        assert_eq!(packed_dna.get(3), Nuc::T);
    }
}

