use std::fs;


fn main() -> std::io::Result<()>{
    let contents = fs::read_to_string("inputs/day1")?;
    match day1_task1(&contents) {
        Ok(zeros) => println!("Task 1 zeros: {}", zeros),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    match day1_task2(&contents) {
        Ok(zeros) => println!("Task 2 zeros: {}", zeros),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}

fn day1_task1(input: &str) -> Result<i32, Box<dyn std::error::Error>>{
    let rotations: Vec<&str> = input.trim().split('\n').collect();
    let mut position = 50;
    let mut zeros = 0;

    for (_,rotation) in rotations.iter().enumerate() {
        let clicks_raw: i32 = rotation[1..].parse()?;
        let clicks = clicks_raw % 100;
        let direction = rotation.chars().next();

        match direction {
            Some('R') => position = position + clicks,
            Some('L') => position = position - clicks,
            _ => return Err(format!("unknown direction: {}", direction.unwrap_or('\0')).into()),
        }

        if position >= 100 {
            position = position - 100;
        }

        if position < 0 {
            position = 100 - position.abs();
        }

        if position == 0 {
            zeros = zeros + 1;
        }

        // if i < 50 {
        //     println!("rotation: {}, clicks: {}, position: {}", rotation, clicks_raw, position);
        // }
    }

    Ok(zeros)
}

fn day1_task2(input: &str) -> Result<i32, Box<dyn std::error::Error>>{
    let rotations: Vec<&str> = input.trim().split('\n').collect();
    // let rotations: Vec<&str> = vec!["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"];
    // let rotations: Vec<&str> = vec!["R100"];
    let mut position = 50;
    let mut zeros = 0;

    for (i,rotation) in rotations.iter().enumerate() {
        if i < 100 {
            print!("start: {} ", position);
        }
        let start_position = position;

        let clicks: i32 = rotation[1..].parse()?;
        let direction = rotation.chars().next();

        match direction {
            Some('R') => position = position + clicks,
            Some('L') => position = position - clicks,
            _ => return Err(format!("unknown direction: {}", direction.unwrap_or('\0')).into()),
        }
        zeros = zeros + (position / 100).abs();

        // not even a mother can love this
        if position < 0 {
            position = position % 100;
            if position != 0 {
                if start_position != 0 {
                    zeros = zeros + 1;
                }
                position = 100 - position.abs();
            } else {
                if start_position != 0 || clicks == 0 {
                    zeros = zeros + 1;
                }
            }
        } else {
            if position == 0 {
                zeros = zeros + 1;
            }
            position = position % 100;
        }

        if i < 100 {
            println!("rotation: {}, position: {}, zeros: {}", rotation, position, zeros);
        }
    }

    Ok(zeros)
}