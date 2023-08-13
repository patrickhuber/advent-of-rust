use std::fs;

pub fn run() -> Result<(), String>{
    let contents = fs::read_to_string("input/day02.txt")
        .expect("Should have been able to read the file");
    let mut lines = contents.lines();
    let rounds = match parse_rounds(&mut lines){
        Ok(rounds)=>rounds,
        Err(msg)=>return Err(msg),
    };
    match score_choice(&rounds){
        Ok(sum) => println!("total score is {}", sum),
        Err(msg) => return Err(msg),
    };
    match score_outcome(&rounds){
        Ok(sum) =>  println!("fixed score id {}", sum),
        Err(msg) => return Err(msg),
    }
    Ok(())
}

pub fn score_choice(rounds: &[Round]) -> Result<i32,String>{
    let mut sum = 0;
    for round in rounds{
        sum += round.player.score() + round.outcome().score()
    }
    Ok(sum)
}

pub fn score_outcome(rounds: &[Round]) -> Result<i32,String>{
    let mut sum = 0;
    for round in rounds{
        let oponent = round.oponent;
        let player = round.outcome.player_choice(oponent);
        sum += player.score() + round.outcome.score();
    }
    Ok(sum)
}

pub struct Round{
    pub player: Choice,
    pub oponent: Choice,
    pub outcome: Outcome
}

#[derive(Eq,PartialEq,Clone,Copy)]
pub enum Choice{
    Rock,
    Paper,
    Scissors
}

#[derive(Eq,PartialEq,Clone,Copy)]
pub enum Outcome{
    Win,
    Lose,
    Tie
}

fn parse_rounds(lines : &mut core::str::Lines) -> Result<Vec<Round>, String>{
    let mut rounds: Vec<Round> = Vec::new();
    for line in lines{
        if line.len() == 0{
            continue;
        }
        let round = match parse_round(line){
            Ok(round)=> round,
            Err(msg)=>return Err(msg),
        };
        rounds.push(round);
    }
    Ok(rounds)
}

fn parse_round(line: &str) -> Result<Round, String> {
    let segments: Vec<&str> = line.split(' ').collect();
    if segments.len() != 2{
        return Err(String::from("invalid segment count"));
    }
    let round = Round{
        player: match segments[1]{
            "X"=>Choice::Rock,
            "Y"=>Choice::Paper,
            "Z"=>Choice::Scissors,
             _ => return Err(String::from("invalid player choice")),
        },
        oponent: match segments[0]{
            "A"=>Choice::Rock,
            "B"=>Choice::Paper,
            "C"=>Choice::Scissors,            
            _ => return Err(String::from("invalid oponent choice")),
        },
        outcome: match segments[1]{
            "X"=>Outcome::Lose,
            "Y"=>Outcome::Tie,
            "Z"=>Outcome::Win,
            _ => return Err(String::from("invalid outcome")),
        }
    };
    Ok(round)
}

impl Round{
    fn outcome(&self)->Outcome{
        if &self.oponent == &self.player{
            return Outcome::Tie;
        }
        match (&self.player.clone(),&self.oponent.clone()){
            (Choice::Paper, Choice::Rock) |
            (Choice::Rock, Choice::Scissors) |
            (Choice::Scissors, Choice::Paper)=>Outcome::Win,
            _=> Outcome::Lose
        }
    }
}

impl Choice{
    fn score(&self)->i32{
        match &self{
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl Outcome{
    fn score(&self)->i32{
        match &self{
            Outcome::Win => 6,
            Outcome::Tie => 3,
            Outcome::Lose => 0,
        }
    }
    fn player_choice(&self, oponent: Choice)->Choice{
        match &self{
            Outcome::Win => match oponent{
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
            Outcome::Lose => match oponent{
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            Outcome::Tie => oponent,
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::day02::{score_choice, score_outcome};
    use super::parse_rounds;

    #[test]
    fn test_choice(){
        let input = "A Y\nB X\nC Z";
        let rounds = parse_rounds(&mut input.lines()).unwrap();
        let sum = score_choice(rounds.as_slice()).unwrap();
        assert_eq!(15, sum);
    }

    #[test]
    fn test_outcome(){
        let input = "A Y\nB X\nC Z";
        let rounds = parse_rounds(&mut input.lines()).unwrap();
        let sum = score_outcome(rounds.as_slice()).unwrap();
        assert_eq!(12, sum);
    }
}