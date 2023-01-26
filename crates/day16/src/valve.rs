use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, newline, one_of, u64},
    combinator::{map, opt},
    error::ParseError,
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};
use once_cell::sync::Lazy;
use utilities::nom_utils::Span;

pub type Int = u64;
pub fn int<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Int, E> {
    u64(input)
}

#[derive(Clone, Debug)]
pub struct Valve {
    flow_rate: Int,
    leads_to: Vec<String>,
}

impl Valve {
    fn parse_valve_name<'a, E: ParseError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, String, E> {
        static ALL_LETTERS: Lazy<String> = Lazy::new(|| ('A'..='Z').collect::<String>());

        map(
            tuple((one_of(ALL_LETTERS.as_str()), one_of(ALL_LETTERS.as_str()))),
            |(c1, c2)| c1.to_string() + &c2.to_string(),
        )(input)
    }

    pub fn parse<'a, E: ParseError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, (String, Self), E> {
        let (input, self_name) = preceded(tag("Valve "), Self::parse_valve_name)(input)?;
        let (input, flow_rate) = preceded(tag(" has flow rate="), int)(input)?;
        let (input, _) = tuple((
            tag("; tunnel"),
            opt(char('s')),
            multispace0,
            tag("lead"),
            opt(char('s')),
            multispace0,
            tag("to valve"),
            opt(char('s')),
            multispace0,
        ))(input)?;
        let (input, leads_to) = separated_list0(tag(", "), Self::parse_valve_name)(input)?;

        Ok((
            input,
            (
                self_name,
                Self {
                    flow_rate,
                    leads_to,
                },
            ),
        ))
    }
}

pub fn parse_all<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<(String, Valve)>, E> {
    // let nl = alt((tag("\n"), tag("\r\n"), tag(" ")));
    separated_list0(newline, Valve::parse)(i)
}
