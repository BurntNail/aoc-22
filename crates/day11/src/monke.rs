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
use std::collections::VecDeque;
use utilities::{int_utils::DivExt, nom_utils::remove_spaces};

type IntItem = u128;
fn int_item(input: &str) -> IResult<&str, IntItem> {
    cc::u128(input)
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
        Ok((input, NewOperation(op, term)))
    }

    pub const fn run(&self, old: IntItem) -> IntItem {
        let term = match self.1 {
            Term::Literal(l) => l,
            Term::Old => old,
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

    pub fn run_round(&mut self) -> Vec<(IntItem, usize)> {
        let mut transfers = Vec::with_capacity(self.items.len());

        for mut next_item in std::mem::take(&mut self.items) {
            next_item = self.operation.run(next_item).div_round_down(3);

            transfers.push((
                next_item,
                if next_item % self.test == 0 {
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

    pub fn clone_items(&self) -> Vec<IntItem> {
        self.items.clone().into()
    }
}

pub fn parse_multiple_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let no_lines = dbg!(input.lines().count());
    let number = dbg!(no_lines.div_round_down(6) - 1);
    count(Monkey::monkey, number)(input)
}

#[cfg(test)]
mod tests {
    use crate::monke::{start_items, Monkey, NewOperation, Operation, Term};
    use nom::{character::complete::multispace0, sequence::preceded};
    use std::collections::VecDeque;

    #[test]
    pub fn monke_test() -> Result<(), Box<dyn std::error::Error>> {
        let mnk = r#"
    Monkey 0:
      Starting items: 61
      Operation: new = old * 11
      Test: divisible by 5
        If true: throw to monkey 7
        If false: throw to monkey 4
    "#;
        let expected = Monkey {
            items: VecDeque::from(vec![61]),
            operation: NewOperation(Operation::Mul, Term::Literal(11)),
            test: 5,
            if_true: 7,
            if_false: 4,
        };

        let actual = Monkey::monkey(mnk)?.1;
        assert_eq!(actual, expected);

        let mnk = r#"
Monkey 1:
  Starting items: 76, 92, 53, 93, 79, 86, 81
  Operation: new = old + 4
  Test: divisible by 2
    If true: throw to monkey 2
    If false: throw to monkey 6
"#;
        let expected = Monkey {
            items: VecDeque::from(vec![76, 92, 53, 93, 79, 86, 81]),
            operation: NewOperation(Operation::Add, Term::Literal(4)),
            test: 2,
            if_true: 2,
            if_false: 6,
        };

        let actual = Monkey::monkey(mnk)?.1;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    pub fn start_items_test() {
        let raw = "                                     \nStarting items: 61\nghjksvdfhjksdhf";
        let (_, items) = preceded(multispace0, start_items)(raw).unwrap();
        assert_eq!(items, VecDeque::from(vec![61]));
    }
}
