use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Value {
    value: i32,
    seen: bool,
}

impl Value {
    fn new(value: i32) -> Self {
        Self {
            value: value,
            seen: false,
        }
    }
}

fn main() {

    let mut list: Vec<Value> = Vec::new();
    if let Ok(lines) = read_lines("./20.input") {
        for line in lines {
            if let Ok(line_str) = line {
                list.push(Value::new(line_str.parse::<i32>().unwrap()));
            }
        }
    }

    let length = list.len();
    let mut pos = 0usize;
    loop {
        //println!("{}", list[pos]);
        if list[pos].seen {
            //println!("Skipping seen value {} at {}", list[pos].value, pos);
            pos +=1;
            continue;
        }
        //println!("{:?}", list.iter().map(|v| v.value).collect::<Vec<i32>>());
        let mut value = list.remove(pos);
        let new_pos = (pos as i32 + value.value).rem_euclid(length as i32 -1);
        value.seen = true;
        println!("Moving value {} from {} to {}", value.value, pos, new_pos);
        list.insert(new_pos as usize, value);

        if (new_pos as usize) < pos {
            pos += 1;
        }

        if pos >= length {
            break;
        }

    }
    //println!("{:?}", list.iter().map(|v| v.value).collect::<Vec<i32>>());
    let zero_index = list.iter().position(|v| v.value == 0).unwrap();
    println!("{zero_index}");
    let onek   = list[(zero_index + 1000).rem_euclid(length)].value;
    let twok   = list[(zero_index + 2000).rem_euclid(length)].value;
    let threek = list[(zero_index + 3000).rem_euclid(length)].value;
    println!("{onek} + {twok} + {threek} = {}", onek + twok + threek);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
