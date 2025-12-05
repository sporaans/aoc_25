use std::fs;

mod days;


fn main() -> std::io::Result<()>{
    let contents = fs::read_to_string("inputs/day5")?;
    match days::day5::task1(&contents) {
        Ok(result) => println!("Task 1 result: {}", result),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    match days::day5::task2(&contents) {
        Ok(result) => println!("Task 2 result: {}", result),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}
