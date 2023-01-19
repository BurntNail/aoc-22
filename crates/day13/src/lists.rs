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
use std::{
    cmp::{Ord, Ordering},
    fmt::{Display, Formatter},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Item {
    List(Vec<Self>),
    Literal(IntItem),
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::List(l) => {
                write!(f, "[")?;
                if !l.is_empty() {
                    let ll = l.len() - 1;
                    for item in l.iter().take(ll) {
                        write!(f, "{item},")?;
                    }
                    write!(f, "{}", l[ll])?;
                }
                write!(f, "]")
            }
            Item::Literal(a) => write!(f, "{a}"),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Literal(first), Item::Literal(second)) => first.cmp(second),
            (Item::List(first), Item::List(second)) => {
                let mut first = first.clone();
                let mut second = second.clone();

                let mut res = None;

                let fl = first.len();
                let sl = second.len();

                for _ in 0..fl {
                    if second.is_empty() {
                        break;
                    }

                    let cmp = first.remove(0).cmp(&second.remove(0));
                    if cmp != Ordering::Equal {
                        res = Some(cmp);
                        break;
                    }
                }

                res.unwrap_or_else(|| fl.cmp(&sl))
            }
            (Item::Literal(first), Item::List(second)) => {
                Item::List(vec![Item::Literal(*first)]).cmp(&Item::List(second.clone()))
            }
            (Item::List(first), Item::Literal(second)) => Item::List(first.clone()).cmp(
                &Item::List(vec![Item::Literal(*second)]), //TODO: look at changing this to work with slices/references
            ),
        }
    }
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
    pub fn compare(&self) -> Ordering {
        self.0.cmp(&self.1)
    }
}

impl Pair {
    pub fn get_all(input: &str) -> IResult<&str, Vec<Item>> {
        let (input, v) = count(
            tuple((Item::parse, multispace0, Item::parse, multispace0)),
            (input.lines().count() + 1) / 3,
        )(input)?;

        let v = v
            .into_iter()
            .flat_map(|(first, _, second, _)| vec![first, second])
            .collect();
        Ok((input, v))
    }

    pub fn get_pairs(mut v: Vec<Item>) -> Vec<Self> {
        let mut v2 = vec![];

        for _ in 0..v.len() / 2 {
            v2.push(Self(v.remove(0), v.remove(0)));
        }

        v2
    }
}
