#![allow(dead_code)]

use anyhow::{Context,Result};
use std::collections::HashSet;
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

pub fn task2(input: &str) -> Result<usize> {
    let input: Vec<&str> = input.trim().lines().collect();
    let input_parts: Vec<&[&str]> = input.split(|x| x.is_empty()).collect();

    let mut fresh_ids: HashSet<i64> = HashSet::new();

    let mut merged_ranges: Vec<(i64,i64)> = Vec::new();

    for range in input_parts[0] {
        let (start_str,end_str) = range.split_once("-").context("problem splitting range")?;

        let start_int: i64 = start_str.parse()?;
        let end_int: i64 = end_str.parse()?;

        let mut overlap_idx: usize = 0;
        let mut overlap_range: (i64,i64) = (0,0);

        // find overlap range 
        for (idx,range) in merged_ranges.iter().enumerate() {
            overlap_range = range_overlap(range, &(start_int,end_int));
            if overlap_range != (0,0) {
                overlap_idx = idx;
                break;
            }
        }

        // reaplace existing or add the new range 
        if overlap_range != (0,0) {
            merged_ranges[overlap_idx] = overlap_range
        } else {
            merged_ranges.push((start_int,end_int))
        }
    }

    println!("{:?}",merged_ranges);

    for range in merged_ranges {
        let (start_int, end_int ) = range;

        for id in start_int..=end_int {
            fresh_ids.insert(id);
        }
    }

    let result = fresh_ids.len();

    Ok(0)
}

fn range_overlap(first: &(i64, i64), second: &(i64, i64)) -> (i64, i64) {
    let (a1, a2) = first;
    let (b1, b2) = second;

    if b1 <= a2 && b2 >= a1 {
        let new_start = min(a1, b1);
        let new_end = max(a2, b2);
        (*new_start, *new_end)
    } else {
        (0, 0)
    }
}