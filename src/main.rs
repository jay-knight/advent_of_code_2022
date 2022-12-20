use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug,Clone)]
struct Valve {
    flow: u8,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(flow: u8, tunnels: Vec<String>) -> Self {
        Valve {
            flow: flow,
            tunnels: tunnels,
        }
    }
}

#[derive(Debug,Clone)]
struct Route {
    open_valves: HashSet<String>,
    score: u32,
    minutes: u8,
}

impl Route {
    fn tick(&mut self) -> bool {
        self.minutes -= 1;
        self.minutes > 0
    }

    fn open_valve(&mut self, name: &str, flow: u8) -> bool {
        if ! self.tick() {
            return false;
        }
        self.open_valves.insert(name.to_owned());
        self.score += (30 - self.minutes as u32) * flow as u32;
        println!("New score: {}", self.score);
        true
    }

    fn valve_is_closed(&self, name: &str) -> bool {
        !self.open_valves.contains(name)
    }
}

fn score<'a>(valves: &HashMap<String, Valve>, name: &str, route: &'a mut Route) {
    let valve = &valves[name];
    if valve.flow > 0 && route.valve_is_closed(&name) {
        if !route.open_valve(&name, valve.flow) {
            return;
        }
    }
    // try all the tunnels, see which one has the highest score
    let mut max_score = 0u32;
    for tunnel in valve.tunnels.to_owned().into_iter().rev() {
        //println!("Go from {} to {}", name, tunnel);
        let mut new_route = route.to_owned();
        if ! new_route.tick() {
            return;
        }
        score(valves, &tunnel.to_owned(), &mut new_route);
        max_score = std::cmp::max(new_route.score, max_score);
    }
    route.score += max_score;

}

