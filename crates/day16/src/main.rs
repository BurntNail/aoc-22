use crate::valve::{parse_all, Valve};
use utilities::nom_utils::{final_parser, report_miette_error, ErrorTree, Span};

mod valve;

fn main() {
    let input = include_str!("input.txt");
    let all: Result<_, ErrorTree<Span>> = final_parser(parse_all::<ErrorTree<Span>>)(input.into());
    let all: Vec<(String, Valve)> = report_miette_error(all, input);
    println!("{all:#?}");
}
