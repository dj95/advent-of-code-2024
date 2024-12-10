pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

fn expand(inp: &str) -> Vec<i64> {
    let mut character = -1;

    inp.chars()
        .map(|c: char| {
            tracing::debug!("{c:?}");
            if c == '\n' {
                return vec![];
            }

            let count = c.to_digit(10).unwrap();
            character += 1;

            let char = match character % 2 {
                0 => character / 2,
                1 => -1,
                _ => panic!(),
            };

            vec![char; count as usize]
        })
        .flatten()
        .collect()
}

fn defrag(disk: &mut Vec<i64>) {
    let mut cursor_left = disk.iter().position(|&x| x == -1).unwrap();
    let mut cursor_right = disk.len() - 1;

    while cursor_right > cursor_left {
        tracing::debug!("{cursor_left:?} {cursor_right:?}");

        if disk[cursor_right] == -1 {
            cursor_right -= 1;

            continue;
        }

        disk[cursor_left] = disk[cursor_right];
        disk[cursor_right] = -1;

        cursor_left = disk.iter().position(|&x| x == -1).unwrap();
        cursor_right -= 1;
    }
}

fn defrag_complete_files(inp: &str) -> Vec<i64> {
    let inp = match inp.strip_suffix('\n') {
        Some(x) => x,
        None => inp,
    };

    let mut data_map = inp
        .chars()
        .zip(0..inp.chars().count())
        .map(|(c, idx)| {
            let char = match idx % 2 {
                0 => idx as i64 / 2,
                1 => -1,
                _ => panic!(),
            };

            (c.to_digit(10).unwrap() as i64, char)
        })
        .collect::<Vec<(i64, i64)>>();

    let mut cursor_right = data_map.len() - 1;

    while cursor_right > 0 {
        let element = data_map[cursor_right];

        if element.1 < 0 {
            cursor_right -= 1;
            continue;
        }

        let candidate = data_map
            .iter()
            .position(|(count, idx)| *count >= element.0 && *idx == -1);

        if let Some(new_index) = candidate {
            if cursor_right > new_index {
                let space = data_map[new_index];

                if space.0 == element.0 {
                    data_map[new_index] = element;
                    data_map[cursor_right] = space;
                } else {
                    data_map[cursor_right] = (element.0, -1);
                    data_map[new_index] = element;
                    data_map.insert(new_index + 1, (space.0 - element.0, -1));
                }
            }
        }

        cursor_right -= 1;
    }

    data_map
        .iter()
        .map(|(count, char)| vec![*char; *count as usize])
        .flatten()
        .collect()
}

fn checksum(disk: &mut Vec<i64>) -> i64 {
    disk.iter()
        .zip(0..disk.len())
        .filter(|el| *el.0 > 0)
        .map(|(count, index)| count * index as i64)
        .sum()
}

pub fn part_one(inp: &str) -> i64 {
    let disk = &mut expand(inp);

    defrag(disk);

    checksum(disk)
}

pub fn part_two(inp: &str) -> i64 {
    let disk = &mut defrag_complete_files(inp);

    checksum(disk)
}

#[cfg(test)]
mod test {
    use crate::*;
    use rstest::*;

    #[rstest]
    #[case(&mut vec![0,0,9,9,8,1,1,1,8,8,8,2,7,7,7,3,3,3,6,4,4,6,5,5,5,5,6,6,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1], 1928)]
    #[test_log::test]
    pub fn test_checksum(#[case] disk: &mut Vec<i64>, #[case] expected_result: i64) {
        let result = checksum(disk);

        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(
        "2333133121414131402",
        &mut vec![0,0,9,9,2,1,1,1,7,7,7,-1,4,4,-1,3,3,3,-1,-1,-1,-1,5,5,5,5,-1,6,6,6,6,-1,-1,-1,-1,-1,8,8,8,8,-1,-1],
    )]
    #[test_log::test]
    pub fn test_defrag_complete_files(#[case] input: &str, #[case] expected_result: &mut Vec<i64>) {
        let input = &mut defrag_complete_files(input);

        assert_eq!(input, expected_result);
    }

    #[rstest]
    #[case(&mut vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2], &mut vec![0, 2, 2 ,1, 1, 1, 2, 2, 2, -1, -1, -1, -1, -1, -1])]
    #[test_log::test]
    pub fn test_defrag(#[case] input: &mut Vec<i64>, #[case] expected_result: &mut Vec<i64>) {
        defrag(input);

        assert_eq!(input, expected_result);
    }

    #[rstest]
    #[case("12345", vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2])]
    #[case("2333133121414131402", vec![0,0,-1,-1,-1,1,1,1,-1,-1,-1,2,-1,-1,-1,3,3,3,-1,4,4,-1,5,5,5,5,-1,6,6,6,6,-1,7,7,7,-1,8,8,8,8,9,9])]
    #[test_log::test]
    pub fn test_expand(#[case] input: &str, #[case] expected_result: Vec<i64>) {
        let result = expand(input);

        assert_eq!(result, expected_result);
    }

    #[test_log::test]
    pub fn test_part_one() {
        let input = "2333133121414131402";

        let res = part_one(&input);

        assert_eq!(res, 1928);
    }

    #[test_log::test]
    pub fn test_part_two() {
        let input = "2333133121414131402";

        let res = part_two(&input);

        assert_eq!(res, 2858);
    }
}
