use miette::GraphicalReportHandler;
use nom::{
    character::complete::multispace0, error::ParseError, sequence::tuple, AsChar, IResult,
    InputIter, InputLength, InputTake, InputTakeAtPosition, Parser,
};
use nom_locate::LocatedSpan;
use nom_supreme::error::{BaseErrorKind, GenericErrorTree};

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

pub type Span<'a> = LocatedSpan<&'a str>;

pub use nom_supreme::{error::ErrorTree, final_parser::final_parser};

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("bad input")]
struct BadInput {
    #[source_code]
    src: &'static str,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
}

pub fn report_miette_error<OK>(
    from: Result<OK, ErrorTree<Span>>,
    input_static: &'static str,
) -> OK {
    match from {
        Ok(from) => from,
        Err(e) => {
            match e {
                GenericErrorTree::Base { location, kind } => {
                    let offset = location.location_offset().into();
                    let err = BadInput {
                        src: input_static,
                        bad_bit: miette::SourceSpan::new(offset, 0.into()),
                        kind,
                    };
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .render_report(&mut s, &err)
                        .unwrap();
                    println!("{s}");
                }
                GenericErrorTree::Stack { .. } => todo!("stack"),
                GenericErrorTree::Alt(_) => todo!("alt"),
            }
            panic!("error dealing with parsing")
        }
    }
}
