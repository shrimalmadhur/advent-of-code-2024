use regex::Regex;
use std::fs;

pub fn day3() {
    let content: String = fs::read_to_string("./src/inputs/day3.txt").unwrap();

    let sum: i32 = cal_sum(&content);
    println!("{}", sum);

    let do_str: &str = "do()";
    let do_not: &str = "don't()";

    let mut part2_sum: i32 = 0;

    let inter_content: Vec<&str> = content.split(do_not).collect();
    for cont in inter_content.iter() {
        let val: Option<usize> = cont.find(do_str);
        if val.is_some() {
            let idx: usize = val.unwrap();
            let fin_str: String = cont.chars().skip(idx + do_str.len()).collect();
            part2_sum += cal_sum(&fin_str);
        }
    }
    let one_sum: i32 = cal_sum(inter_content[0]);

    println!("{}", part2_sum + one_sum);
}

fn cal_sum(s: &str) -> i32 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum: i32 = 0;
    for cap in pattern.captures_iter(s) {
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();

        sum += num1 * num2;
    }
    sum
}
