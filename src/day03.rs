use std::{collections::{HashMap, HashSet}, fs, str::Lines};

pub fn run() -> Result<(),String>{
    let contents = fs::read_to_string("./input/day03.txt")
        .map_err(|err|err.to_string())?;
    run3a(contents)
}

pub fn run3a(contents: String) ->Result<(),String>{    
    let mut lines = contents.lines();
    let sacks = parse_sacks(&mut lines)
        .map_err(|str|str)?; 
    let mut sum: i32= 0;
    for sack in sacks{
        for inter in sack.intersection{
            sum += inter.1;
        }
    }
    println!("sum is {}", sum);
    Ok(())
}

pub struct Sack {
    pub intersection: Vec<(char,i32)>
}

fn parse_sacks(lines: &mut Lines) -> Result<Vec<Sack>, String>{
    let mut sacks: Vec<Sack> = Vec::new();
    for line in lines{
        if line.len() == 0{
            break
        }
        let sack = parse_sack(line.trim())
            .map_err(|x|x)?;
        sacks.push(sack);
    }
    Ok(sacks)
}

fn parse_sack(line: &str)-> Result<Sack, String>{
    let mut sack = Sack{
        intersection: Vec::new(),
    };

    let first = assign_priority(&line[..line.len()/2])
        .map_err(|str|str)?;

    for ch in string_to_hashset(&line[line.len()/2..]){        
        if first.contains_key(&ch){
            let priority = first[&ch];
            sack.intersection.push((ch, priority)); 
        }        
    }
    
    Ok(sack)
}

fn string_to_hashset(str : &str) -> HashSet<char>{    
    let mut hash:HashSet<char> = HashSet::new();
    for ch in str.chars(){
        hash.insert(ch);
    }
    hash
}

fn assign_priority(line: &str) -> Result<HashMap<char,i32>, String>{
    let mut hash:HashMap<char,i32> = HashMap::new();
    for ch in line.chars(){
        let priority:i32 = match ch{
            'a'..='z'=> ch as i32-96,
            'A'..='Z'=> ch as i32-64+26,
            _ =>return Err(format!("invalid char {}", ch))
        };
        hash.insert(ch, priority);
    }
    Ok(hash)
}

#[cfg(test)]
mod tests{
    use super::run3a;

    #[test]
    fn prioritize(){
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        run3a(input.to_string()).unwrap();   
    }
}