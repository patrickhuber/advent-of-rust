use std::{collections::HashSet, fs, str::Lines};

pub fn run() -> Result<(),String>{
    let contents = fs::read_to_string("./input/day03.txt")
        .map_err(|err|err.to_string())?;
    println!("3a sum {}", run3a(&contents)?);
    println!("3b sum {}", run3b(&contents)?);
    Ok(())
}

pub fn run3a(contents: &String) ->Result<i32,String>{    
    let mut lines = contents.lines();
    let sacks = parse_sacks(&mut lines,2)
        .map_err(|str|str)?; 
    let mut sum: i32= 0;
    for group in create_groups(sacks, 2)? {
        let intersect = intersection(group);
        for item in intersect{
            sum += item.priority;
        }
    }
    Ok(sum)
}

pub fn run3b(contents: &String) -> Result<i32,String>{
    let mut lines = contents.lines();
    let sacks = parse_sacks(&mut lines,1)
        .map_err(|str|str)?; 
    let mut sum: i32= 0;
    for group in create_groups(sacks, 3)? {
        let intersect = intersection(group);
        for item in intersect{
            sum += item.priority;
        }
    }
    Ok(sum)
}

#[derive(Eq, PartialEq, Hash)]
pub struct Item{
    pub ch: char,
    pub priority: i32
}

pub struct Sack {
    pub items: Vec<Item>
}

pub struct Group{
    pub sacks: Vec<Sack>
}

fn intersection(group: Group) -> Vec<Item> {
    let mut set:HashSet<Item> = HashSet::new();
    let mut i: i32 = 0;
    for sack in group.sacks{
        if i == 0{
            for item in sack.items{
                set.insert(item);
            }
        }else{
            let mut temp: HashSet<Item> = HashSet::new();
            for item in sack.items{
                if set.contains(&item){
                    temp.insert(item);
                }
            }
            set = temp;
        }
        i += 1;
    }
    let mut items : Vec<Item> = Vec::new();
    for item in set{
        items.push(item)
    }
    items
}

fn create_groups(sacks: Vec<Sack>, sacks_per_group: i32) -> Result<Vec<Group>, String>{
    let mut groups : Vec<Group> = Vec::new();    
    let mut chunk : Vec<Sack> = Vec::new();
    let mut i: i32 = 0;
    for sack in sacks{
        chunk.push(sack);
        if i != 0 && i % sacks_per_group == sacks_per_group -1 {
            let group = Group{
                sacks: chunk,
            };
            groups.push(group);
            chunk = Vec::new();
        }
        i += 1;
    }   
    Ok(groups)
}

fn parse_sacks(lines: &mut Lines, chunks_per_line: i32) -> Result<Vec<Sack>, String>{
    let mut sacks: Vec<Sack> = Vec::new();
    
    for line in lines{
        let mut items : Vec<Item> = Vec::new();
        // n represents the size of the chunk in characters
        let n = line.len() / chunks_per_line as usize;
        for (i, ch) in line.chars().enumerate(){
            items.push(Item { ch, priority: get_priority(ch)? });            
            if i != 0 && i % n as usize == n -1{
                let sack = Sack{
                    items,
                };
                sacks.push(sack);
                items = Vec::new();
            }
        }
    }
    Ok(sacks)
}

fn get_priority(ch: char) -> Result<i32, String>{
    match ch{
        'a'..='z'=> Ok(ch as i32-96),
        'A'..='Z'=> Ok(ch as i32-64+26),
        _ =>return Err(format!("invalid char {}", ch))
    }
}

#[cfg(test)]
mod tests{
    use super::{run3a,run3b};

    #[test]
    fn prioritize(){
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let sum = run3a(&input.to_string()).unwrap();
        assert_eq!(sum, 157);
    }

    #[test]
    fn groups(){
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let sum = run3b(&input.to_string()).unwrap();
        assert_eq!(sum, 70);
    }
}