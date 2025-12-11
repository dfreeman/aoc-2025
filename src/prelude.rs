use core::panic;

pub use crate::exts::*;
pub use crate::solution;

pub mod parse {
  pub use nom::Parser;
  pub use nom::bytes::complete::tag;
  pub use nom::character::complete::{alpha1, char, digit1, newline, space0, space1};
  pub use nom::character::complete::{i8, i16, i32, i64, u8, u16, u32, u64};
  pub use nom::character::{none_of, one_of};
  pub use nom::multi::{many0, many1, separated_list0, separated_list1};
  pub use nom::sequence::{delimited, pair, separated_pair};
}

pub fn part_1(_: String) -> usize {
  panic!("Part 1 not implemented");
}

pub fn part_2(_: String) -> usize {
  panic!("Part 2 not implemented");
}

pub fn parse(input: &str) -> String {
  input.to_string()
}
