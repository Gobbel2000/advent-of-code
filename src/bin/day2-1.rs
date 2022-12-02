use std::fs;

static INPUT: &str = "input/day2.txt";

const SCORE_LOSE: u32 = 0;
const SCORE_DRAW: u32 = 3;
const SCORE_WIN: u32 = 6;

enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl Moves {
    fn from(code: char) -> Result<Self, &'static str> {
        match code {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => Err("Invalid code"),
        }
    }

    fn from_score(score: u32) -> Result<Self, &'static str> {
        match score {
            1 => Ok(Self::Rock),
            2 => Ok(Self::Paper),
            3 => Ok(Self::Scissors),
            _ => Err("Invalid score"),
        }
    }

    // Score for each move
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    // Score for self when playing against other for lose/draw/win
    fn outcome(&self, other: Self) -> u32 {
        let n_self = self.score() -1;
        let n_other = other.score() -1;
        if n_self == n_other {
            SCORE_DRAW
        } else if (n_other + 1) % 3 == n_self {
            SCORE_WIN
        } else {
            SCORE_LOSE
        }
    }

    fn score_of_round(&self, opponent: Self) -> u32 {
        self.score() + self.outcome(opponent)
    }

    // Return the move necessary to reach the wanted outcome after self
    fn move_for_outcome(&self, outcome: char) -> Result<Self, &'static str> {
        let n_self = self.score() -1;
        let n_other = match outcome {
            // lose
            'X' => if n_self == 0 { 2 } else {n_self - 1},
            // draw
            'Y' => n_self,
            // win
            'Z' => (n_self + 1) % 3,
            _ => return Err("Invalid outcome code"),
        };
        Self::from_score(n_other + 1)
    }
}


fn main() {
    let input: String = fs::read_to_string(INPUT)
        .expect("Couldn't open input file");

    let mut tally: u32 = 0;

    for line in input.lines() {
        let mut chars = line.chars();
        let c_opponent = chars.next().unwrap();
        let c_self = chars.skip(1).next().unwrap();
        let move_opponent = Moves::from(c_opponent).unwrap();
        let move_self = move_opponent.move_for_outcome(c_self).unwrap();

        tally += move_self.score_of_round(move_opponent);
    }
    println!("{}", tally);
}
