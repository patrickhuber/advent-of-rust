fn day04a(input: &str) -> Result<i32, String>{
    let mut pairs: Vec<Pair> = Vec::new();
    for line in input.lines(){
        let pair = parse_pair(line)?;
        pairs.push(pair);
    }
    let mut total = 0;
    for pair in pairs{
        if fully_contains(pair){
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

fn fully_contains(p: Pair) -> bool{
    p.lhs.lo <= p.rhs.lo && p.rhs.hi <= p.lhs.hi || 
    p.rhs.lo <= p.lhs.lo && p.lhs.hi <= p.rhs.hi 
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
    use super::day04a;

    #[test]
    fn run(){
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let overlap = day04a(input).unwrap();
        assert_eq!(overlap, 2);
    }
}