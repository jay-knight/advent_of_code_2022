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
    name: String,
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
            name: name.to_string(),
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
                self.monkeys.entry(String::from(name)).and_modify(|m| m.value = Some(value));
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
        println!("{}", monkeys.get_value("root"));
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
