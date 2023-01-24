//! A general-purpose genomics crate for dealing with DNA.

#![warn(missing_docs)]

use std::{convert::TryFrom, fmt::Display, str::FromStr};

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

mod packed {
    // Internally, we have a vector of i8s. Each i8 represents up to 4
    // nucleotides.
    // 00 -> A
    // 01 -> C
    // 10 -> G
    // 11 -> T
    pub struct PackedDna {
        data: Vec<i8>,
        size: usize,
    }

    impl PackedDna {
        // Returns a an option with Some(p) where p is the PackedDna struct 
        // representig the DNA if parsing was successful and None otherwise.
        pub fn from_str(s: &str) -> Option<Self> {
            let upper = s.to_ascii_uppercase();
            // let mut s_idx = 0; // Index in string
            let mut i = 0; // Index in int that is being modified, from 0 to 3
            let mut size = 0;
            let mut x: i8 = 0; // Next int that will be added to the vector
            let mut res: Vec<i8> = Vec::new();
            // let len = s.len();

            for c in upper.chars() {
                // let c = upper[s_idx];
                println!("i: {}", i);
                let y: i8 = match c {
                    'A' => 0,
                    'C' => 1,
                    'G' => 2,
                    'T' => 3,
                    _ => -1,
                };
                
                if y == -1 {
                    return None;
                }

                x = x | (y << ((3-i) * 2)); // Add nucleotide to int
                if i == 3 {
                    res.push(x);
                    // println!("pushed to vec");
                    x = 0;
                }
                
                i = (i + 1) % 4;
                size += 1;
                // s_idx += s_idx;
            }

            if i != 0 {
                res.push(x);
                // println!("pushed to vec");
            }

            let p = PackedDna {
                data: res, 
                size: size,
            };
            // println!("Succesfully parsed DNA of size {}", p.size);
            // println!("Vector has size {}", p.data.len());
            // println!("SUCCESS");
            return Some(p)
        }
    }

    // impl FromIterator for PackedDna {

    // }

    impl PackedDna {
        pub fn get(&self, idx: usize) -> Option<crate::Nuc> {
            let vec_idx = idx / 4;
            let int_idx = idx % 4;
            let x = self.data[vec_idx];
            let mask: i8 = 3 << ((3 - int_idx) * 2);
            let y: u8 = ((x & mask) as u8) >> ((3 - int_idx) * 2);
            let res = match y {
                0 => Some(crate::Nuc::A),
                1 => Some(crate::Nuc::C),
                2 => Some(crate::Nuc::G),
                3 => Some(crate::Nuc::T),
                _ => None, // What should I do here?
            };
            
            res
        }
    }
}

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

#[cfg(test)]
mod tests {
    // TODO: fill in tests

    // Tests a valid char
    // Precondition: c must be in {'a', 'c', 'g', 't', 'A', 'C', 'G', 'T'}
    fn test_char_ok(c: char) {
        let res: Result<crate::Nuc, _> = crate::TryFrom::try_from(c);
        let correct = match c {
            'a' => Some(crate::Nuc::A),
            'c' => Some(crate::Nuc::C),
            'g' => Some(crate::Nuc::G),
            't' => Some(crate::Nuc::T),
            'A' => Some(crate::Nuc::A),
            'C' => Some(crate::Nuc::C),
            'G' => Some(crate::Nuc::G),
            'T' => Some(crate::Nuc::T),
            _ => None
        };
        let b = match res {
            Ok(a) => Some(a) == correct,
            Err(_) => false,
        };
        assert!(b);
    }

    // Tests an invalid char
    // Precondition: c must NOT be in {'a', 'c', 'g', 't', 'A', 'C', 'G', 'T'}
    fn test_char_err(c: char) {
        let res: Result<crate::Nuc, _> = crate::TryFrom::try_from(c);
        let b = match res {
            Ok(_) => false,
            Err(_) => true,
        };
        assert!(b);
    }

    // Tests a valid string
    // Precondition: c must be in {"a", "c", "g", "t", "A", "C", "G", "T"}
    fn test_str_ok(s: &str) {
        let res: Result<crate::Nuc, _> = crate::FromStr::from_str(s);
        let correct = match s {
            "a" => Some(crate::Nuc::A),
            "c" => Some(crate::Nuc::C),
            "g" => Some(crate::Nuc::G),
            "t" => Some(crate::Nuc::T),
            "A" => Some(crate::Nuc::A),
            "C" => Some(crate::Nuc::C),
            "G" => Some(crate::Nuc::G),
            "T" => Some(crate::Nuc::T),
            _ => None
        };
        let b = match res {
            Ok(a) => Some(a) == correct,
            Err(_) => false,
        };
        assert!(b);
    }

    // Tests an invalid string
    // Precondition: c must NOT be in {"a", "c", "g", "t", "A", "C", "G", "T"}
    fn test_str_err(s: &str) {
        let res: Result<crate::Nuc, _> = crate::FromStr::from_str(s);
        let b = match res {
            Ok(_) => false,
            Err(_) => true,
        };
        assert!(b);
    }

    #[test]
    fn tryfrom_char() {
        // Test uppercase chars
        test_char_ok('A');
        test_char_ok('C');
        test_char_ok('G');
        test_char_ok('T');

        // Test lowercase chars
        test_char_ok('a');
        test_char_ok('c');
        test_char_ok('g');
        test_char_ok('t');

        // Test invalid chars
        test_char_err('L');
        test_char_err('x');
    }

    #[test]
    fn fromstr() {
        // Test uppercase strings
        test_str_ok("A");
        test_str_ok("C");
        test_str_ok("G");
        test_str_ok("T");

        // Test lowercase strings
        test_str_ok("a");
        test_str_ok("c");
        test_str_ok("g");
        test_str_ok("t");

        // Test invalid strings 
        test_str_err("L");
        test_str_err("xX");
    }
}
