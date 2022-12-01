use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Input could not be openend");
    let reader = BufReader::new(file);

    let mut sum: Vec<usize> = Vec::new();
    let mut calories_counter: usize = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            sum.push(calories_counter);
            calories_counter = 0;
            continue;
        }
        let calories = usize::from_str_radix(&line, 10).expect("Calories were not a number");
        calories_counter = calories_counter + calories;
    }

    sum.sort_by(|a, b| b.cmp(a));
    let total: usize = (0..3).map(|i| sum[i]).sum();

    println!("{} are the highest calories carried", sum[0]);
    println!("{} calories are carried by the top three Elves", total);
}
