use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

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
        let mut max = 0;
        let size = grid.len();
        for column in 0..size {
            for row in 0..size {
                let can_see = can_see(&grid, column, row);
                println!("Score: {can_see}");
                max = cmp::max(max, can_see);
            }
        }
        println!("Max Visible: {}", max);
    }
}

fn can_see(grid: &Vec<Vec<u32>>, row: usize, column: usize) -> u32 {

    let value = grid[row][column];
    let size = grid.len();
    println!("{row} {column} {value}");

    let mut can_see_top = 0;
    let mut can_see_right = 0;
    let mut can_see_bottom = 0;
    let mut can_see_left = 0;


    // Top
    println!("From Top:");
    if row == 0 {
        can_see_top = 0;
    } else {
        for r in (0..=row-1).rev() {
            let this_value = grid[r][column];
            can_see_top += 1;
            println!("{this_value}");
            if this_value >= value {
                break;
            }
        }
    }
    // Right
    println!("From Right:");
    if column == size {
        can_see_right = 0;
    } else {
        for c in column+1..=size-1 {
            let this_value = grid[row][c];
            can_see_right += 1;
            println!("{this_value}");
            if this_value >= value {
                break;
            }
        }
    }
    // Bottom
    println!("From Bottom:");
    if row == size {
        can_see_bottom = 0;
    } else {
        for r in row+1..=size-1 {
            let this_value = grid[r][column];
            can_see_bottom += 1;
            println!("{this_value}");
            if this_value >= value {
                break;
            }
        }
    }
    // Left
    println!("From Left:");
    if column == 0 {
        can_see_left = 0;
    } else {
        for c in (0..column).rev() {
            let this_value = grid[row][c];
            can_see_left += 1;
            println!("{this_value}");
            if this_value >= value {
                break;
            }
        }
    }
    println!("{can_see_top} {can_see_right} {can_see_bottom} {can_see_left}");
    return can_see_top * can_see_right * can_see_bottom * can_see_left;
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