fn main() {

    let mut valves: HashMap<String,Valve> = HashMap::new();

    // {{{
    valves.insert(String::from("TZ"), Valve{ flow: 0, tunnels: vec![String::from("ZJ"), String::from("DM")]});
    valves.insert(String::from("LH"), Valve{ flow: 0, tunnels: vec![String::from("FP"), String::from("IS")]});
    valves.insert(String::from("AA"), Valve{ flow: 0, tunnels: vec![String::from("XU"), String::from("JH"), String::from("CD"), String::from("WY"), String::from("HK")]});
    valves.insert(String::from("GP"), Valve{ flow: 0, tunnels: vec![String::from("BO"), String::from("KL")]});
    valves.insert(String::from("GN"), Valve{ flow: 0, tunnels: vec![String::from("QO"), String::from("FP")]});
    valves.insert(String::from("QO"), Valve{ flow: 0, tunnels: vec![String::from("CA"), String::from("GN")]});
    valves.insert(String::from("JT"), Valve{ flow: 22, tunnels: vec![String::from("BL")]});
    valves.insert(String::from("DF"), Valve{ flow: 0, tunnels: vec![String::from("BO"), String::from("HK")]});
    valves.insert(String::from("UM"), Valve{ flow: 0, tunnels: vec![String::from("OS"), String::from("LE")]});
    valves.insert(String::from("KJ"), Valve{ flow: 0, tunnels: vec![String::from("YF"), String::from("UK")]});
    valves.insert(String::from("UX"), Valve{ flow: 23, tunnels: vec![String::from("WM"), String::from("ZI")]});
    valves.insert(String::from("ZI"), Valve{ flow: 0, tunnels: vec![String::from("UX"), String::from("AR")]});
    valves.insert(String::from("YF"), Valve{ flow: 0, tunnels: vec![String::from("KJ"), String::from("EK")]});
    valves.insert(String::from("SX"), Valve{ flow: 0, tunnels: vec![String::from("DM"), String::from("CD")]});
    valves.insert(String::from("KZ"), Valve{ flow: 0, tunnels: vec![String::from("FR"), String::from("LE")]});
    valves.insert(String::from("IH"), Valve{ flow: 0, tunnels: vec![String::from("DM"), String::from("IE")]});
    valves.insert(String::from("EL"), Valve{ flow: 0, tunnels: vec![String::from("WQ"), String::from("BO")]});
    valves.insert(String::from("CD"), Valve{ flow: 0, tunnels: vec![String::from("AA"), String::from("SX")]});
    valves.insert(String::from("OR"), Valve{ flow: 0, tunnels: vec![String::from("FP"), String::from("IR")]});
    valves.insert(String::from("EK"), Valve{ flow: 19, tunnels: vec![String::from("YF"), String::from("LK")]});
    valves.insert(String::from("UE"), Valve{ flow: 0, tunnels: vec![String::from("FP"), String::from("LG")]});
    valves.insert(String::from("WQ"), Valve{ flow: 0, tunnels: vec![String::from("EL"), String::from("DM")]});
    valves.insert(String::from("XI"), Valve{ flow: 0, tunnels: vec![String::from("YH"), String::from("DM")]});
    valves.insert(String::from("GO"), Valve{ flow: 0, tunnels: vec![String::from("BO"), String::from("CQ")]});
    valves.insert(String::from("IR"), Valve{ flow: 0, tunnels: vec![String::from("ZJ"), String::from("OR")]});
    valves.insert(String::from("WY"), Valve{ flow: 0, tunnels: vec![String::from("UI"), String::from("AA")]});
    valves.insert(String::from("JH"), Valve{ flow: 0, tunnels: vec![String::from("AA"), String::from("CA")]});
    valves.insert(String::from("WM"), Valve{ flow: 0, tunnels: vec![String::from("UX"), String::from("YH")]});
    valves.insert(String::from("OS"), Valve{ flow: 0, tunnels: vec![String::from("UM"), String::from("CA")]});
    valves.insert(String::from("AE"), Valve{ flow: 0, tunnels: vec![String::from("FP"), String::from("YH")]});
    valves.insert(String::from("LG"), Valve{ flow: 0, tunnels: vec![String::from("UE"), String::from("LE")]});
    valves.insert(String::from("IS"), Valve{ flow: 0, tunnels: vec![String::from("LH"), String::from("AR")]});
    valves.insert(String::from("XU"), Valve{ flow: 0, tunnels: vec![String::from("AA"), String::from("TU")]});
    valves.insert(String::from("KL"), Valve{ flow: 0, tunnels: vec![String::from("GP"), String::from("TU")]});
    valves.insert(String::from("LV"), Valve{ flow: 0, tunnels: vec![String::from("UK"), String::from("TU")]});
    valves.insert(String::from("UI"), Valve{ flow: 0, tunnels: vec![String::from("ZJ"), String::from("WY")]});
    valves.insert(String::from("IL"), Valve{ flow: 0, tunnels: vec![String::from("GW"), String::from("LK")]});
    valves.insert(String::from("XY"), Valve{ flow: 0, tunnels: vec![String::from("AZ"), String::from("CA")]});
    valves.insert(String::from("JF"), Valve{ flow: 15, tunnels: vec![String::from("FR"), String::from("BK")]});
    valves.insert(String::from("UK"), Valve{ flow: 18, tunnels: vec![String::from("LV"), String::from("KJ")]});
    valves.insert(String::from("CA"), Valve{ flow: 13, tunnels: vec![String::from("JH"), String::from("XY"), String::from("QO"), String::from("BK"), String::from("OS")]});
    valves.insert(String::from("BL"), Valve{ flow: 0, tunnels: vec![String::from("JT"), String::from("GW")]});
    valves.insert(String::from("GW"), Valve{ flow: 16, tunnels: vec![String::from("IL"), String::from("BL")]});
    valves.insert(String::from("CQ"), Valve{ flow: 0, tunnels: vec![String::from("ZJ"), String::from("GO")]});
    valves.insert(String::from("HK"), Valve{ flow: 0, tunnels: vec![String::from("DF"), String::from("AA")]});
    valves.insert(String::from("BO"), Valve{ flow: 4, tunnels: vec![String::from("GO"), String::from("GP"), String::from("EL"), String::from("DF")]});
    valves.insert(String::from("TU"), Valve{ flow: 11, tunnels: vec![String::from("XU"), String::from("IE"), String::from("KL"), String::from("LV")]});
    valves.insert(String::from("AZ"), Valve{ flow: 0, tunnels: vec![String::from("ZJ"), String::from("XY")]});
    valves.insert(String::from("FP"), Valve{ flow: 5, tunnels: vec![String::from("GN"), String::from("AE"), String::from("UE"), String::from("LH"), String::from("OR")]});
    valves.insert(String::from("LE"), Valve{ flow: 14, tunnels: vec![String::from("KZ"), String::from("LG"), String::from("UM")]});
    valves.insert(String::from("IE"), Valve{ flow: 0, tunnels: vec![String::from("IH"), String::from("TU")]});
    valves.insert(String::from("NZ"), Valve{ flow: 0, tunnels: vec![String::from("YH"), String::from("AR")]});
    valves.insert(String::from("DM"), Valve{ flow: 3, tunnels: vec![String::from("WQ"), String::from("IH"), String::from("TZ"), String::from("SX"), String::from("XI")]});
    valves.insert(String::from("YH"), Valve{ flow: 21, tunnels: vec![String::from("WM"), String::from("NZ"), String::from("AE"), String::from("XI")]});
    valves.insert(String::from("BK"), Valve{ flow: 0, tunnels: vec![String::from("JF"), String::from("CA")]});
    valves.insert(String::from("LK"), Valve{ flow: 0, tunnels: vec![String::from("EK"), String::from("IL")]});
    valves.insert(String::from("AR"), Valve{ flow: 20, tunnels: vec![String::from("IS"), String::from("NZ"), String::from("ZI")]});
    valves.insert(String::from("ZJ"), Valve{ flow: 9, tunnels: vec![String::from("IR"), String::from("AZ"), String::from("TZ"), String::from("UI"), String::from("CQ")]});
    valves.insert(String::from("FR"), Valve{ flow: 0, tunnels: vec![String::from("JF"), String::from("KZ")]});
    // }}}

    let mut route = Route {
        open_valves: HashSet::new(),
        score: 0,
        minutes: 30,
    };

    score(&valves, "AA", &mut route);
    println!("{}", route.score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
