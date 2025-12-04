pub use nom::Parser;

use nom::{AsChar, Input, character::complete::*, error::Error, multi::*};
use std::fmt::Debug;

pub trait ParseFull
where
    Self: Input + Debug,
{
    /// Parses the entire input using the provided parser, panicking if the input fails
    /// to parse or if any input remains unparsed.
    fn parse_full<P: Parser<Self, Error = Error<Self>>>(self, parser: P) -> P::Output;
}

impl ParseFull for &str {
    fn parse_full<P>(self, mut parser: P) -> P::Output
    where
        P: Parser<Self, Error = Error<Self>>,
    {
        match parser.parse(self.trim()).expect("Parse error") {
            ("", output) => output,
            (remaining, _) => panic!("Failed to parse entire input; trailing: {:?}", remaining),
        }
    }
}

pub trait CharParseUtils
where
    Self: Input + Debug,
{
    /// Parses the input as a grid of characters, separated by newlines.
    fn parse_char_grid(self) -> Vec<Vec<char>>;

    /// Parses the input, applying the provided parser line by line.
    fn parse_lines<P: Parser<Self, Error = Error<Self>>>(self, parser: P) -> Vec<P::Output>;
}

impl<T> CharParseUtils for T
where
    T: ParseFull,
    T::Item: AsChar,
{
    fn parse_char_grid(self) -> Vec<Vec<char>> {
        self.parse_lines(many0(none_of("\n")))
    }

    fn parse_lines<P: Parser<Self, Error = Error<Self>>>(self, parser: P) -> Vec<P::Output> {
        self.parse_full(separated_list0(newline, parser))
    }
}
