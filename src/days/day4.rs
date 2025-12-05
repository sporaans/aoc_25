#![allow(dead_code)]

use anyhow::Result;
use ndarray::{Array2, s};

pub fn task1(input: &str) -> Result<i64> {
    let input_vec: Vec<char> = input
        .trim()
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect();

    let size = (input_vec.len()+1).isqrt();
    let mut result: i64 = 0;
    let mut result_matrix: Vec<char> = Vec::new();

    let matrix = Array2::from_shape_vec((size,size), input_vec)?;
    let padded_matrix = pad_matrix(&matrix);
    for window in padded_matrix.windows((3,3)) {
        let mut roll_count = 0;
        if window[[1,1]] != '@' {
            result_matrix.push(window[[1,1]]);
            continue;
        }

        for &element in window.iter() {
            if element == '@' {
                roll_count += 1;
            }
        }
        if roll_count < 5 {
            result_matrix.push('x');
            result += 1;
        } else {
            result_matrix.push(window[[1,1]])
        }
    }
    print_matrix(result_matrix, size);

    Ok(result)
}

pub fn task2(input: &str) -> Result<i64> {
    let input_vec: Vec<char> = input
        .trim()
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect();


    let mut current_state = input_vec;
    let size = (current_state.len()+1).isqrt();
    let mut result: i64 = 0;
    let mut all_removed = false;

    while !all_removed {
        let (next_state, removed) = remove_rolls(current_state, size);
        current_state = next_state;
        if removed == 0 {
            all_removed = true;
        }
        result += removed as i64;
    }

    Ok(result)
}

fn pad_matrix(matrix: &Array2<char>) -> Array2<char> {
    let (rows, cols) = matrix.dim();
    let new_rows = rows + 2;
    let new_cols = cols + 2;
    
    let mut padded = Array2::from_elem((new_rows, new_cols),'.');
    
    // Copy original matrix into center
    padded.slice_mut(s![1..1+rows, 1..1+cols])
        .assign(matrix);
    
    padded
}

fn print_matrix(data: Vec<char>, size: usize) {
    for (i, s) in data.iter().enumerate() {
        if i % size == 0 && i!=0{
            println!();
            print!("{}", s);
        } else {
            print!("{}", s);
        }
    }
    println!();
}

fn remove_rolls(matrix_vec: Vec<char>, size:usize) -> (Vec<char>, i32) {
    let mut result = 0;
    let mut result_matrix: Vec<char> = Vec::new();

    let matrix = Array2::from_shape_vec((size,size), matrix_vec).unwrap();
    let padded_matrix = pad_matrix(&matrix);
    for window in padded_matrix.windows((3,3)) {
        let mut roll_count = 0;
        if window[[1,1]] != '@' {
            result_matrix.push(window[[1,1]]);
            continue;
        }

        for &element in window.iter() {
            if element == '@' {
                roll_count += 1;
            }
        }
        if roll_count < 5 {
            result_matrix.push('x');
            result += 1;
        } else {
            result_matrix.push(window[[1,1]])
        }
    }
    (result_matrix, result)
}