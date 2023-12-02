enum ActionKind {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose, // X
    Draw, // Y
    Win,  // Z
}

impl Into<ActionKind> for char {
    fn into(self) -> ActionKind {
        match self {
            'A' => ActionKind::Rock,
            'B' => ActionKind::Paper,
            'C' => ActionKind::Scissors,
            'X' => ActionKind::Rock,
            'Y' => ActionKind::Paper,
            'Z' => ActionKind::Scissors,
            _ => unreachable!("Unknown char"),
        }
    }
}

impl Into<Outcome> for char {
    fn into(self) -> Outcome {
        match self {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!("unknown char for outcome"),
        }
    }
}

struct Game {
    elf_action: ActionKind,
    your_action: ActionKind,
}

/*
The winner of the whole tournament is the player with the highest score.
Your total score is the sum of your scores for each round.
The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
 */

impl Game {
    fn score(&self) -> i32 {
        match (&self.elf_action, &self.your_action) {
            (ActionKind::Rock, ActionKind::Rock) => 1 + 3,
            (ActionKind::Rock, ActionKind::Paper) => 2 + 6,
            (ActionKind::Rock, ActionKind::Scissors) => 3 + 0,
            (ActionKind::Paper, ActionKind::Rock) => 1 + 0,
            (ActionKind::Paper, ActionKind::Paper) => 2 + 3,
            (ActionKind::Paper, ActionKind::Scissors) => 3 + 6,
            (ActionKind::Scissors, ActionKind::Rock) => 1 + 6,
            (ActionKind::Scissors, ActionKind::Paper) => 2 + 0,
            (ActionKind::Scissors, ActionKind::Scissors) => 3 + 3,
        }
    }
}

struct Game2 {
    elf_action: ActionKind,
    outcome: Outcome,
}

impl Game2 {
    fn score(&self) -> i32 {
        match (&self.elf_action, &self.outcome) {
            (ActionKind::Rock, Outcome::Lose) => 0 + 3,
            (ActionKind::Rock, Outcome::Draw) => 3 + 1,
            (ActionKind::Rock, Outcome::Win) => 6 + 2,
            (ActionKind::Paper, Outcome::Lose) => 0 + 1,
            (ActionKind::Paper, Outcome::Draw) => 3 + 2,
            (ActionKind::Paper, Outcome::Win) => 6 + 3,
            (ActionKind::Scissors, Outcome::Lose) => 0 + 2,
            (ActionKind::Scissors, Outcome::Draw) => 3 + 3,
            (ActionKind::Scissors, Outcome::Win) => 6 + 1,
        }
    }
}

fn main() {
    let lines = include_str!("./input.txt").lines();
    let mut total = 0;
    let mut total_2 = 0;
    for line in lines.into_iter() {
        let chars: Vec<char> = line.chars().collect();
        let game = Game {
            elf_action: chars[0].into(),
            your_action: chars[2].into(),
        };
        total += game.score();
        let game2 = Game2 {
            elf_action: chars[0].into(),
            outcome: chars[2].into(),
        };
        total_2 += game2.score();
    }
    println!("Total: {}", total);
    println!("Total 2: {}", total_2);
}
