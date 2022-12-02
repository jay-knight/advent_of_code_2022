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
                let outcome = match plays[1] {
                    "X" => 0,
                    "Y" => 3,
                    "Z" => 6,
                    _   => 9999
                };
                let my_play: &str;
                if outcome == 3 {
                    my_play = opp_play;
                } else {
                    my_play = match (opp_play, outcome) {
                        ("R", 0) => "S",
                        ("R", 6) => "P",
                        ("P", 0) => "R",
                        ("P", 6) => "S",
                        ("S", 0) => "P",
                        ("S", 6) => "R",
                        _ => "XXXX"
                    };
                }
                let bonus = match my_play {
                    "R" => 1,
                    "P" => 2,
                    "S" => 3,
                    _   => 9999
                };
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

