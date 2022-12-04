use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./3.input") {
        // Consumes the iterator, returns an (Optional) String

        let mut elf_lines: Vec<String> = Vec::new();
        let mut elves: Vec<HashSet<char>> = Vec::new();
        let mut line_number = 0;
        let mut total = 0;

        for line in lines {
            if let Ok(line_str) = line {
                line_number += 1;
                elf_lines.push(line_str);
                let line_hash: HashSet<char> = HashSet::from_iter(elf_lines.last().expect("eh?").chars());
                elves.push(line_hash);
                if line_number % 3 != 0 {
                    continue;
                }
                
                let mut badges: Vec<char> = elves[0]
                    .intersection(&elves[1])
                    .into_iter()
                    .map(|i| *i)
                    .collect::<Vec<_>>();

                badges = elves[2]
                    .intersection(&HashSet::from_iter(badges))
                    .into_iter()
                    .map(|i| *i)
                    .collect::<Vec<_>>();

                let badge = badges.first().unwrap();
                let score = char_to_score(*badge);
                total += score;
                
                println!("Lines:\n{}\n{}\n{}\nBadge: {}\nScore: {}\nTotal: {}", elf_lines[0], elf_lines[1], elf_lines[2], badge, score, total);
                elf_lines.clear();
                elves.clear();
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

