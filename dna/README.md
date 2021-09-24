# This Crate
This is library crate intended to include re-usable elements for operating on and dealing with DNA.

Provided is an implementation of a [nucleotide](https://en.wikipedia.org/wiki/Nucleotide) and some parsers to convert characters and strings into a nucleotide.

# Background
DNA for our purposes can be thought of as a simple string drawn from the alphabet of nucleotides. Nucleotides are a four-element alphabet (A,C,G,T).
Because we are limited to this four-character alphabet we can represent a DNA string more compactly by packing bits together.

# Tasks
There are a few tasks that should be completed in order, all of which are marked with `TODO` statements in the code.

1. Fill in the existing unit tests in `src/lib.rs`
2. Add a new `packed` module containing an implementation of `PackedDna`, see comments for details
