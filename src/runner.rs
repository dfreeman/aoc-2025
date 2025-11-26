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

pub struct Solution<S1: ToString, S2: ToString> {
    pub day: u8,
    pub year: u16,
    pub part_1: fn(&str) -> S1,
    pub part_2: fn(&str) -> S2,
}

impl<S1: ToString, S2: ToString> Solution<S1, S2> {
    pub fn run(&self) {
        let options = CLIOptions::parse();
        let client = libaoc::Client::new().expect("Failed to create AoC client");
        let puzzle_id = (self.year, self.day);
        let input = client
            .get_input(&puzzle_id)
            .expect("Failed to fetch puzzle input");

        if options.part.includes_part_1() {
            let (result, duration) = time(|| (self.part_1)(&input));
            println!("Part 1: {} (took {:?})", result, duration);
            if options.submit {
                client
                    .submit(&puzzle_id, Some(1), result)
                    .expect("Failed to submit answer for Part 1");
            }
        }

        if options.part.includes_part_2() {
            let (result, duration) = time(|| (self.part_2)(&input));
            println!("Part 2: {} (took {:?})", result, duration);
            if options.submit {
                client
                    .submit(&puzzle_id, Some(2), result)
                    .expect("Failed to submit answer for Part 2");
            }
        }
    }
}

fn time<S: ToString>(f: impl FnOnce() -> S) -> (String, std::time::Duration) {
    let start = Instant::now();
    let result = f();
    let end = Instant::now();
    (result.to_string(), end - start)
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
