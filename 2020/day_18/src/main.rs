use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1 as digit, space0 as space},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
  };
  
use std::str::FromStr;

fn parens(i: &str) -> IResult<&str, i64> {
    delimited(space, delimited(tag("("), expr, tag(")")), space)(i)
}

fn factor(i: &str) -> IResult<&str, i64> {
    alt((
        map_res(delimited(space, digit, space), FromStr::from_str),
        parens,
    ))(i)
}
  
fn term(i: &str) -> IResult<&str, i64> {
    let (i, init) = factor(i)?;

    fold_many0(
        pair(alt((char('+'), char('-'))), factor),
        init,
        |acc, (op, val): (char, i64)| {
        if op == '+' {
            acc + val
        } else {
            acc - val
        }
        },
    )(i)
}
  
fn expr(i: &str) -> IResult<&str, i64> {
    let (i, init) = term(i)?;

    fold_many0(
        pair(alt((char('*'), char('/'))), term),
        init,
        |acc, (op, val): (char, i64)| {
        if op == '*' {
            acc * val
        } else {
            acc / val
        }
        },
    )(i)
}
  
fn main() {
    let input = 
        include_str!("input.txt").trim()
        .lines()
        .map(|line| expr(line).map(|(_, x)| x).expect("bad expr"));

    let result: i64 = input.sum();

    println!("{:?}", result);
}

#[test]
fn test() {
    assert_eq!(expr("1 + (2 * 3) + (4 * (5 + 6))"), Ok(("", 51)));
    assert_eq!(expr("2 * 3 + (4 * 5)"), Ok(("", 46)));
    assert_eq!(expr("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(("", 1445)));
    assert_eq!(expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), Ok(("", 669060)));
    assert_eq!(expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), Ok(("", 23340)));
}
