use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug,Clone)]
enum Operation {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug,Clone)]
struct Expression {
    left: String,
    right: String,
    operation: Operation,
}

#[derive(Debug,Clone)]
struct Monkey {
    value: Option<i128>,
    expression: Option<Expression>,
}

#[derive(Debug,Clone)]
struct Monkeys {
    monkeys: HashMap<String, Monkey>,
}

impl Monkeys {
    fn insert(&mut self, name: &str, value: Option<i128>, expression: Option<Expression>) {
        self.monkeys.insert(name.to_string(), Monkey {
            value: value,
            expression: expression,
        });
    }

    fn get_value(&mut self, name: &str) -> i128 {
        let monkey = self.monkeys.get(name).unwrap();
        match monkey.value {
            Some(value) => value,
            None => {
                let expression = monkey.expression.to_owned().unwrap();
                //println!("{:?} {:?}", self.get_value(&expression.left), self.get_value(&expression.right));
                let value = match expression.operation {
                    Operation::Plus   => self.get_value(&expression.left) + self.get_value(&expression.right),
                    Operation::Minus  => self.get_value(&expression.left) - self.get_value(&expression.right),
                    Operation::Times  => self.get_value(&expression.left) * self.get_value(&expression.right),
                    Operation::Divide => self.get_value(&expression.left) / self.get_value(&expression.right),
                };
                //self.monkeys.entry(String::from(name)).and_modify(|m| m.value = Some(value));
                value
            }
        }
    }

}

fn main() {
    if let Ok(lines) = read_lines("./21.input") {
        let mut monkeys = Monkeys{monkeys:HashMap::new()};
        for line in lines {
            if let Ok(line_str) = line {
                //println!("{line_str}");
                let (name, expression_str) = line_str.split_once(": ").unwrap();
                match expression_str.parse::<i128>() {
                    Ok(value) => monkeys.insert(name, Some(value), None),
                    Err(_) => {
                        let re = Regex::new(r"([a-z]{4}) ([\+\-\*/]) ([a-z]{4})").unwrap();
                        let caps = re.captures(expression_str).unwrap();
                        //println!("{:?}", caps);
                        monkeys.insert(name, None, Some(Expression {
                            left: String::from(caps.get(1).unwrap().as_str()),
                            right: String::from(caps.get(3).unwrap().as_str()),
                            operation: match caps.get(2).unwrap().as_str().chars().nth(0) {
                                Some('+') => Operation::Plus,
                                Some('-') => Operation::Minus,
                                Some('*') => Operation::Times,
                                Some('/') => Operation::Divide,
                                None      => panic!("I don't know this operation"),
                                _      => panic!("I don't know this operation"),
                            },

                        }));

                    },
                }
                //println!("{name} ... {expression}");
            }
        }

        for i in 0..100 {
            // this was kind of an iterative guessing game
            let value: i128 = 3403989691750 + (1*i);
            monkeys.monkeys.entry(String::from("humn")).and_modify(|m| m.value = Some(value));
            let root_monkey = monkeys.monkeys.get("root").unwrap();
            let root_left  = root_monkey.expression.to_owned().unwrap().left;
            let root_right = root_monkey.expression.to_owned().unwrap().right;
            let left_val = monkeys.get_value(&root_left);
            let right_val = monkeys.get_value(&root_right);
            println!("{} ({}): {} - {} = {}", i, value, left_val, right_val, left_val - right_val);
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
