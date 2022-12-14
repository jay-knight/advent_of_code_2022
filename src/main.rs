use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::{min, max};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

enum Thing {
    Rock,
    Sand,
}

fn points_between(first: &Point, second: &Point) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let xrange = min(first.x, second.x)..=max(first.x, second.x);
    let yrange = min(first.y, second.y)..=max(first.y, second.y);

    for x in xrange.clone() {
        for y in yrange.clone() {
            points.push(Point{x:x, y:y});
        }
    }
    return points;
}

fn print_area(area: HashMap<Point, Thing>) {
    let maxy = 561;
    let maxx = 157;
    for x in 0..=maxx {
        for y in 0..=maxy {
            match area.get(&Point{x:x, y:y}) {
                Some(Thing::Rock) => print!("#"),
                Some(Thing::Sand) => print!("o"),
                None => print!(" "),
            }
        }
        println!("");
    }
}

fn main() {
    let mut area: HashMap<Point, Thing> = HashMap::new();
    let maxy = 561;
    let maxx = 157;
    if let Ok(lines) = read_lines("./14.input") {
        for line in lines {
            if let Ok(line_str) = line {
                let points: Vec<String> = line_str.split(" -> ").map(|s| s.to_string()).collect();
                let mut point_vec: Vec<Point> = Vec::new();
                for point in points {
                    let (y,x) = point.split_once(",").unwrap();
                    point_vec.push(Point{
                        x: x.parse::<isize>().unwrap(),
                        y: y.parse::<isize>().unwrap(),
                    });
                }
                for p in 1..point_vec.len() {
                    let fp = point_vec[p-1];
                    let sp = point_vec[p];
                    let points = points_between(&fp, &sp);
                    for ap in points {
                        area.insert(ap, Thing::Rock);
                    }
                }
            }
        }
    }
    let points = points_between(
        &Point {
            x: 159,
            y: -100
        },
        &Point {
            x: 159,
            y: 1100
        });
    for ap in points {
        area.insert(ap, Thing::Rock);
    }

    let mut c = 0u32;
    'outer: loop {
        c += 1;
        println!("{c}");
        // start a new sand at 0, 500
        let mut sand_point = Point{ x : 0, y : 500};
        loop {
            if sand_point.x > 159 {
                // We're full
                panic!("Overflowed, make a bigger floor");
            } else if ! area.contains_key(&Point{x: sand_point.x + 1, y: sand_point.y}) {
                sand_point = Point{x: sand_point.x + 1, y: sand_point.y};
                continue;
            } else if ! area.contains_key(&Point{x: sand_point.x + 1, y: sand_point.y - 1}) {
                // Down and left
                sand_point = Point{x: sand_point.x + 1, y: sand_point.y - 1};
                continue;
            } else if ! area.contains_key(&Point{x: sand_point.x + 1, y: sand_point.y + 1}) {
                // Down and right
                sand_point = Point{x: sand_point.x + 1, y: sand_point.y + 1};
                continue;
            } else {
                // nowhere else to go, add the sand to the area
                area.insert(sand_point, Thing::Sand);
                if sand_point.x == 0 && sand_point.y == 500 {
                    break 'outer;
                }else {
                    break;
                }
            }
        }
    }
    print_area(area);
    println!("All Full, {c} grains of sand!");


}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
