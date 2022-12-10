use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::RegexSet;

struct Computer {
    clock: i32,
    xreg: i32,
    total_signal_strength: i32,
}

impl Computer {
    fn tick(&mut self) {
        self.clock += 1;
        if (self.clock + 20) % 40 == 0 {
            self.add_signal_strength();
            println!("Clock: {}, X: {}: SS: {}", self.clock, self.xreg, self.total_signal_strength);
        }
    }
    fn add_signal_strength(&mut self) {
        self.total_signal_strength += self.clock * self.xreg;
    }
    fn noop(&mut self) {
        self.tick();
    }

    fn addx(&mut self, arg: i32) {
        self.tick();
        self.tick();
        self.xreg += arg;
    }
}

fn main() {
    let mut compy = Computer{clock: 0, xreg: 1, total_signal_strength: 0};

    let set = RegexSet::new(&[
        r"^noop$",
        r"^addx -?\d+$",
    ]).unwrap();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./10.input") {
        for line in lines {
            if let Ok(line_str) = line {
                let matches:Vec<_> = set.matches(&line_str).into_iter().collect();
                println!("{line_str} {:?}", matches);
                match matches[0] {
                    0 => compy.noop(),
                    1 => {
                        let arg = line_str
                            .split(" ")
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>()[1]
                            .parse::<i32>()
                            .unwrap();
                        compy.addx(arg);
                    },
                    _ => panic!("Unexpected input!"),
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
