
use std::cmp::Ordering;
fn main() {
    let part_one = part_one();
    println!("{}", part_one);
    let part_two = part_two();
    println!("{}", part_two);
}

fn part_one() -> Score {
    include_str!("./day_02.txt")
        .trim()
        .split("\n")
        .map(|round| {
            let mut round = round.split(" ");
            let opponent = Choice::from(round.next().unwrap());
            let me = Choice::from(round.next().unwrap());

            get_score_part_one(me, opponent)
        })
        .sum()
}

fn get_score_part_one(me: Choice, opponent: Choice) -> Score {
    let shape_score = Score::from(&me);
    let result = match (me, opponent) {
        (x, y) if x == y => GameResult::Draw,
        (x, y) if x < y => GameResult::Lose,
        _ => GameResult::Win,
    };
    let result_score = Score::from(&result);

    shape_score + result_score
}

fn part_two() -> Score {
    include_str!("./day_02.txt")
        .trim()
        .split("\n")
        .map(|round| {
            let mut round = round.split(" ");
            let opponent = Choice::from(round.next().unwrap());
            let result = GameResult::from(round.next().unwrap());

            get_score_part_two(opponent, result)
        })
        .sum()
}

fn get_score_part_two(opponent: Choice, result: GameResult) -> Score {
    let result_score = Score::from(&result);
    let me = match (opponent, result) {
        (Choice::Rock, GameResult::Lose) => Choice::Scissors,
        (Choice::Rock, GameResult::Draw) => Choice::Rock,
        (Choice::Rock, GameResult::Win) => Choice::Paper,
        (Choice::Paper, GameResult::Lose) => Choice::Rock,
        (Choice::Paper, GameResult::Draw) => Choice::Paper,
        (Choice::Paper, GameResult::Win) => Choice::Scissors,
        (Choice::Scissors, GameResult::Lose) => Choice::Paper,
        (Choice::Scissors, GameResult::Draw) => Choice::Scissors,
        (Choice::Scissors, GameResult::Win) => Choice::Rock,
    };
    let choice_score = Score::from(&me);

    result_score + choice_score
}

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Choice::Rock, Choice::Scissors) => Some(Ordering::Greater),
            (Choice::Rock, Choice::Rock) => Some(Ordering::Equal),
            (Choice::Rock, Choice::Paper) => Some(Ordering::Less),
            (Choice::Paper, Choice::Rock) => Some(Ordering::Greater),
            (Choice::Paper, Choice::Paper) => Some(Ordering::Equal),
            (Choice::Paper, Choice::Scissors) => Some(Ordering::Less),
            (Choice::Scissors, Choice::Paper) => Some(Ordering::Greater),
            (Choice::Scissors, Choice::Scissors) => Some(Ordering::Equal),
            (Choice::Scissors, Choice::Rock) => Some(Ordering::Less),
        }
    }
}

impl PartialEq for Choice {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Choice::Rock, Choice::Rock) => true,
            (Choice::Paper, Choice::Paper) => true,
            (Choice::Scissors, Choice::Scissors) => true,
            _ => false,
        }
    }
}

impl From<&str> for Choice {
    fn from(value: &str) -> Choice {
        match value {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => panic!("invalid choice"),
        }
    }
}

type Score = usize;

impl From<&Choice> for Score {
    fn from(choice: &Choice) -> Score {
        match choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

impl From<&str> for GameResult {
    fn from(str: &str) -> GameResult {
        match str {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!("invalid result"),
        }
    }
}

impl From<&GameResult> for Score {
    fn from(result: &GameResult) -> Score {
        match result {
            GameResult::Lose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        }
    }
}
