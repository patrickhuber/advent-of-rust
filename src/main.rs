use std::fs::{self};

struct Elf {
    foods :Vec<Food>
}

struct Food{
    calories :i32
}

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut elves: Vec<Elf> = Vec::new();

    let mut elf = Elf{
        foods : Vec::new()
    };
    for line in contents.lines(){
        if line.len() == 0{
            elves.push(elf);
            elf = Elf{foods: Vec::new()};
            continue;
        }
        let calories= line.parse::<i32>().unwrap();
        elf.foods.push(Food { calories: calories })
    }
    let mut max = 0;
    for elf in elves{
        let mut sum = 0;
        for food in elf.foods{
            sum += food.calories;
        }
        if sum > max {
            max = sum;
        }
    }
    println!("maximum calories {}", max);
    Ok(())
}
