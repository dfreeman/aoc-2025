# Advent of Code 2025 🎄

I'm using AoC as a chance to get my hands dirty with a bit of Rust this year. I'm sure there's plenty in here that's unidiomatic or unperformant, but it's been a fun experience so far.

Each day is its own binary under `src/bin` that uses [`libaoc`](https://docs.rs/libaoc/0.1.55/libaoc/) to fetch puzzle inputs and submit answers. It expects an `AOC_AUTH_TOKEN` environment variable.

Each binary will, by default, run and print the results for both parts of its day's problem. It optionally accepts a `--part` flag to only run one part of the problem, and a `--submit` flag to also submit the answer for the part(s) it runs.

```sh
cargo run --release --bin=day01 -- --part=1
```
