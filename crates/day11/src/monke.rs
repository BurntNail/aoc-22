use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete as cc,
        complete::{multispace0, one_of},
    },
    combinator::{map, value},
    multi::{count, separated_list0},
    sequence::{preceded, tuple},
    IResult,
};
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::Zero;
use std::collections::VecDeque;
use utilities::{int_utils::DivExt, nom_utils::remove_spaces};

pub type IntItem = BigUint;
fn int_item(input: &str) -> IResult<&str, IntItem> {
    map(cc::u128, IntItem::from)(input)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operation {
    Mul,
    Add,
}
impl Operation {
    pub fn operation(input: &str) -> IResult<&str, Operation> {
        let (input, op) = one_of("*+")(input)?;
        Ok((
            input,
            match op {
                '*' => Operation::Mul,
                '+' => Operation::Add,
                _ => unreachable!("nom shouldn't let us get here"),
            },
        ))
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    Literal(IntItem),
    Old,
}
impl Term {
    pub fn term(input: &str) -> IResult<&str, Term> {
        alt((map(int_item, Term::Literal), value(Term::Old, tag("old"))))(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewOperation(pub Operation, pub Term);
impl NewOperation {
    pub fn new_operation(input: &str) -> IResult<&str, Self> {
        let (input, (op, _, term)) = preceded(
            tag("Operation: new = old "),
            tuple((Operation::operation, cc::space1, Term::term)),
        )(input)?;
        Ok((input, Self(op, term)))
    }

    pub fn run(&self, old: IntItem) -> IntItem {
        let term = match self.1.clone() {
            Term::Literal(l) => l,
            Term::Old => old.clone(),
        };
        match self.0 {
            Operation::Mul => old * term,
            Operation::Add => old + term,
        }
    }
}

pub fn start_items(input: &str) -> IResult<&str, VecDeque<IntItem>> {
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, items) = separated_list0(tag(", "), int_item)(input)?;

    Ok((input, items.into()))
}
pub fn test(input: &str) -> IResult<&str, IntItem> {
    preceded(tag("Test: divisible by "), int_item)(input)
}
pub fn if_then_to(input: &str) -> IResult<&str, usize> {
    let start = tuple((
        tag("If "),
        alt((tag("true"), tag("false"))),
        tag(": throw to monkey "),
    ));
    let (input, index) = preceded(start, cc::u32)(input)?;
    Ok((input, index as usize))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monkey {
    items: VecDeque<IntItem>,
    operation: NewOperation,
    test: IntItem,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    pub fn monkey(input: &str) -> IResult<&str, Self> {
        let (input, _) = preceded(multispace0, tuple((tag("Monkey "), int_item, tag(":"))))(input)?;

        let (input, items) = remove_spaces(start_items)(input)?;
        let (input, operation) = remove_spaces(NewOperation::new_operation)(input)?;
        let (input, test) = remove_spaces(test)(input)?;
        let (input, if_true) = remove_spaces(if_then_to)(input)?;
        let (input, if_false) = remove_spaces(if_then_to)(input)?;

        Ok((
            input,
            Self {
                items,
                operation,
                test,
                if_true,
                if_false,
            },
        ))
    }

    pub fn run_round(&mut self, div_factor: IntItem) -> Vec<(IntItem, usize)> {
        let mut transfers = Vec::with_capacity(self.items.len());

        for mut next_item in std::mem::take(&mut self.items) {
            next_item = self
                .operation
                .run(next_item)
                .div_round_down(div_factor.clone());

            transfers.push((
                next_item.clone(),
                if next_item.mod_floor(&self.test) == IntItem::zero() {
                    self.if_true
                } else {
                    self.if_false
                },
            ));
        }

        transfers
    }

    pub fn add_item(&mut self, i: IntItem) {
        self.items.push_back(i);
    }

    #[allow(dead_code)]
    pub fn clone_items(&self) -> Vec<IntItem> {
        self.items.clone().into()
    }

    pub fn clear_items(&mut self) {
        self.items.clear();
    }
}

pub fn parse_multiple_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let no_lines = dbg!(input.lines().count());
    let number = dbg!(no_lines.div_round_down(6) - 1);
    count(Monkey::monkey, number)(input)
}
