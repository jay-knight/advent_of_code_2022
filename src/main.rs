use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./3.input") {
        // Consumes the iterator, returns an (Optional) String

        let mut total = 0;

        let alpha_offset = ('A' as u32) - 1;

        println!("{}", ('a' as u32) - alpha_offset);

        for line in lines {
            if let Ok(line_str) = line {
                let length = line_str.chars().count();
                let (first, second) = line_str.split_at(length / 2);
                let first_set: HashSet<char> = HashSet::from_iter(first.chars());
                let second_set: HashSet<char> = HashSet::from_iter(second.chars());
                let mut common = ' ';
                for item in first_set.intersection(&second_set) {
                    common = *item;
                }
                let score = char_to_score(common);
                total += score;
                println!("{} {}\n{}\n{}\nCommon: {}, Score: {}, Total: {}", length, line_str, first, second, common, score, total);
            }
        }
    }
}

fn char_to_score(character: char) -> u32 {
    let value = character as u32;
    if value >= 'a' as u32 && value <= 'z' as u32 {
        return value - 'a' as u32 + 1;
    } else {
        return value - 'A' as u32 + 27;
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

