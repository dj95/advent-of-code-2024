pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}


pub fn part_one(inp: &str) -> String {
    "".to_string()
}

pub fn part_two(inp: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = "";

        let res = part_one(&input);

        assert_eq!(res, "");
    }

    #[test]
    pub fn test_part_two() {
        let input = "";

        let res = part_two(&input);

        assert_eq!(res, "");
    }
}
