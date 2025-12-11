use crate::prelude::parse::*;
use nom::error::Error;
use nom::{AsChar, Input};
use std::fmt::Debug;

pub trait Parse
where
  Self: Input + Debug,
{
  /// Parses the entire input using the provided parser, panicking if the input fails
  /// to parse or if any input remains unparsed.
  fn parse_full<P: Parser<Self, Error = Error<Self>>>(self, parser: P) -> P::Output;

  /// Parses the input using the provided parser, returning any remaining unparsed input
  /// along with the parser output, or panicking on parse errors.
  fn parse_partial<P: Parser<Self, Error = Error<Self>>>(self, parser: P) -> (Self, P::Output);
}

impl Parse for &str {
  fn parse_full<P>(self, parser: P) -> P::Output
  where
    P: Parser<Self, Error = Error<Self>>,
  {
    match self.parse_partial(parser) {
      ("", output) => output,
      (remaining, _) => panic!("Failed to parse entire input; trailing: {:?}", remaining),
    }
  }

  fn parse_partial<P: Parser<Self, Error = Error<Self>>>(self, mut parser: P) -> (Self, P::Output) {
    parser.parse(self.trim()).expect("Parse error")
  }
}

pub trait ParseChars
where
  Self: Input + Debug,
{
  /// Parses the input as a grid of characters, separated by newlines.
  fn parse_char_grid(self) -> Vec<Vec<char>>;

  /// Parses the input, applying the provided parser line by line.
  fn parse_lines<P: Parser<Self, Error = Error<Self>>>(self, parser: P) -> Vec<P::Output>;
}

impl<T> ParseChars for T
where
  T: Parse,
  T::Item: AsChar,
{
  fn parse_char_grid(self) -> Vec<Vec<char>> {
    self.parse_lines(many0(none_of("\n")))
  }

  fn parse_lines<P: Parser<Self, Error = Error<Self>>>(self, parser: P) -> Vec<P::Output> {
    self.parse_full(separated_list0(newline, parser))
  }
}
