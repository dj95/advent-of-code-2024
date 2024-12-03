use nom::{
    bytes::complete::tag,
    character::{
        complete::{char, u32},
        streaming::anychar,
    },
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

type Mul = (u32, u32);

fn parse_mul(input: &str) -> IResult<&str, Mul> {
    delimited(tag("mul("), separated_pair(u32, char(','), u32), char(')'))(input)
}

fn parse_with_garbage(input: &str) -> IResult<&str, Mul> {
    many_till(anychar, parse_mul)(input)
        .inspect(|e| tracing::debug!("{e:?}"))
        .map(|(inp, res)| (inp, res.1))
}

fn parse_muls(input: &str) -> IResult<&str, Vec<Mul>> {
    tracing::debug!("{input}");
    many1(nom::combinator::complete(parse_with_garbage))(input)
}

pub fn parse(inp: &str) -> Result<Vec<Mul>, String> {
    match parse_muls(inp) {
        Ok((_, result)) => Ok(result),
        Err(err) => Err(format!("Failed to parse input: {:?}", err)),
    }
}

pub fn part_one(inp: &str) -> u32 {
    let instructions = parse(inp).unwrap();

    instructions.iter().map(|(x, y)| x * y).sum()
}

pub fn part_two(inp: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test_log::test]
    pub fn test_parse_mul() {
        let res = parse_mul("mul(1,2)");
        assert_eq!(res, Ok(("", (1, 2))));
    }

    #[test_log::test]
    pub fn test_parse_with_garbage() {
        let res = parse_with_garbage("asldfjasldfjasdmul(1,2)");
        assert_eq!(res, Ok(("", (1, 2))));
    }

    #[test_log::test]
    pub fn test_parse_muls() {
        let res = parse_muls("asldfjasldfjasdmul(1,2)asldfja&$(*&)mul(2,3)lasdjfas");
        assert_eq!(res, Ok(("lasdjfas", vec![(1, 2), (2, 3)])));
    }

    #[test_log::test]
    pub fn test_part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let res = part_one(input);

        assert_eq!(res, 161);
    }

    #[test_log::test]
    pub fn test_part_two() {
        let input = "";

        let res = part_two(input);

        assert_eq!(res, "");
    }
}
