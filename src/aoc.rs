use anyhow::Result;

#[derive(clap::Parser)]
pub struct RunOptions {
    /// Mode to run in
    pub mode: Mode,

    /// Solution day to run
    pub day: u8,

    /// Which part to run; defaults to both
    #[arg(value_enum, default_value_t = Part::Both)]
    pub part: Part,
}

#[derive(Clone, Copy, Eq, PartialEq, clap::ValueEnum)]
pub enum Part {
    #[value(name = "1")]
    One,
    #[value(name = "2")]
    Two,
    #[value(hide = true)]
    Both,
}

#[derive(Clone, Copy, Eq, PartialEq, clap::ValueEnum)]
pub enum Mode {
    Test,
    Print,
    Submit,
}

pub struct Day<const Y: u16, const N: u8>;

impl<const Y: u16, const N: u8> Day<Y, N> {
    pub fn run<Parsed, Output: ToString>(&self, options: &RunOptions) -> Result<()>
    where
        Self: Parser<Parsed> + Solution<Parsed, Output>,
    {
        let client = libaoc::Client::new()?;
        let input = client.get_input(&(Y, N))?;
        let parsed = self.parse(input)?;
        let part1 = self.part1(&parsed);
        println!("Part 1: {}", part1.to_string());
        let part2 = self.part2(&parsed);
        println!("Part 2: {}", part2.to_string());
        Ok(())
    }
}

pub trait Parser<T> {
    fn parse(&self, input: String) -> Result<T>;
}

pub trait Solution<Input = String, Output: ToString = String> {
    #[allow(unused_variables)]
    fn part1(&self, input: &Input) -> Output {
        panic!("Not implemented");
    }

    #[allow(unused_variables)]
    fn part2(&self, input: &Input) -> Output {
        panic!("Not implemented");
    }
}

impl<T: Solution<String>> Parser<String> for T {
    fn parse(&self, input: String) -> Result<String> {
        Ok(input)
    }
}
