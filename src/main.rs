use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;
use regex::RegexSet;

struct Cwd {
    dir: Vec<String>,
    sizes: HashMap<String, usize>,
}

fn dir_to_string(dir: Vec<String>) -> String {
    let mut dir_str = "/".to_string();
    dir_str.push_str(dir.join("/").as_str());
    return dir_str.to_string();
}

impl Cwd {
    fn up(&mut self) {
        //println!("----> in up()");
        // remove last element from self.dir
        self.dir.pop(); // last one is empty string
        //println!("----> new directory: {}", self.to_string());
    }

    fn cd(&mut self, to_dir: String) {
        //println!("----> in cd()");
        self.dir.push(to_dir);
        //println!("----> new directory: {}", self.to_string());
        // add to_dir to the end of self.dir
    }
    
    fn to_string(&self) -> String {
        return dir_to_string(self.dir.to_vec());
    }

    fn add_bytes(&mut self, bytes: usize) {
        let mut dir = self.dir.to_vec();
        loop {
            let dir_string = dir_to_string(dir.to_vec());
            *self.sizes.entry(dir_string.to_string()).or_insert(0) += bytes;
            println!("Adding {} to {:?} ({})", bytes, dir, self.sizes.get(&dir_string).unwrap());
            if let None = dir.pop() {
                break;
            }
        }
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./7.input") {
        // Consumes the iterator, returns an (Optional) String

        let mut cwd = Cwd {
            dir: vec![],
            sizes: HashMap::new(),
        };

        let set = RegexSet::new(&[
            r"^\$ cd /$",
            r"^\$ cd \.\.$",
            r"^\$ cd ([a-z]*)$",
            r"^\$ ls$",
            r"^dir (.*)$",
            r"^(\d+) (.*)$",
        ]).unwrap();

        for line in lines {
            if let Ok(line_str) = line {

                let matches:Vec<_> = set.matches(&line_str).into_iter().collect();
                println!("{line_str}");
                match matches[0] {
                    0 => (), //println!("--> cd /"),
                    1 => cwd.up(),
                    2 => cwd.cd(Regex::new(r"^\$ cd ([a-z]*)$").unwrap().captures(&line_str).unwrap().get(1).unwrap().as_str().to_string()),
                    3 => (), //println!("--> ls"),
                    4 => (), //println!("--> a directory"),
                    5 => cwd.add_bytes(Regex::new(r"^(\d+) (.*)$")
                           .unwrap()
                           .captures(&line_str)
                           .unwrap()
                           .get(1)
                           .unwrap()
                           .as_str()
                           .to_string()
                           .parse::<usize>()
                           .unwrap()),
                    _ => println!("--> SOMETHING ELSE"),
                }

            }
        }
        let mut total = 0;
        for (dir, size) in cwd.sizes.iter() {
            println!("{dir} has size {size}");
            if size <= &100000 {
                total += size;
                println!("Including! ({total})");
            }
        }
        println!("Total: {}", total);
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
