fn main() {
    let input = std::fs::read_to_string("./src/bin/day_04.txt").expect("file");
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").expect("split comma");
            let left = Sections::from(left);
            let right = Sections::from(right);

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
        .map(|line| {
            let (left, right) = line.split_once(",").expect("split comma");
            let left = Sections::from(left);
            let right = Sections::from(right);

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
