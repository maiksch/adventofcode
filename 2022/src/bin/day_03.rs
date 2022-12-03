const PRIORITIES: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() {
    let input = include_str!("day_03.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let length = line.len();
            let left = &line[0..length / 2];
            let right = &line[length / 2..];
            let item = find_item(left, right);
            get_priority(&item)
        })
        .sum()
}

fn find_item(left: &str, right: &str) -> char {
    left.chars()
        .find_map(|x| match right.contains(x) {
            true => Some(x),
            _ => None,
        })
        .unwrap()
}

fn get_priority(item: &char) -> usize {
    let priorities = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    match item {
        x if x.is_lowercase() => priorities.iter().position(|&y| y == *x).unwrap() + 1,
        x if x.is_uppercase() => {
            priorities
                .iter()
                .position(|&y| x.to_lowercase().eq(y.to_lowercase()))
                .unwrap()
                + 27
        }
        _ => unimplemented!("invalid priority"),
    }
}

fn part_two(input: &str) -> usize {
    let mut from = 0;
    let mut to = 3;
    let mut groups: Vec<Vec<&str>> = vec![];
    let lines: Vec<&str> = input.lines().collect();
    let mut priority = 0;

    while to <= lines.len() {
        groups.push(lines[from..to].to_vec());
        from += 3;
        to += 3;
    }

    for group in groups {
        let mut prio: Option<usize>;

        prio = find_priority(&group, &PRIORITIES.clone().map(|x| x.to_string()));

        if prio.is_none() {
            let priorities = PRIORITIES.clone().map(|x| x.to_uppercase().to_string());
            prio = find_priority(&group, &priorities);
        }

        if let Some(v) = prio {
            priority += v;
        }
    }

    priority
}

fn find_priority(group: &Vec<&str>, priorities: &[String; 26]) -> Option<usize> {
    priorities.iter().find_map(|prio| {
        let mut found = false;
        for rucksack in group.iter() {
            let mut rucksack = rucksack.chars();
            found = match rucksack.find(|x| x.to_string() == *prio) {
                Some(_) => true,
                None => return None,
            };
        }
        if found {
            return Some(get_priority_2(prio, priorities));
        }

        return None;
    })
}

fn get_priority_2(item: &String, priorities: &[String; 26]) -> usize {
    let item: char = item.chars().nth(0).unwrap();
    match item {
        x if x.is_lowercase() => priorities.iter().position(|y| *y == x.to_string()).unwrap() + 1,
        x if x.is_uppercase() => {
            priorities
                .iter()
                .position(|y| *y == x.to_uppercase().to_string())
                .unwrap()
                + 27
        }
        _ => unimplemented!("invalid priority"),
    }
}
