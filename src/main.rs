use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./5.input") {
        // Consumes the iterator, returns an (Optional) String

        // Hard-code initial configuration
        let mut stacks: Vec<Vec<char>> = Vec::new();
        stacks.push("NCRTMZP".chars().collect());
        stacks.push("DNTSBZ".chars().collect());
        stacks.push("MGQRFCTG".chars().collect());
        stacks.push("GRZ".chars().collect());
        stacks.push("ZNRH".chars().collect());
        stacks.push("FHSWPZLD".chars().collect());
        stacks.push("WDZRCGM".chars().collect());
        stacks.push("SJFLHWZQ".chars().collect());
        stacks.push("SQPWN".chars().collect());
        println!("Initial:");
        print_stacks(&stacks);

        let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();

        for line in lines {
            if let Ok(line_str) = line {
                println!("{}", line_str);
                let move_cap = move_re.captures(&line_str).unwrap();
                let count = move_cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let source = move_cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let dest = move_cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
                //println!("{} {} {}", count, source, dest);

                for _ in 1..=count {
                    let moving = stacks[source - 1].pop().unwrap();
                    stacks[dest - 1].push(moving);
                }
                print_stacks(&stacks);
            }
        }
    }
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for (i, stack) in stacks.iter().enumerate() {
        print!("{}: ", i+1);
        for character in stack {
            print!("{} ", character);
        }
        println!("");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

