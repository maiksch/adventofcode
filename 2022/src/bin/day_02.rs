fn main() {
    let part_one = part_one();
    println!("{}", part_one);

    let part_two = part_two();
    println!("{}", part_two);
}

fn part_one() -> Score {
    include_str!("./day_02.txt")
        .lines()
        .map(|round| {
            let round = round.split(" ").collect::<Vec<&str>>();
            let opponent = Choice::from(round[0]);
            let me = Choice::from(round[1]);
            get_score_part_one(me, opponent)
        })
        .sum()
}

fn get_score_part_one(me: Choice, opponent: Choice) -> Score {
    let shape_score = Score::from(&me);
    let result = match (me, opponent) {
        (Choice::Rock, Choice::Scissors) => Outcome::Win,
        (Choice::Rock, Choice::Rock) => Outcome::Draw,
        (Choice::Rock, Choice::Paper) => Outcome::Lose,
        (Choice::Paper, Choice::Rock) => Outcome::Win,
        (Choice::Paper, Choice::Paper) => Outcome::Draw,
        (Choice::Paper, Choice::Scissors) => Outcome::Lose,
        (Choice::Scissors, Choice::Paper) => Outcome::Win,
        (Choice::Scissors, Choice::Scissors) => Outcome::Draw,
        (Choice::Scissors, Choice::Rock) => Outcome::Lose,
    };
    let result_score = Score::from(&result);

    shape_score + result_score
}

fn part_two() -> Score {
    include_str!("./day_02.txt")
        .lines()
        .map(|round| {
            let round = round.split(" ").collect::<Vec<&str>>();
            let opponent = Choice::from(round[0]);
            let result = Outcome::from(round[1]);
            get_score_part_two(opponent, result)
        })
        .sum()
}

fn get_score_part_two(opponent: Choice, result: Outcome) -> Score {
    let result_score = Score::from(&result);
    let me = match (opponent, result) {
        (Choice::Rock, Outcome::Lose) => Choice::Scissors,
        (Choice::Rock, Outcome::Draw) => Choice::Rock,
        (Choice::Rock, Outcome::Win) => Choice::Paper,
        (Choice::Paper, Outcome::Lose) => Choice::Rock,
        (Choice::Paper, Outcome::Draw) => Choice::Paper,
        (Choice::Paper, Outcome::Win) => Choice::Scissors,
        (Choice::Scissors, Outcome::Lose) => Choice::Paper,
        (Choice::Scissors, Outcome::Draw) => Choice::Scissors,
        (Choice::Scissors, Outcome::Win) => Choice::Rock,
    };
    let choice_score = Score::from(&me);

    result_score + choice_score
}

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Choice {
    fn from(value: &str) -> Choice {
        match value {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => unimplemented!("invalid choice"),
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

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Outcome {
        match value {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unimplemented!("invalid outcome"),
        }
    }
}

impl From<&Outcome> for Score {
    fn from(result: &Outcome) -> Score {
        match result {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}
