use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn up(&mut self) {
        self.step(0, 1);
    }
    fn down(&mut self) {
        self.step(0, -1);
    }
    fn left(&mut self) {
        self.step(-1, 0);
    }
    fn right(&mut self) {
        self.step(1, 0);
    }
    fn step(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn diff(&self, other: &Position) -> Position {
        let other = Position {
            x: self.x - other.x,
            y: self.y - other.y,
        };
        return other;
    }
}

fn main() {

    let mut head_position = Position { x: 0, y: 0 }; 
    let mut tail_position = Position { x: 0, y: 0 }; 

    let mut tail_positions: HashSet<Position> = HashSet::new();
    tail_positions.insert(tail_position);

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./9.input") {

        for line in lines {
            if let Ok(line_str) = line {
                let parts: Vec<String> = line_str.split(" ").map(|s| s.to_string()).collect();
                let (direction, steps) = (&parts[0], parts[1].parse::<u32>().unwrap());
                println!("{line_str} -> step {direction} {steps} times");
                for _ in 0..steps {
                    // move the head
                    match direction.as_str() {
                        "U" => head_position.up(),
                        "D" => head_position.down(),
                        "L" => head_position.left(),
                        "R" => head_position.right(),
                        _ => println!("WHHAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAT?"),
                    }
                    println!("New head_position: {}, {}", head_position.x, head_position.y);
                    let diff = head_position.diff(&tail_position);
                    println!("Diff {}, {}", diff.x, diff.y);

                    if diff.x.abs() + diff.y.abs() == 3 {
                        println!("Need to move diagonally");
                        tail_position.step(
                                diff.x / diff.x.abs(),
                                diff.y / diff.y.abs(),
                            )
                    } else if diff.x.abs() == 2  || diff.y.abs() == 2 {
                        // Need to move one step
                        println!("Need to move one step");
                        tail_position.step(diff.x / 2, diff.y / 2)
                    }
                    tail_positions.insert(tail_position);
                    println!("New tail: {}, {}", tail_position.x, tail_position.y);
                }
            }
        }
    }
    println!("Tail positions: {:?}", tail_positions.len());
}



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
