use std::{cmp::Ordering, str::FromStr};

use anyhow::Result;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Data {
    Single(usize),
    List(Vec<Data>),
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Data::Single(l), Data::Single(r)) => l.cmp(r),
            (Data::Single(l), Data::List(r)) => vec![Data::Single(*l)].cmp(r),
            (Data::List(l), Data::Single(r)) => l.cmp(&vec![Data::Single(*r)]),
            (Data::List(l), Data::List(r)) => l.cmp(r),
        }
    }
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let depth = s.chars().filter(|x| *x == '[').count();
        if depth == 0 {
            return Ok(Data::Single(s.parse().unwrap()));
        }

        if depth == 1 {
            let values = s
                .replace("[", "")
                .replace("]", "")
                .split(",")
                .map(|x| x.parse::<usize>())
                .filter(|x| x.is_ok())
                .map(|x| Data::Single(x.unwrap()))
                .collect::<Vec<Data>>();
            return Ok(Data::List(values));
        }

        let mut list = vec![];
        let mut from_index = 1;
        let mut bracket_counter = 0;

        for (i, char) in s.chars().enumerate() {
            if char == '[' {
                bracket_counter += 1;
            }
            if char == ']' {
                bracket_counter -= 1;
            }
            if (char == ',' && bracket_counter == 1) || (char == ']' && bracket_counter == 0) {
                let substring = &s[from_index..i];
                list.push(substring.parse::<Data>().unwrap());
                from_index = i + 1;
            }
        }

        return Ok(Data::List(list));
    }
}

fn part_two(input: &str) -> usize {
    let dividers = [
        Data::List(vec![Data::List(vec![Data::Single(2)])]),
        Data::List(vec![Data::List(vec![Data::Single(6)])]),
    ];

    let mut packets = input
        .replace("\n\n", "\n")
        .lines()
        .map(|line| line.parse::<Data>().unwrap())
        .collect::<Vec<Data>>();

    packets.extend(dividers.clone());
    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter(|x| dividers.contains(x.1))
        .map(|x| x.0 + 1)
        .product()
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|x| {
            let (left, right) = x.split_once("\n").unwrap();
            let left = left.parse::<Data>().unwrap();
            let right = right.parse::<Data>().unwrap();
            (left, right)
        })
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, _)| i + 1)
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./src/bin/day_13.txt")?;
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./src/bin/day_13.test").unwrap();

        assert_eq!(super::part_one(&input), 13);
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./src/bin/day_13.test").unwrap();

        assert_eq!(super::part_two(&input), 140);
    }
}
