mod day01;
mod day02;

fn main() -> Result<(),String>{
    println!("--------------- DAY 01 --------------------");
    day01::run()?;
    
    println!("--------------- DAY 02 --------------------");
    day02::run()?;
    Ok(())
}
