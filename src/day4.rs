use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day4() {
    let mut matrix: Vec<Vec<char>> = vec![];
    if let Ok(lines) = read_lines("./src/inputs/day4.txt") {
        for line in lines.flatten() {
            let ch: Vec<char> = line.chars().collect();
            matrix.push(ch);
        }
    }

    let mut ctr: i32 = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'X' {
                ctr += check(matrix.clone(), i, j)
            }
        }
    }

    println!("{}", ctr)
}

fn check_char(c1: char, c2: char, c3: char) -> bool {
    return c1 == 'M' && c2 == 'A' && c3 == 'S';
}

fn check(m: Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut ctr: i32 = 0;
    // check forwards
    if col + 3 < m[row].len() && check_char(m[row][col + 1], m[row][col + 2], m[row][col + 3]) {
        ctr += 1
    }

    // check backwards
    if col as i32 - 3 >= 0 && check_char(m[row][col - 1], m[row][col - 2], m[row][col - 3]) {
        ctr += 1
    }

    // check up
    if row as i32 - 3 >= 0 && check_char(m[row - 1][col], m[row - 2][col], m[row - 3][col]) {
        ctr += 1
    }

    // check down
    if row + 3 < m.len() && check_char(m[row + 1][col], m[row + 2][col], m[row + 3][col]) {
        ctr += 1
    }

    // backwards up
    if col as i32 - 3 >= 0
        && row as i32 - 3 >= 0
        && check_char(
            m[row - 1][col - 1],
            m[row - 2][col - 2],
            m[row - 3][col - 3],
        )
    {
        ctr += 1
    }

    // forwards up
    if col + 3 < m[row].len()
        && row as i32 - 3 >= 0
        && check_char(
            m[row - 1][col + 1],
            m[row - 2][col + 2],
            m[row - 3][col + 3],
        )
    {
        ctr += 1
    }

    // backwards down
    if col as i32 - 3 >= 0
        && row + 3 < m.len()
        && check_char(
            m[row + 1][col - 1],
            m[row + 2][col - 2],
            m[row + 3][col - 3],
        )
    {
        ctr += 1
    }

    // forwards down
    if col + 3 < m[row].len()
        && row + 3 < m.len()
        && check_char(
            m[row + 1][col + 1],
            m[row + 2][col + 2],
            m[row + 3][col + 3],
        )
    {
        ctr += 1
    }
    return ctr;
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
