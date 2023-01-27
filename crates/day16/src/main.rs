use std::collections::HashMap;

use crate::{parse::{parse_all, Valve}, network::{all_combos, get_best}};
use utilities::nom_utils::{final_parser, report_miette_error, ErrorTree, Span};

mod parse;
mod network;

fn main() {
    let input = include_str!("input.txt");
    let all: Result<_, ErrorTree<Span>> = final_parser(parse_all::<ErrorTree<Span>>)(input.into());
    let all = report_miette_error(all, input).into_iter().collect::<HashMap<String, Valve>>();
    

    let all_combos = all_combos("AA".into(), all.clone(), 30);
    println!("{all_combos:?}");
    let best = get_best(all_combos, all.into_iter().map(|(s, v)| (s, v.flow_rate())).collect());
    println!("{best:#?}");
}
