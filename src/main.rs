use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./4.input") {
        // Consumes the iterator, returns an (Optional) String

        let mut total = 0;

        for line in lines {
            if let Ok(line_str) = line {
                let ranges: Vec<&str> = line_str.split(",").collect();

                let first_range: Vec<&str> = ranges[0].split("-").collect();
                let second_range: Vec<&str> = ranges[1].split("-").collect();
                println!("{}", line_str);
                println!("{:?}", first_range);
                println!("{:?}", second_range);
                if first_range[0].parse::<u32>().unwrap() >=
                    second_range[0].parse::<u32>().unwrap() &&
                    first_range[1].parse::<u32>().unwrap() <=
                    second_range[1].parse::<u32>().unwrap()
                    ||
                    second_range[0].parse::<u32>().unwrap() >=
                    first_range[0].parse::<u32>().unwrap() &&
                    second_range[1].parse::<u32>().unwrap() <=
                    first_range[1].parse::<u32>().unwrap() {
                        total += 1;
                        println!("One contains the other! {}", total);
                }

            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

