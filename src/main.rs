use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./1.input") {
        // Consumes the iterator, returns an (Optional) String
        let mut max = 0;
        let mut total = 0;
        for line in lines {
            if let Ok(line_str) = line {
                if line_str == "" {
                    println!("Group Total: {}, Max: {}", total, max);
                    if total > max {
                        max = total;
                    }
                    total = 0;
                } else {
                    let number = line_str.parse::<i32>().unwrap();
                    total += number;
                    println!("{} -> {}", line_str, total);
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

