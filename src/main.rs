use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;


#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Copy, Clone, Debug)]
struct Pair {
    sensor: Point,
    beacon: Point,
    distance: i32,
}

impl Pair {
    fn new(sensor: Point, beacon: Point) -> Self {
        Pair {
            sensor: sensor,
            beacon: beacon,
            distance: sensor.distance(&beacon),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Stretch {
    from: i32,
    to: i32,
}
impl Ord for Stretch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.from.cmp(&other.from)
    }
}

impl PartialOrd for Stretch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Stretch {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from 
   }
}

impl Eq for Stretch { }

fn main() {
    let mut pairs: Vec<Pair> = Vec::new();

    let max_coord = 4000000i32;
    pairs.push(Pair::new(Point{x: 3088287, y: 2966967}, Point{x: 3340990,  y: 2451747}));
    pairs.push(Pair::new(Point{x: 289570,  y: 339999 }, Point{x: 20077,    y: 1235084}));
    pairs.push(Pair::new(Point{x: 1940197, y: 3386754}, Point{x: 2010485,  y: 3291030}));
    pairs.push(Pair::new(Point{x: 1979355, y: 2150711}, Point{x: 1690952,  y: 2000000}));
    pairs.push(Pair::new(Point{x: 2859415, y: 1555438}, Point{x: 3340990,  y: 2451747}));
    pairs.push(Pair::new(Point{x: 1015582, y: 2054755}, Point{x: 1690952,  y: 2000000}));
    pairs.push(Pair::new(Point{x: 1794782, y: 3963737}, Point{x: 2183727,  y: 4148084}));
    pairs.push(Pair::new(Point{x: 2357608, y: 2559811}, Point{x: 2010485,  y: 3291030}));
    pairs.push(Pair::new(Point{x: 2936,    y: 1218210}, Point{x: 20077,    y: 1235084}));
    pairs.push(Pair::new(Point{x: 2404143, y: 3161036}, Point{x: 2010485,  y: 3291030}));
    pairs.push(Pair::new(Point{x: 12522,   y: 1706324}, Point{x: 20077,    y: 1235084}));
    pairs.push(Pair::new(Point{x: 1989162, y: 3317864}, Point{x: 2010485,  y: 3291030}));
    pairs.push(Pair::new(Point{x: 167388,  y: 3570975}, Point{x: -1018858, y: 4296788}));
    pairs.push(Pair::new(Point{x: 1586527, y: 2233885}, Point{x: 1690952,  y: 2000000}));
    pairs.push(Pair::new(Point{x: 746571,  y: 1442967}, Point{x: 20077,    y: 1235084}));
    pairs.push(Pair::new(Point{x: 3969726, y: 3857699}, Point{x: 3207147,  y: 4217920}));
    pairs.push(Pair::new(Point{x: 1403393, y: 2413121}, Point{x: 1690952,  y: 2000000}));
    pairs.push(Pair::new(Point{x: 2343717, y: 3649198}, Point{x: 2183727,  y: 4148084}));
    pairs.push(Pair::new(Point{x: 1473424, y: 688269 }, Point{x: 2053598,  y: -169389}));
    pairs.push(Pair::new(Point{x: 2669347, y: 190833 }, Point{x: 2053598,  y: -169389}));
    pairs.push(Pair::new(Point{x: 2973167, y: 3783783}, Point{x: 3207147,  y: 4217920}));
    pairs.push(Pair::new(Point{x: 2011835, y: 3314181}, Point{x: 2010485,  y: 3291030}));
    pairs.push(Pair::new(Point{x: 1602224, y: 2989728}, Point{x: 2010485,  y: 3291030}));
    pairs.push(Pair::new(Point{x: 3928889, y: 1064434}, Point{x: 3340990,  y: 2451747}));
    pairs.push(Pair::new(Point{x: 2018358, y: 3301778}, Point{x: 2010485,  y: 3291030}));
    pairs.push(Pair::new(Point{x: 1811905, y: 2084187}, Point{x: 1690952,  y: 2000000}));
    pairs.push(Pair::new(Point{x: 1767697, y: 1873118}, Point{x: 1690952,  y: 2000000}));
    pairs.push(Pair::new(Point{x: 260786,  y: 1154525}, Point{x: 20077,    y: 1235084}));

    // example data:
    //let max_coord = 40u32;
    //pairs.push(Pair{sensor: Point{x: 2, y: 18}, beacon: Point{x: -2, y: 15}});
    //pairs.push(Pair{sensor: Point{x: 9, y: 16}, beacon: Point{x: 10, y: 16}});
    //pairs.push(Pair{sensor: Point{x: 13, y: 2}, beacon: Point{x: 15, y: 3}});
    //pairs.push(Pair{sensor: Point{x: 12, y: 14}, beacon: Point{x: 10, y: 16}});
    //pairs.push(Pair{sensor: Point{x: 10, y: 20}, beacon: Point{x: 10, y: 16}});
    //pairs.push(Pair{sensor: Point{x: 14, y: 17}, beacon: Point{x: 10, y: 16}});
    //pairs.push(Pair{sensor: Point{x: 8, y: 7}, beacon: Point{x: 2, y: 10}});
    //pairs.push(Pair{sensor: Point{x: 2, y: 0}, beacon: Point{x: 2, y: 10}});
    //pairs.push(Pair{sensor: Point{x: 0, y: 11}, beacon: Point{x: 2, y: 10}});
    //pairs.push(Pair{sensor: Point{x: 20, y: 14}, beacon: Point{x: 25, y: 17}});
    //pairs.push(Pair{sensor: Point{x: 17, y: 20}, beacon: Point{x: 21, y: 22}});
    //pairs.push(Pair{sensor: Point{x: 16, y: 7}, beacon: Point{x: 15, y: 3}});
    //pairs.push(Pair{sensor: Point{x: 14, y: 3}, beacon: Point{x: 15, y: 3}});
    //pairs.push(Pair{sensor: Point{x: 20, y: 1}, beacon: Point{x: 15, y: 3}});

    //println!("{:?}", pairs);
    'yloop: for the_y in 0i32..=max_coord {
        let mut ranges: Vec<Stretch> = Vec::new();
        for pair in pairs.iter() {
            if (pair.sensor.y - the_y).abs() <= pair.distance {
                let left  = pair.sensor.x - (pair.distance - (pair.sensor.y - the_y).abs());
                let right = pair.sensor.x + (pair.distance - (pair.sensor.y - the_y).abs());
                if ! (left > max_coord || right < 0) {
                    ranges.push(Stretch{from: left, to: right});
                }
                //println!("Covers {left} - {right}");
            }
        }
        ranges.sort();
        let mut r = 1usize;
        loop {
            // if r is completely within r-1, remove r, it's just getting in the way
            if ranges[r].from >= ranges[r-1].from && ranges[r].to <= ranges[r-1].to {
                ranges.remove(r);
                // r now points to the next range
                // continue without incrementing
                if r >= ranges.len() {
                    break;
                }
                continue;
            }
            if ranges[r-1].to < ranges[r].from {
                let foundy = the_y as u128;
                let foundx = ranges[r].from as u128 - 1;
                println!("Found at {foundx}, {foundy}");
                println!("Frequency: {}", foundx * max_coord as u128 + foundy);
                println!("{:?}", ranges);
                break 'yloop;
            }
            r += 1;
            if r >= ranges.len() {
                break;
            }
        }
        //break;
    }


}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
