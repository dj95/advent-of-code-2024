use std::collections::HashMap;

use nom::{
    character::{
        complete,
        streaming::{multispace1, newline},
    },
    combinator::opt,
    sequence::tuple,
    IResult,
};

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

fn parse_lists(inp: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    let mut inp = inp;
    while !inp.is_empty() {
        tracing::trace!("{inp}");
        let (line_inp, tuple) = tuple((complete::i32, multispace1, complete::i32))(inp)?;

        left_list.push(tuple.0);
        right_list.push(tuple.2);

        tracing::debug!("{tuple:?}");
        let (line_inp, _) = opt(newline)(line_inp)?;
        inp = line_inp;
    }

    Ok(("", (left_list, right_list)))
}

pub fn part_one(inp: &str) -> i32 {
    let res = match parse_lists(inp) {
        Ok(res) => res.1,
        Err(e) => panic!("{e}"),
    };

    let mut left = res.0;
    let mut right = res.1;

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub fn part_two(inp: &str) -> i32 {
    let res = match parse_lists(inp) {
        Ok(res) => res.1,
        Err(e) => panic!("{e}"),
    };

    let max = res
        .1
        .into_iter()
        .fold(HashMap::<i32, usize>::new(), |mut m, x| {
            *m.entry(x).or_default() += 1;
            m
        });

    tracing::debug!("{max:?}");

    res.0
        .iter()
        .map(|&e| *max.get(&e).unwrap_or(&0) as i32 * e)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test_log::test]
    pub fn test_part_one() {
        let input = "3   4\n\
            4   3\n\
            2   5\n\
            1   3\n\
            3   9\n\
            3   3\n"
            .to_owned();

        let res = part_one(&input);

        assert_eq!(res, 11);
    }

    #[test_log::test]
    pub fn test_part_two() {
        let input = "3   4\n\
            4   3\n\
            2   5\n\
            1   3\n\
            3   9\n\
            3   3\n"
            .to_owned();

        let res = part_two(&input);

        assert_eq!(res, 31);
    }
}
