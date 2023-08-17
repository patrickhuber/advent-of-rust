mod day01;
mod day02;
mod day03;
mod day04;

fn main() -> Result<(),String>{
    print_header(1);
    day01::run()?;
    
    print_header(2);
    day02::run()?;

    print_header(3);
    day03::run()?;

    print_header(4);
    day04::run()?;
    Ok(())
}

fn print_header(day: i32){
    print!("--------------- DAY ");
    if day < 10{
        print!("0");
    }
    println!("{} --------------------", day.to_string());
}
