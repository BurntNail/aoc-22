use crate::{int_item, IntItem};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    multi::{count, separated_list0},
    sequence::tuple,
    IResult,
};
use std::cmp::Ordering;
use utilities::vec_utils::VecUtils;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Item {
    List(Vec<Self>),
    Literal(IntItem),
}

pub fn compare(first: Item, second: Item, padding: Option<String>) -> Ordering {
    let padding = padding.unwrap_or_default() + "\t";
    println!("{padding}Comparing {first:?} and {second:?}");
    let res = match (first, second) {
        (Item::Literal(first), Item::Literal(second)) => first.cmp(&second),
        (Item::List(mut first), Item::List(mut second)) => {
            let mut res = None;

            for _ in 0..first.len() {
                if second.is_empty() {
                    res = Some(Ordering::Greater);
                    break;
                }

                let cmp = compare(first.remove(0), second.remove(0), Some(padding.clone()));
                if matches!(cmp, Ordering::Less | Ordering::Greater) {
                    res = Some(cmp);
                    break;
                }
            }

            res.unwrap_or(Ordering::Equal)
        }
        (Item::Literal(first), Item::List(second)) => compare(
            Item::List(vec![Item::Literal(first)]),
            Item::List(second),
            Some(padding.clone()),
        ),
        (Item::List(first), Item::Literal(second)) => compare(
            Item::List(first),
            Item::List(vec![Item::Literal(second)]),
            Some(padding.clone()),
        ),
    };
    println!("{padding}{res:?}");
    res
}

impl Item {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let interior = separated_list0(tag(","), alt((map(int_item, Item::Literal), Item::parse)));

        let (input, (_, items, _)) = tuple((alt((tag("["), tag(","))), interior, tag("]")))(input)?;

        Ok((input, Item::List(items)))
    }
}

#[derive(Clone, Debug)]
pub struct Pair(Item, Item);
impl Pair {
    pub fn compare(self) -> Ordering {
        compare(self.0, self.1, None)
    }
}

impl Pair {
    pub fn get_pairs(input: &str) -> IResult<&str, Vec<Self>> {
        let (input, v) = count(
            tuple((Item::parse, multispace0, Item::parse, multispace0)),
            (input.lines().count() + 1) / 3,
        )(input)?;

        let v = v.map(|(first, _, second, _)| Pair(first, second));
        Ok((input, v))
    }
}
