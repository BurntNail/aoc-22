use nom::{
    character::complete::multispace0, error::ParseError, sequence::tuple, AsChar, IResult,
    InputIter, InputLength, InputTake, InputTakeAtPosition, Parser,
};

pub fn remove_spaces<I, O1, E: ParseError<I>, F>(parser: F) -> impl FnMut(I) -> IResult<I, O1, E>
where
    I: Clone + InputLength + InputIter + InputTake + InputTakeAtPosition,
    <I as InputIter>::Item: AsChar + Clone,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    F: Parser<I, O1, E> + Copy,
{
    move |input: I| {
        let (input, (_, res, _)) = tuple((multispace0, parser, multispace0))(input)?;
        Ok((input, res))
    }
}
