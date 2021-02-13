use std::{collections::HashMap, convert::{TryFrom}, io::Write};

pub struct RockPaperSciccors {
    win_table: HashMap<Move, Move>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Sciccors,
}

impl TryFrom<&str> for Move {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Move, Self::Error> {
        match value {
            "Rock" => Ok(Move::Rock),
            "Paper" => Ok(Move::Paper),
            "Sciccors" => Ok(Move::Sciccors),
            _ => Err("Invalid move"),
        }
    }
}

impl RockPaperSciccors {
    pub fn new() -> Self {
        Self {
            win_table: [(Move::Rock, Move::Sciccors), (Move::Sciccors, Move::Paper), (Move::Paper, Move::Rock)].iter().cloned().collect()
        }
    }
    pub fn run(&self) -> ! {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();

        let mut lhs = String::new();
        let mut rhs = String::new();

        loop {    
            print!("Enter your move, first player: ");
            stdout.flush().expect("Failed to flush");
            stdin.read_line(&mut lhs).expect("Failed to read a line");
            let lhs_move = Move::try_from(lhs.trim()).expect("Invalid move");

            print!("Enter your move, second player: ");
            stdout.flush().expect("Failed to flush");
            stdin.read_line(&mut rhs).expect("Failed to read a line");
            let rhs_move = Move::try_from(rhs.trim()).expect("Invalid move");

            if lhs_move == rhs_move {
                println!("Draw")
            } else if self.win_table[&lhs_move] == rhs_move {
                println!("First player won")
            } else {
                println!("Second player won")
            }

            lhs.clear();
            rhs.clear();
        }
    }
}