use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./2.input") {
        // Consumes the iterator, returns an (Optional) String

        let mut total = 0;

        for line in lines {
            if let Ok(line_str) = line {
                let plays: Vec<&str> = line_str.split(" ").collect();
                let opp_play = match plays[0] {
                    "A" => "R",
                    "B" => "P",
                    "C" => "S",
                    _   => "XXX"
                };
                let my_play = match plays[1] {
                    "X" => "R",
                    "Y" => "P",
                    "Z" => "S",
                    _   => "XXX"
                };
                let bonus = match my_play {
                    "R" => 1,
                    "P" => 2,
                    "S" => 3,
                    _   => 9999
                };
                let outcome: u16;
                if my_play == opp_play {
                    outcome = 3;
                } else {
                    outcome = match (opp_play, my_play) {
                        ("R", "P") => 6,
                        ("R", "S") => 0,
                        ("P", "R") => 0,
                        ("P", "S") => 6,
                        ("S", "R") => 6,
                        ("S", "P") => 0,
                        _ => 8888
                    };
                }
                let score = bonus + outcome;
                total += score;
                println!("{} : {}-{} ({} + {} = {}) -> {}", line_str, opp_play, my_play, bonus, outcome, score, total);
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

