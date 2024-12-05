pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> u32 {
    let rules_and_updates = inp.split("\n\n").collect::<Vec<&str>>();
    tracing::debug!("{rules_and_updates:?}");
    let rules = parse_rules(rules_and_updates[0]);
    let updates = parse_updates(rules_and_updates[1]);

    updates
        .iter()
        .filter(|update| check_update(&rules, update))
        .map(|x| get_center_element(x))
        .sum()
}

fn get_center_element(update: &[u32]) -> u32 {
    update[update.len() / 2]
}

fn check_update(rules: &[(u32, u32)], update: &[u32]) -> bool {
    rules
        .iter()
        .map(|rule| {
            let left = match update.iter().position(|&x| x == rule.0) {
                Some(x) => x,
                None => return true,
            };
            let right = match update.iter().position(|&x| x == rule.1) {
                Some(x) => x,
                None => return true,
            };

            left < right
        })
        .all(|x| x)
}

#[tracing::instrument]
fn parse_updates(update_input: &str) -> Vec<Vec<u32>> {
    update_input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

#[tracing::instrument]
fn parse_rules(rules_string: &str) -> Vec<(u32, u32)> {
    rules_string
        .lines()
        .map(|line| {
            let numbers = line
                .split('|')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (numbers[0], numbers[1])
        })
        .collect()
}

pub fn part_two(inp: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test_log::test]
    pub fn test_check_update() {
        let rules = vec![(75, 47)];
        let update = vec![75, 47, 61, 53, 29];
        assert!(check_update(&rules, &update));
    }

    #[test_log::test]
    pub fn test_parse_updates() {
        let result = parse_updates(
            "97,61,53,29,13
75,29,13",
        );
        assert_eq!(result, vec![vec![97, 61, 53, 29, 13], vec![75, 29, 13]]);
    }

    #[test_log::test]
    pub fn test_parse_rules() {
        let result = parse_rules(
            "47|53
97|13",
        );
        assert_eq!(result, vec![(47, 53), (97, 13)]);
    }

    #[test_log::test]
    pub fn test_part_one() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let res = part_one(input);

        assert_eq!(res, 143);
    }

    #[test]
    pub fn test_part_two() {
        let input = "";

        let res = part_two(input);

        assert_eq!(res, "");
    }
}
