use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day2() {
    let pattern = Regex::new(r"\s+").unwrap();
    let mut counter: i32 = 0;
    let mut part2_count: i32 = 0;
    if let Ok(lines) = read_lines("./src/inputs/day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let nums: Vec<i32> = pattern
                .split(&line)
                .filter_map(|s| s.parse().ok())
                .collect();

            if (check_increasing(nums.clone()) || check_decreasing(nums.clone()))
                && check_diff(nums.clone())
            {
                counter += 1;
            }

            if check_part2(nums.clone()) {
                part2_count += 1;
            }
        }
    }
    println!("{}", counter);
    println!("{}", part2_count);
}

fn check_part2(vec: Vec<i32>) -> bool {
    let mut result: Vec<Vec<i32>> = Vec::new();

    // For each index, create a new vector without that element
    for i in 0..vec.len() {
        let sub_vec: Vec<i32> = vec
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx != i)
            .map(|(_, &val)| val)
            .collect();

        result.push(sub_vec);
    }

    for res in result {
        if (check_increasing(res.clone()) || check_decreasing(res.clone()))
            && check_diff(res.clone())
        {
            return true;
        }
    }
    return false;
}

fn check_diff(vec: Vec<i32>) -> bool {
    let mut prev: i32 = 0;
    let mut i: i32 = 0;
    for num in vec {
        if i == 0 {
            i = 1;
            prev = num;
            continue;
        }
        let diff = (num - prev).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        prev = num;
    }
    return true;
}

fn check_increasing(vec: Vec<i32>) -> bool {
    let mut prev: i32 = 0;
    for num in vec {
        if num < prev {
            return false;
        }
        prev = num;
    }
    return true;
}

fn check_decreasing(vec: Vec<i32>) -> bool {
    let mut prev: i32 = 0;
    let mut i: i32 = 0;
    for num in vec {
        if i == 0 {
            i = 1;
            prev = num;
            continue;
        }
        if num > prev {
            return false;
        }
        prev = num;
    }
    return true;
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
