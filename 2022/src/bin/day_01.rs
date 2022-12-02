use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("./src/bin/day_01.txt").expect("Input could not be openend");
    let reader = BufReader::new(file);

    let mut sum = vec![];
    let mut calories_counter = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        let calories = str::parse::<usize>(&line);

        if calories.is_ok() {
            calories_counter += calories.unwrap();
        }

        sum.push(calories_counter);
        calories_counter = 0;
    }

    sum.sort_by(|a, b| b.cmp(a));

    println!("{} are the highest calories carried", sum[0]);

    let total = sum[0..3].iter().sum::<usize>();

    println!("{} calories are carried by the top three Elves", total);
}
