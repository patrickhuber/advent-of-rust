use std::fs;

pub fn run() -> Result<(), String>{
    let content = fs::read_to_string("./input/day04.txt")
        .map_err(|op|op.to_string())?;
    let contains = day04a(&content)?;
    println!("{} ranges contain", contains);
    let overlap = day04b(&content)?;
    println!("{} ranges overlap", overlap);
    Ok(())
}

fn day04a(input: &str) -> Result<i32, String>{    
    let pairs = parse_pairs(input)?;
    let mut total = 0;
    for pair in pairs{
        if fully_contains(pair){
            total += 1;
        }
    }
    Ok(total)
}

fn day04b(input: &str) -> Result<i32, String>{
    let pairs = parse_pairs(input)?;
    let mut total = 0;
    for pair in pairs{
        if overlap(pair){
            total += 1;
        }
    }
    Ok(total)
}

pub struct Range {
    pub hi: i32,
    pub lo: i32
}

pub struct Pair {
    pub lhs: Range,
    pub rhs: Range
}

fn overlap(p: Pair) -> bool{
    range_overlap(&p.lhs, &p.rhs) ||
    range_overlap(&p.rhs, &p.lhs) ||
    fully_contains(p)
}

fn range_overlap(r1: &Range, r2: &Range) -> bool{
    r1.lo <= r2.lo && r2.lo <= r1.hi
}

fn fully_contains(p: Pair) -> bool{
    range_contains(&p.lhs, &p.rhs) ||
    range_contains(&p.rhs, &p.lhs) 
}

fn range_contains(r1: &Range, r2: &Range) -> bool{
    r1.lo <= r2.lo && r2.hi <= r1.hi
}

fn parse_pairs(input: &str) -> Result<Vec<Pair>, String>{
    let mut pairs: Vec<Pair> = Vec::new();
    for line in input.lines(){
        if line.len() == 0{
            continue;
        }
        let pair = parse_pair(line)?;
        pairs.push(pair);
    }
    Ok(pairs)
}

fn parse_pair(s: &str) -> Result<Pair, String>{
    let splits : Vec<&str> = s.split(',').collect();
    if splits.len() != 2{
        return Err(format!("parse_pair expected 2 splits but found {}", splits.len()));
    }
    Ok(Pair { 
        lhs: parse_range(splits[0])?, 
        rhs: parse_range(splits[1])?,
    })
}

fn parse_range(s: &str) -> Result<Range, String>{
    let splits: Vec<&str> = s.split('-').collect();
    if splits.len() != 2{
        return Err(format!("parse_range expected 2 splits but found {}", splits.len()));
    }

    Ok(Range{
        lo: splits[0].parse::<i32>()
            .map_err(|op|op.to_string())?,        
        hi: splits[1].parse::<i32>()
            .map_err(|op|op.to_string())?,
    })
}

#[cfg(test)]
mod tests{
    use super::{day04b,day04a};

    #[test]
    fn contained(){
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let overlap = day04a(input).unwrap();
        assert_eq!(overlap, 2);
    }

    #[test]
    fn overlaps(){
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let overlap = day04b(input).unwrap();
        assert_eq!(overlap, 4);
    }
}