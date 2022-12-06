use std::str::FromStr;

struct Stacks {
    stacks: Vec<Stack>,
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let indexes = s.lines()
            .filter(|line| match line.chars().nth(1) {
                Some(x) => x.is_digit(10),
                None => false,
            })
            .flat_map(|x| x.trim().split("   ").flat_map(|y| y.parse::<u8>()));

        let mut stacks = vec![];
        // for i in indexes {
        //     println!("{}", i);
        //     stacks.push(Stack {
        //         index: i,
        //         crates: vec![],
        //     })
        // }

        let mut looking = true;
        for line in s.lines().rev() {
            if looking {
                if line.starts_with("[") {
                    looking = false;
                } else {
                    continue;
                }
            }

            let line = line.replace("[", "").replace("]", "");
            println!("{}", line);
            stacks.push(line);
        }

        Ok(Stacks {
            stacks: vec![],
        })
    }
}

struct Stack {
    index: u8,
    crates: Vec<char>,
}

fn part_one(input: &str) -> &str {
    let stacks = input.parse::<Stacks>();
    ""
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day_05.test").unwrap();
    println!("{}", part_one(&input));
}
