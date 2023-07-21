use std::{str::FromStr, cmp::Ordering};

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissor = 3
}

impl FromStr for Move{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissor),
            _ => Err("Not a known move".to_string()),
        }
    }
}   


impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Rock && other == &Move::Scissor {
            Some(Ordering::Greater)
        } else if self == &Move::Scissor && other == &Move::Rock {
            Some(Ordering::Less)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}
pub fn process_part1(input: &str) -> String {
    let result: u32 = input
            .lines()
            .map(|line| {
                let moves: Vec<Move> = line
                    .split(" ")
                    .map(|mv| mv.parse::<Move>().unwrap())
                    .collect();
                
                match moves[0].partial_cmp(&moves[1]) {
                    Some(Ordering::Equal) => 3 + moves[1] as u32,
                    Some(Ordering::Greater) => 0 + moves[1] as u32,
                    Some(Ordering::Less) => 6 + moves[1] as u32,
                    None => {
                        panic!("moves should be comparable")
                    }
                }
        })
        .sum();
        result.to_string()
}


pub fn process_part2(input: &str) -> String {
    let result: u32 = input
            .lines()
            .map(|line| {
                let moves: Vec<&str> = line.split(" ").collect();
                let opponent_move = moves[0].parse::<Move>().unwrap();

                match moves[1] {
                    "X" => {
                        let our_move = match opponent_move {
                            Move::Rock => Move::Scissor,
                            Move::Scissor => Move::Paper,
                            Move::Paper => Move::Rock                    
                        };
                        0 + our_move as u32
                    },
                    "Y" => {
                        3 + opponent_move as u32
                    },
                    "Z" => {
                        let our_move = match opponent_move {
                            Move::Paper => Move::Scissor,
                            Move::Scissor => Move::Rock,
                            Move::Rock => Move::Paper
                        };
                        6 + our_move as u32
                    },
                    _ => panic!("invalid option")
                }
            }).sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part_1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part_2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
