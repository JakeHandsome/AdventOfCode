use common::read_input_file_for_project;
fn main() {
    let mut lines = read_input_file_for_project!();
    let mut score = 0u64;
    let mut score2 = 0u64;
    while let Some(Ok(line)) = lines.next() {
        let opponent: RPS = (&line[..1]).into();
        let me: RPS = (&line[2..3]).into();
        let result = play_rps(&me, &opponent);
        score += result.to_score() + me.to_score();

        let desired_result: RPSResult = (&line[2..3]).into();
        let my_move = determine_my_move(&opponent, &desired_result);
        score2 += desired_result.to_score() + my_move.to_score();
    }
    println!("Part1 {}", score);
    println!("Part2 {}", score2);
}

// Converts &str to RPS
impl Into<RPS> for &str {
    fn into(self) -> RPS {
        match self {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => todo!(),
        }
    }
}

// Converts &str to RPSResult
impl Into<RPSResult> for &str {
    fn into(self) -> RPSResult {
        match self {
            "X" => RPSResult::Loss,
            "Y" => RPSResult::Draw,
            "Z" => RPSResult::Win,
            _ => todo!(),
        }
    }
}

impl RPS {
    fn to_score(&self) -> u64 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl RPSResult {
    fn to_score(&self) -> u64 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Draw => 3,
            RPSResult::Loss => 0,
        }
    }
}

// Plays rock papper scissors, returning the result
fn play_rps(me: &RPS, opponent: &RPS) -> RPSResult {
    match (me, opponent) {
        (RPS::Scissors, RPS::Scissors) | (RPS::Paper, RPS::Paper) | (RPS::Rock, RPS::Rock) => RPSResult::Draw,
        (RPS::Scissors, RPS::Paper) | (RPS::Paper, RPS::Rock) | (RPS::Rock, RPS::Scissors) => RPSResult::Win,
        (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) | (RPS::Rock, RPS::Paper) => RPSResult::Loss,
    }
}

// Based on the opponent and desired result picks my move
fn determine_my_move(opponent: &RPS, desired_result: &RPSResult) -> RPS {
    match (opponent, desired_result) {
        (RPS::Rock, RPSResult::Win) | (RPS::Paper, RPSResult::Draw) | (RPS::Scissors, RPSResult::Loss) => RPS::Paper,
        (RPS::Rock, RPSResult::Draw) | (RPS::Paper, RPSResult::Loss) | (RPS::Scissors, RPSResult::Win) => RPS::Rock,
        (RPS::Rock, RPSResult::Loss) | (RPS::Paper, RPSResult::Win) | (RPS::Scissors, RPSResult::Draw) => RPS::Scissors,
    }
}

#[derive(Debug, Clone)]
enum RPSResult {
    Win,
    Draw,
    Loss,
}

#[derive(Debug, Clone, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
