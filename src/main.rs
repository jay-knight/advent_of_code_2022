use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Point {
    x: i8,
    y: i8,
    z: i8,
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        let mut neighbors: Vec<Point> = Vec::new();
        neighbors.push(Point {
            x: self.x+1,
            y: self.y,
            z: self.z,
        });
        neighbors.push(Point {
            x: self.x-1,
            y: self.y,
            z: self.z,
        });
        neighbors.push(Point {
            x: self.x,
            y: self.y+1,
            z: self.z,
        });
        neighbors.push(Point {
            x: self.x,
            y: self.y-1,
            z: self.z,
        });
        neighbors.push(Point {
            x: self.x,
            y: self.y,
            z: self.z+1,
        });
        neighbors.push(Point {
            x: self.x,
            y: self.y,
            z: self.z-1,
        });
        neighbors
    }
}


fn main() {
    if let Ok(lines) = read_lines("./18.input") {
        let mut points: HashSet<Point> = HashSet::new();
        for line in lines {
            if let Ok(line_str) = line {
                let coords: Vec<&str> = line_str.split(",").collect();
                points.insert(Point {
                    x: coords[0].parse::<i8>().unwrap(),
                    y: coords[1].parse::<i8>().unwrap(),
                    z: coords[2].parse::<i8>().unwrap(),
                });
                //println!("{:?}", points);
            }
        }

        let mut surface_area = 0u32;
        for point in points.clone() {
            for neighbor in point.neighbors().iter() {
                if ! points.contains(&neighbor) {
                    surface_area += 1;
                }
            }
        }
        println!("{surface_area}");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
