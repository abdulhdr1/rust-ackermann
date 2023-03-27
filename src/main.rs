use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
};

struct Ackermann {
    x: u32,
    y: u32,
    cache: bool,
}

impl Ackermann {
    fn new(x: u32, y: u32, cache: bool) -> Self {
        Self { x, y, cache }
    }

    fn calculate(&self) -> u32 {
        if self.cache {
            match File::open("./cache.txt") {
                Err(_) => {
                    File::create("./cache.txt").expect("Unable to create file");
                }
                Ok(_) => match self.search_cache() {
                    Some(value_found) => return value_found,
                    None => {}
                },
            }
        }

        match (self.x, self.y) {
            (0, _) => self.y + 1,
            (_, 0) => {
                let value_found = Self::new(self.x - 1, 1, self.cache).calculate();
                if self.cache {
                    self.write_cache(value_found);
                }
                value_found
            }
            _ => {
                let value_found = Self::new(
                    self.x - 1,
                    Self::new(self.x, self.y - 1, self.cache).calculate(),
                    self.cache,
                )
                .calculate();

                if self.cache {
                    self.write_cache(value_found);
                }

                println!("Calculated {} for A({},{})", value_found, self.x, self.y);
                value_found
            }
        }
    }

    fn write_cache(&self, value_found: u32) -> () {
        let mut file = File::open("./cache.txt").expect("Unable to open file");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("Unable to read file");
        buffer.push_str(&format!("\"{}-{}\": {}\n", self.x, self.y, value_found));
        let mut file = File::create("./cache.txt").expect("Unable to create file");
        file.write_all(buffer.as_bytes())
            .expect("Unable to write file");
    }

    fn search_cache(&self) -> Option<u32> {
        let mut file = File::open("./cache.txt").expect("Unable to open file");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("Unable to read file");

        match buffer
            .split("\n")
            .find(|s| s.starts_with(&format!("\"{}-{}\"", self.x, self.y)))
        {
            None => None,
            Some(cache_value) => {
                let value_found = cache_value.split(":").collect::<Vec<&str>>()[1]
                    .trim()
                    .parse::<u32>()
                    .expect("Invalid cache found");

                println!("Cache hit for A({},{}): {}", self.x, self.y, value_found);
                return Some(value_found);
            }
        }
    }
}

fn main() {
    print!("In A (x,y) \n");
    print!("What's the starting value for x? ");
    let x = prompt_number();
    print!("What's the starting value for y? ");
    let y = prompt_number();
    print!("Should it use a caching layer? y/n ");
    let use_cache = prompt_cache();

    Ackermann::new(x, y, use_cache).calculate();
}

fn prompt_number() -> u32 {
    let _ = stdout().flush();

    let mut s = String::new();

    stdin().read_line(&mut s).expect("read error");

    s.split_whitespace()
        .next()
        .unwrap()
        .parse::<u32>()
        .expect("Pass a valid positive integer")
}

fn prompt_cache() -> bool {
    let _ = stdout().flush();
    let mut s = String::new();

    stdin().read_line(&mut s).expect("read error");

    let v = s.split_whitespace().next().unwrap();
    assert!(v == "y" || v == "n", "Pass a valid option");
    return v == "y";
}
