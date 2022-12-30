use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone)]
struct Shape {
    shape: Vec<(i32, i32)>,
}

impl Shape {

    fn shift(&mut self, dir: (i32, i32), stack: &Self) -> bool {
        // Can we move that way?
        for p in self.shape.clone() {
            // Will I hit anything on the stack?
            if stack.shape.contains(&(p.0 + dir.0, p.1 + dir.1)) {
                //println!("Hit the stack");
                return false;
            }
            // Will I hit a wall?
            if p.0 + dir.0 > 6 || p.0 + dir.0 < 0 {
                //println!("Hit the wall");
                return false;
            }
            if p.1 + dir.1 < 0 {
                //println!("Hit the floor");
                return false;
            }
        }
        self.shape = self.shape.iter().map(|p| (p.0 + dir.0, p.1 + dir.1)).collect();
        true
    }

    fn up(&mut self, a: i32, stack: &Self) -> bool {
        self.shift((0, a), stack)
    }
    fn down(&mut self, stack: &Self) -> bool {
        //println!("Down!");
        self.shift((0, -1), stack)
    }
    fn right(&mut self, stack: &Self) -> bool {
        //println!("Right!");
        self.shift((1, 0), stack)
    }
    fn left(&mut self, stack: &Self) -> bool {
        //println!("Left!");
        self.shift((-1, 0), stack)
    }

    fn add(&mut self, other: &Self) {
        for p in other.shape.clone() {
            self.shape.push(p);
        }
    }

    fn height(&self) -> i32 {
        match self.shape.is_empty() {
            true => 0,
            false => self.shape.iter().max_by(|p,o| p.1.cmp(&o.1)).unwrap().1 + 1
        }
    }

    fn draw(&self) {
        for y in (0..self.height()+1).rev() {
            print!("|");
            for x in 0..=6 {
                if self.shape.contains(&(x,y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!("---------");
    }
}

fn main() {
    //let wind: Vec<Direction> = Vec::new();
    if let Ok(lines) = read_lines("./17.input") {
        for line in lines {
            if let Ok(line_str) = line {
                let mut shapes: Vec<Shape> = Vec::new();

                // ..####.
                let mut shape = Shape{shape: Vec::new()};
                shape.shape.push((2, 0));
                shape.shape.push((3, 0));
                shape.shape.push((4, 0));
                shape.shape.push((5, 0));
                shapes.push(shape);

                // ...#...
                // ..###..
                // ...#...
                let mut shape = Shape{shape: Vec::new()};
                shape.shape.push((3, 0));
                shape.shape.push((2, 1));
                shape.shape.push((3, 1));
                shape.shape.push((4, 1));
                shape.shape.push((3, 2));
                shapes.push(shape);

                // ....#..
                // ....#..
                // ..###..
                let mut shape = Shape{shape: Vec::new()};
                shape.shape.push((2, 0));
                shape.shape.push((3, 0));
                shape.shape.push((4, 0));
                shape.shape.push((4, 1));
                shape.shape.push((4, 2));
                shapes.push(shape);

                // ..#....
                // ..#....
                // ..#....
                // ..#....
                let mut shape = Shape{shape: Vec::new()};
                shape.shape.push((2, 0));
                shape.shape.push((2, 1));
                shape.shape.push((2, 2));
                shape.shape.push((2, 3));
                shapes.push(shape);

                // ..##...
                // ..##...
                let mut shape = Shape{shape: Vec::new()};
                shape.shape.push((2, 0));
                shape.shape.push((3, 0));
                shape.shape.push((2, 1));
                shape.shape.push((3, 1));
                shapes.push(shape);


                let wind: Vec<Direction> = line_str.trim().chars().map(|c| {
                    match c {
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        _   => panic!("Not a direction"),
                    }
                }).collect();
                //let wind: Vec<Direction> = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".trim().chars().map(|c| {
                //    match c {
                //        '<' => Direction::Left,
                //        '>' => Direction::Right,
                //        _   => panic!("Not a direction"),
                //    }
                //}).collect();
                println!("{:?}", wind);

                let mut shape_idx = 0;
                let mut wind_idx = 0;
                let mut dropped = 0;
                let mut stack = Shape { shape: vec![] };

                'shape_loop: loop {
                    // Are we done?
                    dropped += 1;
                    if dropped > 2022 {
                        println!("Final height: {}", stack.height());
                        //println!("{:?}", stack.shape);
                        //stack.draw();
                        break;
                    }
                    //stack.draw();
                    println!("Dropping {dropped}!");
                    // Place next shape 3 above height
                    if shape_idx >= shapes.len() {
                        shape_idx = 0;
                    }
                    let mut shape: Shape = shapes.get(shape_idx).unwrap().clone();
                    shape_idx += 1;
                    //println!("Stack height: {}", stack.height());
                    shape.up(stack.height() + 3, &stack);
                    //shape.draw();

                    // Now let the wind blow it around for a while:
                    loop {
                        if wind_idx >= wind.len() {
                            wind_idx = 0;
                        }
                        //println!("wind: {wind_idx}");
                        let wind_dir = wind.get(wind_idx);
                        wind_idx += 1;
                        match wind_dir.unwrap() {
                            Direction::Left => shape.left(&stack),
                            Direction::Right => shape.right(&stack),
                        };
                        //shape.draw();
                        // Now it drops one, if it can:
                        if ! shape.down(&stack) {
                            //println!("We hit the bottom");
                            stack.add(&shape);
                            continue 'shape_loop;
                        }
                        //shape.draw();
                    }

                }

                break;
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
