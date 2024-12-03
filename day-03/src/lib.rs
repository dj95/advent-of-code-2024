use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{char, u32},
        streaming::anychar,
    },
    combinator::{map, value},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

fn parse_mul(input: &str) -> IResult<&str, Instructions> {
    map(
        delimited(tag("mul("), separated_pair(u32, char(','), u32), char(')')),
        |res| Instructions::Mul(res.0, res.1),
    )(input)
}

fn parse_muls(input: &str) -> IResult<&str, Vec<Instructions>> {
    tracing::debug!("{input}");
    many1(nom::combinator::complete(map(
        many_till(anychar, parse_mul),
        |(_, res)| res,
    )))(input)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instructions {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_muls_with_instructions(input: &str) -> IResult<&str, Vec<Instructions>> {
    many1(nom::combinator::complete(map(
        many_till(
            anychar,
            alt((
                value(Instructions::Do, tag("do()")),
                value(Instructions::Dont, tag("don't()")),
                parse_mul,
            )),
        ),
        |(_, res)| res,
    )))(input)
}

pub fn part_one(inp: &str) -> u32 {
    let instructions = parse_muls(inp).unwrap().1;

    instructions
        .iter()
        .map(|ins| match ins {
            Instructions::Mul(x, y) => x * y,
            Instructions::Do => 0,
            Instructions::Dont => 0,
        })
        .sum()
}

pub fn part_two(inp: &str) -> u32 {
    let instructions = parse_muls_with_instructions(inp).unwrap().1;

    tracing::debug!("----- {instructions:?}");

    let mut sum = 0;
    let mut should_add = true;
    for instruction in instructions.iter() {
        match instruction {
            Instructions::Mul(x, y) => {
                if should_add {
                    sum += x * y;
                }
            }
            Instructions::Do => {
                should_add = true;
            }
            Instructions::Dont => should_add = false,
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test_log::test]
    pub fn test_parse_mul() {
        let res = parse_mul("mul(1,2)");
        assert_eq!(res, Ok(("", Instructions::Mul(1, 2))));
    }

    #[test_log::test]
    pub fn test_parse_muls() {
        let res = parse_muls("asldfjasldfjasdmul(1,2)asldfja&$(*&)mul(2,3)lasdjfas");
        assert_eq!(
            res,
            Ok((
                "lasdjfas",
                vec![Instructions::Mul(1, 2), Instructions::Mul(2, 3)]
            ))
        );
    }

    #[test_log::test]
    pub fn test_part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let res = part_one(input);

        assert_eq!(res, 161);
    }

    #[test_log::test]
    pub fn test_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let res = part_two(input);

        assert_eq!(res, 48);
    }
}
