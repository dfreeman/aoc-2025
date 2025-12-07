use std::time::Instant;

use clap::Parser;

#[macro_export]
macro_rules! solution {
    ($($config:tt)*) => {
        pub fn main() {
            $crate::runner::Solution { $($config)* }.run();
        }
    };
}

pub struct Solution<I, S1: ToString, S2: ToString> {
  pub day: u8,
  pub year: u16,
  pub parse: fn(&str) -> I,
  pub part_1: fn(&I) -> S1,
  pub part_2: fn(&I) -> S2,
}

impl<I, S1: ToString, S2: ToString> Solution<I, S1, S2> {
  pub fn run(&self) {
    let options = CLIOptions::parse();
    let client = libaoc::Client::new().expect("Failed to create AoC client");
    let puzzle_id = (self.year, self.day);
    let input = client
      .get_input(&puzzle_id)
      .expect("Failed to fetch puzzle input");
    let (parsed, parse_time) = time(|| (self.parse)(&input));

    if options.part.includes_part_1() {
      let (result, solve_time) = time(|| (self.part_1)(&parsed));
      let result = result.to_string();

      println!("Part 1: {result} (parse {parse_time:?}, solve {solve_time:?})");

      if options.submit {
        client
          .submit(&puzzle_id, Some(1), result)
          .expect("Failed to submit answer for Part 1");
      }
    }

    if options.part.includes_part_2() {
      let (result, solve_time) = time(|| (self.part_2)(&parsed));
      let result = result.to_string();

      println!("Part 2: {result} (parse {parse_time:?}, solve {solve_time:?})");

      if options.submit {
        client
          .submit(&puzzle_id, Some(2), result)
          .expect("Failed to submit answer for Part 2");
      }
    }
  }
}

fn time<S>(f: impl FnOnce() -> S) -> (S, std::time::Duration) {
  let start = Instant::now();
  let result = f();
  let end = Instant::now();
  (result, end - start)
}

#[derive(clap::Parser)]
pub struct CLIOptions {
  /// Which part to run
  #[arg(short, long, default_value_t = PartSelection::Both)]
  pub part: PartSelection,

  /// Whether the answer should be submitted automatically
  #[arg(short, long, default_value_t = false)]
  pub submit: bool,
}

#[derive(Clone, Copy, Eq, PartialEq, clap::ValueEnum)]
pub enum PartSelection {
  #[value(name = "1")]
  One,
  #[value(name = "2")]
  Two,
  #[value(name = "both", hide = true)]
  Both,
}

impl ToString for PartSelection {
  fn to_string(&self) -> String {
    match self {
      PartSelection::One => "1".to_string(),
      PartSelection::Two => "2".to_string(),
      PartSelection::Both => "both".to_string(),
    }
  }
}

impl PartSelection {
  pub fn includes_part_1(&self) -> bool {
    matches!(self, PartSelection::One | PartSelection::Both)
  }

  pub fn includes_part_2(&self) -> bool {
    matches!(self, PartSelection::Two | PartSelection::Both)
  }
}
