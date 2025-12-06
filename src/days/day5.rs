#![allow(dead_code)]

use anyhow::{Context,Result};
use std::cmp::{min, max};


pub fn task1(input: &str) -> Result<i64> {
    let input: Vec<&str> = input.trim().lines().collect();
    let input_parts: Vec<&[&str]> = input.split(|x| x.is_empty()).collect();
    let mut result = 0;

    for produce in input_parts[1] {
        let mut fresh = false;
        for range in input_parts[0] {
            let produce_int: i64 = produce.parse()?;
            let (start_str,end_str) = range.split_once("-").context("problem splitting range")?;

            let start_int: i64 = start_str.parse()?;
            let end_int: i64 = end_str.parse()?;

            if produce_int >= start_int && produce_int <= end_int {
                fresh = true;
            }   
        }
        if fresh {result += 1}
    }

    Ok(result)
}

pub fn task2(input: &str) -> Result<i64> {
    let input: Vec<&str> = input.trim().lines().collect();
    let input_parts: Vec<&[&str]> = input.split(|x| x.is_empty()).collect();

    let mut result: i64 = 0;

    let mut ranges: Vec<(i64,i64)> = Vec::new();
    // transform to ints
    for range_str in input_parts[0] {
        let (start_str,end_str) = range_str.split_once("-").context("problem splitting range")?;

        let start_int: i64 = start_str.parse()?;
        let end_int: i64 = end_str.parse()?;

        ranges.push((start_int,end_int))
    }

    // merge ranges
    loop {
        let output = merge_ranges(&ranges);
        if ranges.len() == output.len() {
            break
        }
        ranges = output;
    }

    println!("{:?}",ranges);

    // sum the ints
    for range in ranges {
        let (start_int, end_int ) = range;

        let id_count = end_int-start_int+1; //cuz inclusive ranges
        result += id_count;
    }

    Ok(result)
}

fn range_overlap(first: &(i64, i64), second: &(i64, i64)) -> Option<(i64, i64)> {
    let (a1, a2) = first;
    let (b1, b2) = second;

    if b1 <= a2 && b2 >= a1 {
        let new_start = min(a1, b1);
        let new_end = max(a2, b2);
        Some((*new_start, *new_end))
    } else {
        None
    }
}

fn merge_ranges (ranges: &[(i64,i64)]) -> Vec<(i64,i64)> {
    let mut merged_ranges: Vec<(i64,i64)> = Vec::new();

    for range in ranges {
        let (start_int, end_int) = *range;

        let mut overlap_idx: usize = 0;
        let mut overlap_range: Option<(i64, i64)> = None;

        // find overlap range 
        for (idx,range) in merged_ranges.iter().enumerate() {
            overlap_range = range_overlap(range, &(start_int,end_int));
            if overlap_range != None {
                overlap_idx = idx;
                break;
            }
        }

        // replace existing or add the new range 
        match overlap_range {
            Some(or) => merged_ranges[overlap_idx] = or,
            None => merged_ranges.push((start_int,end_int))
        }
    }

    merged_ranges
}