use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input/day_01.txt").expect("Input could not be openend");
    let reader = BufReader::new(file);

    let mut sum = vec![];
    let mut calories_counter = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            sum.push(calories_counter);
            calories_counter = 0;
            continue;
        }
        let calories = str::parse::<usize>(&line).expect("Calories were not a number");
        calories_counter = calories_counter + calories;
    }

    sum.sort_by(|a, b| b.cmp(a));

    println!("{} are the highest calories carried", sum[0]);

    let total = sum[0..3].iter().sum::<usize>();

    println!("{} calories are carried by the top three Elves", total);
}
