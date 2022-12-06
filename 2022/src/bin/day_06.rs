fn main() {
    let input = std::fs::read_to_string("./src/bin/day_06.txt").expect("file to be there");
    println!("{}", solve(&input, 4));
    println!("{}", solve(&input, 14));
}

fn solve(input: &str, unique_count: usize) -> usize {
    let mut idx = unique_count;

    while idx <= input.len() {
        let characters_to_check = &input[idx - unique_count..idx];
        if !has_double_character(characters_to_check) {
            return idx;
        }
        idx += 1;
    }

    0 // this will never be reached
}

fn has_double_character(s: &str) -> bool {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars[..chars.len()-1]
        .iter()
        .enumerate()
        // compares each character with its successor
        .find_map(|(i, &x)| match x == chars[i+1] {
            true => Some(()),
            false => None,
        })
        .is_some()
}

#[cfg(test)]
mod test {
    #[test]
    fn part_one() {
        assert_eq!(super::solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(super::solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(super::solve("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(super::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(super::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(super::solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(super::solve("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(super::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(super::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
