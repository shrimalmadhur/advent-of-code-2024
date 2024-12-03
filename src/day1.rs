use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day1() {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    let pattern = Regex::new(r"\s+").unwrap();
    if let Ok(lines) = read_lines("./src/inputs/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let nums: Vec<&str> = pattern.split(&line).collect();
            let num0 = nums[0].parse::<i32>().unwrap_or(0);
            let num1 = nums[1].parse::<i32>().unwrap_or(0);
            vec1.push(num0);
            vec2.push(num1);
            let value = map.get(&num1).unwrap_or(&0);
            map.insert(num1, value + 1);
            // println!("{:?}", nums);
        }
    }
    vec1.sort();
    vec2.sort();

    let mut sum: i32 = 0;
    for (i, num) in vec1.iter().enumerate() {
        sum += (num - vec2[i]).abs()
    }
    println!("{}", sum);

    let mut fin: i32 = 0;
    for num in &vec1 {
        let value = map.get(&num).unwrap_or(&0);
        fin += value * num;
    }
    println!("{}", fin)
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
