use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn snafu_to_int(s: &str) -> i128 {
    //let r:_ = s.chars().rev().collect::<String>();
    //println!("{:?}", s);
    //println!("{:?}", r);
    let mut i = 1;
    let mut number: i128 = 0;
    for c in s.chars().rev() {
        let digit = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _   => panic!("Invalid character"),
        };
        number += digit * i;
        i *= 5;
    }
    number
}

fn int_to_snafu(n: i128) -> String {
    let mut num = n.clone();
    let mut s = String::from("");
    while num != 0 {
        let rem = num % 5;
        let schar = match rem {
            2 => { num /= 5; '2'},
            1 => { num /= 5; '1'},
            0 => { num /= 5; '0'},
            // these include "carrying"
            3 => { num = (num + 5) / 5; '='},
            4 => { num = (num + 5) / 5; '-'},
            _ => panic!("this shouldn't be possible"),
        };
        println!("{} + {} -> {}", schar, s, num);
        s = format!("{}{}", schar, s);
    }
    s
}

fn main() {
    let mut total: i128 = 0;
    if let Ok(lines) = read_lines("./25.input") {
        for line in lines {
            if let Ok(line_str) = line {
                let n = snafu_to_int(line_str.trim());
                total += n;
                println!("{:?} -> {}", line_str, n);
            }
        }
    }
    println!("{total} -> {}", int_to_snafu(total));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
