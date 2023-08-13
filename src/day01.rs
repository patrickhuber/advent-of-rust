use std::fs;

pub fn run() -> Result<(),String>{
    let contents = fs::read_to_string("input/day01.txt")
        .expect("Should have been able to read the file");
  
    let mut lines = contents.lines();
    let elves = parse_elves(&mut lines);
    let mut max = 0;
    let mut max3: [i32; 3] = [0; 3];

    for elf in elves{
        let mut sum = 0;
        for food in elf.foods{
            sum += food.calories;
        }
        if sum > max {
            max = sum;
        }
        // insertion sort
        for x in 0..max3.len(){
            if sum < max3[x]{
                continue;
            }else if sum > max3[x] {                
                let temp = max3[x];
                max3[x]= sum;
                sum = temp;
            }
        }
    }
    println!("maximum calories {}", max);
    println!("maximum 3 calories {:?}", max3);
    println!("maximum total {}", max3.iter().sum::<i32>());
    Ok(())
}


pub struct Elf {
    pub foods :Vec<Food>
}

pub struct Food{
    pub calories :i32
}

pub fn parse_elves(lines: &mut core::str::Lines) -> Vec<Elf>{    
    let mut elves: Vec<Elf> = Vec::new();
    loop{
        match parse_elf(lines){
            None => break,
            Some(elf)=>{
                elves.push(elf);
            }
        }
    }
    elves
}

fn parse_elf(lines: &mut core::str::Lines) -> Option<Elf>{    
    let mut elf = Elf{
        foods: Vec::new(),
    };
    loop{
        match lines.next(){
            None => return None,
            Some(line) => {
                if line.len() == 0{
                    return Some(elf)
                }
                let calories = match line.parse::<i32>(){
                    Err(_) => 0,
                    Ok(i) =>i
                };
                elf.foods.push(Food { calories: calories })
            }
        }
    }
}