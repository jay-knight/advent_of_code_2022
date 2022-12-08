use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut grid: Vec<Vec<u32>> = Vec::new();
    let mut total = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./8.input") {

        for line in lines {
            if let Ok(line_str) = line {
                grid.push(line_str
                    .chars()
                    .flat_map(|c| c.to_digit(10))
                    .collect());
            }
        }
        for column in 0..99 {
            for row in 0..99 {
                let can_see = can_see(&grid, column, row);
                if can_see {
                    total += 1;
                }
                println!("{:?}", can_see);
            }
        }
        println!("Total Visible: {}", total);
    }
}

fn can_see(grid: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {

    let value = grid[row][column];
    println!("{row} {column} {value}");

    let mut can_see_top = true;
    let mut can_see_right = true;
    let mut can_see_bottom = true;
    let mut can_see_left = true;

    // Top
    println!("From Top:");
    for r in 0..row {
        let this_value = grid[r][column];
        println!("{this_value}");
        if this_value >= value {
            println!("Can't see!");
            can_see_top = false;
            break;
        }
    }
    if can_see_top {return true};
    // Right
    println!("From Right:");
    for c in column+1..99 {
        //println!("Checking {row}, {c}");
        let this_value = grid[row][c];
        println!("{this_value}");
        if this_value >= value {
            println!("Can't see!");
            can_see_right = false;
            break;
        }
    }
    if can_see_right {return true};
    // Bottom
    println!("From Bottom:");
    for r in row+1..99 {
        let this_value = grid[r][column];
        println!("{this_value}");
        if this_value >= value {
            println!("Can't see!");
            can_see_bottom = false;
            break;
        }
    }
    if can_see_bottom {return true};
    // Left
    println!("From Left:");
    for c in 0..column {
        let this_value = grid[row][c];
        println!("{this_value}");
        if this_value >= value {
            println!("Can't see!");
            can_see_left = false;
            break;
        }
    }
    return can_see_left;
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
