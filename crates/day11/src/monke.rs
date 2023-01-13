use nom::{
    bytes::complete::tag,
    character::{complete as cc, complete::one_of},
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::VecDeque;

type IntItem = u128;
fn parse_intitem(input: &str) -> IResult<&str, IntItem> {
    cc::u128(input)
}

#[derive(Clone, Copy)]
pub enum Operation {
    Mul,
    Add,
}
pub struct NewOperation(Operation, u128);
pub fn parse_operation(input: &str) -> IResult<&str, NewOperation> {
    let (input, _) = tag("new = old ")(input)?;
    let (input, (op, term)) = tuple((one_of("*+"), preceded(cc::space1, parse_intitem)))(input)?;

    let op = match op {
        '*' => Operation::Mul,
        '+' => Operation::Add,
        _ => panic!("Unseen {op:?} op"),
    };

    Ok((input, NewOperation(op, term)))
}

pub fn start_items(input: &str) -> IResult<&str, Vec<u128>> {
    let (input, _) = tag("Starting items: ")(input)?;
    separated_list0(tag(", "), parse_intitem)(input)
}
pub fn test(input: &str) -> IResult<&str, IntItem> {
    preceded(tag("Test: divisible by "), parse_intitem)(input)
}

pub struct Monkey {
    items: VecDeque<u128>,
    operation: NewOperation,
    test: u128,
}

pub fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let [start_bit, start_items, op, test, if_true, if_false]: _ =
        input.lines().collect::<Vec<&str>>().try_into().unwrap();

    let _ = tuple((tag("Monkey "), parse_intitem, tag))(start_bit)?;

    todo!()
}
