use anyhow::{Context, Result};

pub fn task1(input: &str) -> Result<i64> {
    let banks: Vec<&str> = input.trim().split('\n').collect();
    // let banks = vec!["818181911112111"];
    let mut result: i64 = 0;

    for bank in banks {
        let batteries: Vec<i32> = bank
            .chars()
            .map(|b| b.to_digit(10).expect("not a digit") as i32)
            .collect();
        let largest: &i32 = batteries[..batteries.len()-1]
            .iter().max().context("problem getting largest")?; 
        let largest_idx = batteries[..batteries
            .len()-1]
            .iter()
            .position(|x| x == largest)
            .context("problem getting largest_ix")?;

        let largest_after = batteries[largest_idx+1..]
            .iter().max().context("problem getting largest_after")?;

        let constructed_int:i64 = format!("{}{}",largest,largest_after)
            .parse().context("problem parsing final result")?;

        // println!("largest joltage found: {}",constructed_int);
        result += constructed_int;
    }

    Ok(result)
}

pub fn task2(input: &str) -> Result<i64> {
    let banks: Vec<&str> = input.trim().split('\n').collect();
    // let banks = vec!["987654321111111"];
    let mut result: i64 = 0;

    for bank in banks {
        let mut constructed: i64 = 0;
        let mut pos: usize = 0;

        let batteries: Vec<i64> = bank
            .chars()
            .map(|b| b.to_digit(10).expect("not a digit") as i64)
            .collect();

        for i in 0..12 {
            let slice = &batteries[pos..batteries.len()-11+i];
            let (largest,largest_idx) = find_largest(&slice)?;

            pos = pos + largest_idx + 1;
            constructed = constructed * 10 + largest;
        }
        // println!("constructed result: {}", constructed);
        result += constructed;
    }

    Ok(result)
}


fn find_largest (batteries: &[i64]) -> Result<(i64, usize)> {
    let largest: &i64 = batteries.iter().max().context("problem getting largest")?; 
    let largest_idx = batteries
        .iter()
        .position(|x| x == largest)
        .context("problem getting largest_ix")?;

    Ok((*largest, largest_idx))
}