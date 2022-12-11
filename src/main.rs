use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
struct Monkey {
    items: Vec<u128>,
    true_recpt: usize,
    false_recpt: usize,
    inspections: u64,
    operation: fn(&u128) -> u128,
    test: fn(&u128) -> bool,
}

impl Monkey {
    fn new(
        true_recpt: usize,
        false_recpt: usize,
        items: Vec<u128>,
        operation: fn(&u128) -> u128,
        test: fn(&u128) -> bool,
    ) -> Monkey {
        return Monkey {
            items: items,
            true_recpt: true_recpt,
            false_recpt: false_recpt,
            operation: operation,
            test: test,
            inspections: 0,
        };
    }
}

fn main() {

    let mut monkeys: Vec<Monkey> = Vec::new();

    monkeys.push(Monkey::new(4, 3, vec![80],
        |v| v * 5, |v| v %  2 == 0));
    monkeys.push(Monkey::new(5, 6, vec![75, 83, 74],
        |v| v + 7, |v| v %  7 == 0));
    monkeys.push(Monkey::new(7, 0, vec![86, 67, 61, 96, 52, 63, 73],
        |v| v + 5, |v| v %  3 == 0));
    monkeys.push(Monkey::new(1, 5, vec![85, 83, 55, 85, 57, 70, 85, 52],
        |v| v + 8, |v| v % 17 == 0));
    monkeys.push(Monkey::new(3, 1, vec![67, 75, 91, 72, 89],
        |v| v + 4, |v| v % 11 == 0));
    monkeys.push(Monkey::new(6, 2, vec![66, 64, 68, 92, 68, 77],
        |v| v * 2, |v| v % 19 == 0));
    monkeys.push(Monkey::new(2, 7, vec![97, 94, 79, 88],
        |v| v.pow(2), |v| v %  5 == 0));
    monkeys.push(Monkey::new(4, 0, vec![77, 85],
        |v| v + 6, |v| v % 13 == 0));

    for time in 0..10_000 {
        //println!("");
        println!("==={time}=== ({})", monkeys.len());
        for i in 0..monkeys.len() {
            print!("M{i}");
            //println!("");
            //println!("Monkey {i}: ");
            if monkeys[i].items.len() == 0 {
                continue;
            }
            for _ in 0..monkeys[i].items.len() {
                print!(".");
                //println!("");
                let mut value = monkeys[i].items.remove(0);
                //print!("{value}");
                value = (monkeys[i].operation)(&value);
                monkeys[i].inspections += 1;
                //print!(" -> {value}");
                //value /= 3;
                //print!(" -> {value}");
                //print!(" -> test?");
                let mut throw_to:usize = 99;
                if (monkeys[i].test)(&value) {
                    throw_to = monkeys[i].true_recpt as usize;
                    //print!(" YES!");
                } else {
                    throw_to = monkeys[i].false_recpt as usize;
                    //print!(" No!");
                }
                //print!(" Throw to {throw_to}");
                monkeys[throw_to].items.push(value % 9699690);
            }

        }
        println!("");
    }
    println!("");
    for i in 0..monkeys.len() {
        println!("{i}: {}", monkeys[i].inspections);
    }


    // File hosts must exist in current path before this produces output
    //if let Ok(lines) = read_lines("./10.input") {
    //    for line in lines {
    //        if let Ok(line_str) = line {
    //        }
    //    }
    //}
}



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
