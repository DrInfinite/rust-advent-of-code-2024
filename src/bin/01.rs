use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    let sum = left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum();

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);

    let mut seen_ids = HashMap::new();

    let sum: u32 = left
        .iter()
        .map(|id| {
            let count = seen_ids
                .entry(id)
                .or_insert_with(|| count_occurrences(id, &right));
            return *id * *count;
        })
        .sum();

    return Some(sum);
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lines = input.lines();

    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in lines {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<u32>().unwrap());
        right.push(split.next().unwrap().parse::<u32>().unwrap())
    }

    return (left, right);
}

fn count_occurrences(id: &u32, list: &Vec<u32>) -> u32 {
    return list.iter().filter(|value| **value == *id).count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
