mod aoc;
mod day01;
mod day02;

use clap::Parser;
use seq_macro::seq;

fn main() -> anyhow::Result<()> {
    let options = aoc::RunOptions::parse();
    seq!(N in 1..=2 {
        match options.day {
            #(N => aoc::Day::<2024, N>.run(&options),)*
            _ => panic!("Day {} is not implemented yet", options.day),
        }
    })
}
