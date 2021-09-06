use std::io::stdin;

use rand::prelude::*;

const HANDS: [Hand; 3] = [ Hand::Rock, Hand::Paper, Hand::Scissors ];

// -----------------------------------------------------------------------------
//     - Outcome -
// -----------------------------------------------------------------------------
enum Outcome {
    Win,
    Lose,
    Tie,
}

// -----------------------------------------------------------------------------
//     - Hand -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

impl Hand {
    fn from_choice(choice: u32) -> Self {
        match choice {
            1 => Hand::Rock,
            2 => Hand::Paper,
            3 => Hand::Scissors,
            _ => unreachable!(),
        }
    }
}

fn prompt() {
    eprintln!("");
    eprintln!("1) Rock");
    eprintln!("2) Paper");
    eprintln!("3) Scissors");

}

// -----------------------------------------------------------------------------
//     - Random hand selection -
// -----------------------------------------------------------------------------
fn random_hand() -> Hand {
    let mut rng = thread_rng();
    *HANDS.choose(&mut rng).unwrap()
}

// -----------------------------------------------------------------------------
//     - Main -
// -----------------------------------------------------------------------------
fn main() {
    let mut stdin = stdin();
    let mut buf = String::new();
    let mut score = 0;

    prompt();
    while let Ok(_) = stdin.read_line(&mut buf) {

        // User hand
        let hand = buf
            .trim()
            .parse::<u32>()
            .ok()
            .filter(|choice| *choice >= 1 || *choice >= 3)
            .map(Hand::from_choice)
            .unwrap_or(Hand::Invalid);

        // Clear the buffer from the previous choice
        buf.clear();

        if let Hand::Invalid = hand {
            eprintln!("invalid hand");
            continue;
        }

        // Opponents hand, randomly selected
        let opponent = random_hand();
        eprintln!("Opponent: {:?}", opponent);

        let outcome = match (hand, opponent) {
            // Lose
            (Hand::Rock, Hand::Paper) => Outcome::Lose,
            (Hand::Paper, Hand::Scissors) => Outcome::Lose,
            (Hand::Scissors, Hand::Rock) => Outcome::Lose,

            // Win
            (Hand::Rock, Hand::Scissors) => Outcome::Win,
            (Hand::Paper, Hand::Rock) => Outcome::Win,
            (Hand::Scissors, Hand::Paper) => Outcome::Win,

            // Tie
            (Hand::Rock, Hand::Rock) 
            | (Hand::Paper, Hand::Paper) 
            | (Hand::Scissors, Hand::Scissors) => Outcome::Tie,

            // Unreachable (for now, but if you change this it might not be, but there you go...)
            _ => unreachable!()
        };

        match outcome {
            Outcome::Lose => eprintln!("You lose"),
            Outcome::Tie => eprintln!("It's a tie (wear it for Christmas"),
            Outcome::Win =>  {
                eprintln!("You win!");
                score += 1;
            }
        }

        eprintln!("Current score: {}", score);
        prompt();
    }
}
