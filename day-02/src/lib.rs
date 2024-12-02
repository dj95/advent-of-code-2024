use nom::{bytes::complete::tag, character::complete, multi::separated_list0, IResult};

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn parse_grid(inp: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let mut output: Vec<Vec<i32>> = vec![];

    for line in inp.lines() {
        tracing::debug!("{line:?}");
        let (_, vector) = separated_list0(tag(" "), complete::i32)(line)?;
        output.push(vector);
    }

    Ok((inp, output))
}

#[derive(PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
}

#[tracing::instrument]
pub fn is_safe(row: &Vec<i32>) -> bool {
    let mut direction: Option<Direction> = None;
    let mut invalid_dir = false;
    let invalid_levels = row
        .iter()
        .zip(row.iter().skip(1))
        .filter(|(&x, &y)| {
            if invalid_dir {
                true;
            }

            let res = y - x;

            if res == 0 {
                return true;
            }

            let new_dir = match res < 0 {
                true => Direction::Down,
                false => Direction::Up,
            };

            if let Some(dir) = &direction {
                if dir != &new_dir {
                    invalid_dir = true
                }
            }

            direction = Some(new_dir);

            let x = res.abs();
            x <= 0 || x > 3
        })
        .count();

    invalid_levels == 0 && !invalid_dir
}

#[tracing::instrument]
pub fn is_safe_with_damper(row: &Vec<i32>) -> bool {
    if is_safe(row) {
        return true;
    }

    for i in 0..row.len() {
        let mut new_row = row.clone();
        new_row.remove(i);

        if is_safe(&new_row) {
            return true;
        }
    }

    false
}

pub fn part_one(inp: &str) -> i32 {
    let grid = match parse_grid(inp) {
        Ok((_, val)) => val,
        Err(e) => panic!("{e}"),
    };

    tracing::debug!("{grid:?}");

    grid.iter().filter(|&x| is_safe(x)).count() as i32
}

pub fn part_two(inp: &str) -> i32 {
    let grid = match parse_grid(inp) {
        Ok((_, val)) => val,
        Err(e) => panic!("{e}"),
    };

    tracing::debug!("{grid:?}");

    grid.iter().filter(|&x| is_safe_with_damper(x)).count() as i32
}

#[cfg(test)]
mod test {
    use crate::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![7, 6, 4, 2, 1],  true)]
    #[case(vec![1, 2, 7, 8, 9],  false)]
    #[case(vec![9, 7, 6, 2, 1],  false)]
    #[case(vec![1, 3, 2, 4, 5],  false)]
    #[case(vec![8, 6, 4, 4, 1],  false)]
    #[case(vec![1, 3, 6, 7, 9],  true)]
    #[test_log::test]
    pub fn test_is_safe(#[case] row: Vec<i32>, #[case] expected: bool) {
        let res = is_safe(&row);

        assert_eq!(expected, res);
    }

    #[rstest]
    #[case(vec![7, 6, 4, 2, 1],  true)]
    #[case(vec![1, 2, 7, 8, 9],  false)]
    #[case(vec![9, 7, 6, 2, 1],  false)]
    #[case(vec![1, 3, 2, 4, 5],  true)]
    #[case(vec![8, 6, 4, 4, 1],  true)]
    #[case(vec![1, 3, 6, 7, 9],  true)]
    #[test_log::test]
    pub fn test_is_safe_with_damper(#[case] row: Vec<i32>, #[case] expected: bool) {
        let res = is_safe_with_damper(&row);

        assert_eq!(expected, res);
    }

    #[test_log::test]
    pub fn test_part_one() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

        let res = part_one(input);

        assert_eq!(res, 2);
    }

    #[test]
    pub fn test_part_two() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

        let res = part_two(input);

        assert_eq!(res, 4);
    }
}
