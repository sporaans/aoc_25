#![allow(dead_code)]

use anyhow::{Result, bail};


pub fn task1(input: &str) -> Result<i64> {
    let input: Vec<&str> = input.trim().lines().collect();
    let mut result = 0;
    let operations: Vec<&str> = input[input.len()-1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect(); 

    let mut numbers_mx: Vec<Vec<i64>> = Vec::new();
    for (i,line) in input[..input.len()-1].iter().enumerate() {
        let numbers_vec: Vec<i64> = line
            .split(" ")
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

        if numbers_mx.len() < numbers_vec.len() {
            numbers_mx.resize(numbers_vec.len(), Vec::new());
        }

        for (j, nr) in numbers_vec.iter().enumerate() {
            if numbers_mx[j].len() < input.len()-1{
                numbers_mx[j].resize(input.len()-1,  0)
            }

            numbers_mx[j][i] = *nr;
        }
    }

    for (i, numbers) in numbers_mx.iter().enumerate() {
        let col_result: i64;
        match operations.get(i) {
            Some(&"+") => col_result = numbers.iter().sum(),
            Some(&"*") => col_result = numbers.iter().fold(1, |acc,x | acc*x),
            Some(x) => bail!("invalid operation {}", x),
            None => bail!("operation can not be found for idx {}", i)
        }
        result += col_result
    }

    Ok(result)
}

pub fn task2(_input: &str) -> Result<i64> {
    // println!("{}", input);
    Ok(0)
}
