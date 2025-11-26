use nom::Parser;

pub trait ParseExts
where
    Self: nom::Input + std::fmt::Debug,
{
    fn parse_char_grid(self) -> Vec<Vec<char>>
    where
        Self::Item: nom::AsChar,
    {
        self.parse_full(nom::multi::separated_list0(
            nom::character::complete::newline,
            nom::multi::many0(nom::character::complete::none_of("\n")),
        ))
    }

    fn parse_full<P>(self, parser: P) -> P::Output
    where
        P: Parser<Self, Error = nom::error::Error<Self>>;

    fn parse_lines<P>(self, parser: P) -> Vec<P::Output>
    where
        P: Parser<Self, Error = nom::error::Error<Self>>,
        Self::Item: nom::AsChar,
    {
        self.parse_full(nom::multi::separated_list0(
            nom::character::complete::newline,
            parser,
        ))
    }
}

impl ParseExts for &str {
    fn parse_full<P>(self, mut parser: P) -> P::Output
    where
        P: Parser<Self, Error = nom::error::Error<Self>>,
    {
        match parser.parse(self.trim()).expect("Parse error") {
            ("", output) => output,
            (remaining, _) => panic!("Failed to parse entire input; trailing: {:?}", remaining),
        }
    }
}
