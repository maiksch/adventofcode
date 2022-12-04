fn main() {
    let input = include_str!("day_04.txt");

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .map(|line| {
            let left = Sections::from(line[0]);
            let right = Sections::from(line[1]);
            left.contains(&right) || right.contains(&left)
        })
        .map(|contains| match contains {
            true => 1,
            false => 0,
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .map(|line| {
            let left = Sections::from(line[0]);
            let right = Sections::from(line[1]);
            left.overlap(&right) || right.overlap(&left)
        })
        .map(|contains| match contains {
            true => 1,
            false => 0,
        })
        .sum()
}

#[derive(Debug)]
struct Sections {
    from: u64,
    to: u64,
}

impl From<&str> for Sections {
    fn from(value: &str) -> Self {
        let v: Vec<u64> = value.split("-").map(|x| x.parse().unwrap()).collect();
        Self {
            from: v[0],
            to: v[1],
        }
    }
}

impl Sections {
    fn contains(&self, other: &Sections) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlap(&self, other: &Sections) -> bool {
        self.from <= other.from && self.to >= other.from
    }
}
