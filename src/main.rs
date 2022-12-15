use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::cmp::{min, max};

struct Point {
    x: i32,
    y: i32,
}

struct Pair {
    sensor: Point,
    beacon: Point,
}

impl Pair {
    fn distance(&self) -> i32 {
        (self.beacon.x - self.sensor.x).abs() + (self.beacon.y - self.sensor.y).abs()
    }
}

fn main() {
    let mut pairs: Vec<Pair> = Vec::new();

    let the_y: i32 = 2000000;
    pairs.push(Pair{sensor: Point{x: 3088287, y: 2966967}, beacon: Point{x: 3340990,  y: 2451747}});
    pairs.push(Pair{sensor: Point{x: 289570,  y: 339999 }, beacon: Point{x: 20077,    y: 1235084}});
    pairs.push(Pair{sensor: Point{x: 1940197, y: 3386754}, beacon: Point{x: 2010485,  y: 3291030}});
    pairs.push(Pair{sensor: Point{x: 1979355, y: 2150711}, beacon: Point{x: 1690952,  y: 2000000}});
    pairs.push(Pair{sensor: Point{x: 2859415, y: 1555438}, beacon: Point{x: 3340990,  y: 2451747}});
    pairs.push(Pair{sensor: Point{x: 1015582, y: 2054755}, beacon: Point{x: 1690952,  y: 2000000}});
    pairs.push(Pair{sensor: Point{x: 1794782, y: 3963737}, beacon: Point{x: 2183727,  y: 4148084}});
    pairs.push(Pair{sensor: Point{x: 2357608, y: 2559811}, beacon: Point{x: 2010485,  y: 3291030}});
    pairs.push(Pair{sensor: Point{x: 2936,    y: 1218210}, beacon: Point{x: 20077,    y: 1235084}});
    pairs.push(Pair{sensor: Point{x: 2404143, y: 3161036}, beacon: Point{x: 2010485,  y: 3291030}});
    pairs.push(Pair{sensor: Point{x: 12522,   y: 1706324}, beacon: Point{x: 20077,    y: 1235084}});
    pairs.push(Pair{sensor: Point{x: 1989162, y: 3317864}, beacon: Point{x: 2010485,  y: 3291030}});
    pairs.push(Pair{sensor: Point{x: 167388,  y: 3570975}, beacon: Point{x: -1018858, y: 4296788}});
    pairs.push(Pair{sensor: Point{x: 1586527, y: 2233885}, beacon: Point{x: 1690952,  y: 2000000}});
    pairs.push(Pair{sensor: Point{x: 746571,  y: 1442967}, beacon: Point{x: 20077,    y: 1235084}});
    pairs.push(Pair{sensor: Point{x: 3969726, y: 3857699}, beacon: Point{x: 3207147,  y: 4217920}});
    pairs.push(Pair{sensor: Point{x: 1403393, y: 2413121}, beacon: Point{x: 1690952,  y: 2000000}});
    pairs.push(Pair{sensor: Point{x: 2343717, y: 3649198}, beacon: Point{x: 2183727,  y: 4148084}});
    pairs.push(Pair{sensor: Point{x: 1473424, y: 688269 }, beacon: Point{x: 2053598,  y: -169389}});
    pairs.push(Pair{sensor: Point{x: 2669347, y: 190833 }, beacon: Point{x: 2053598,  y: -169389}});
    pairs.push(Pair{sensor: Point{x: 2973167, y: 3783783}, beacon: Point{x: 3207147,  y: 4217920}});
    pairs.push(Pair{sensor: Point{x: 2011835, y: 3314181}, beacon: Point{x: 2010485,  y: 3291030}});
    pairs.push(Pair{sensor: Point{x: 1602224, y: 2989728}, beacon: Point{x: 2010485,  y: 3291030}});
    pairs.push(Pair{sensor: Point{x: 3928889, y: 1064434}, beacon: Point{x: 3340990,  y: 2451747}});
    pairs.push(Pair{sensor: Point{x: 2018358, y: 3301778}, beacon: Point{x: 2010485,  y: 3291030}});
    pairs.push(Pair{sensor: Point{x: 1811905, y: 2084187}, beacon: Point{x: 1690952,  y: 2000000}});
    pairs.push(Pair{sensor: Point{x: 1767697, y: 1873118}, beacon: Point{x: 1690952,  y: 2000000}});
    pairs.push(Pair{sensor: Point{x: 260786,  y: 1154525}, beacon: Point{x: 20077,    y: 1235084}});

    // example data:
    //let the_y: i32 = 10;
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

    let mut ys: HashSet<i32> = HashSet::new();
    for pair in pairs {
        let distance = pair.distance();
        if (pair.sensor.y - the_y).abs() <= distance {
            let left  = pair.sensor.x - (distance - (pair.sensor.y - the_y).abs());
            let right = pair.sensor.x + (distance - (pair.sensor.y - the_y).abs());
            println!("Covers {left} - {right}");
            let mut n = 0u32;
            for y in left..=right {
                ys.insert(y);
                n += 1;
            }
            println!("{n}");
        }
    }
    //let mut ys = ys.iter().collect::<Vec<&i32>>();
    //ys.sort();
    //println!("{:?}", ys);
    println!("{:?}", ys.len() - 1); // -1 because one beacon is on this line

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
