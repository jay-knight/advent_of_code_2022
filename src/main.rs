use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Value {
    value: i128,
    initial: usize,
}

impl Value {
    fn new(value: i128, initial: usize) -> Self {
        Self {
            value: value,
            initial: initial,
        }
    }
}

fn mix(list: &mut Vec<Value>) {
    let length = list.len();
    let mut initial = 0usize;
    loop {
        let pos = list.iter().position(|v| v.initial == initial).unwrap();
        //println!("{}", list[pos]);
        //println!("{:?}", list.iter().map(|v| v.value).collect::<Vec<i128>>());
        let value = list.remove(pos);
        let new_pos = (pos as i128 + value.value).rem_euclid(length as i128 -1);
        println!("Moving value {} from {} to {}", value.value, pos, new_pos);
        list.insert(new_pos as usize, value);

        initial += 1;

        if initial >= length {
            break;
        }

    }
}

fn main() {

    let mut list: Vec<Value> = Vec::new();
    let key: i128 = 811589153;
    if let Ok(lines) = read_lines("./20.input") {
        let mut line_number: usize = 0;
        for line in lines {
            if let Ok(line_str) = line {
                list.push(Value::new(line_str.parse::<i128>().unwrap() * key, line_number));
            }
            line_number += 1;
        }
    }

    for _ in 0..10 {
        mix(&mut list);
    }

    let length = list.len();
    //println!("{:?}", list.iter().map(|v| v.value).collect::<Vec<i128>>());
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
