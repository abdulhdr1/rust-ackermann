use std::{
    fs::{self, File},
    io::{stdin, stdout, Read, Write},
};
fn main() {
    print!("In A (x,y) \n");
    print!("What's the starting value for x? ");
    let _ = stdout().flush();
    let x = get_number();
    print!("What's the starting value for y? ");
    let _ = stdout().flush();
    let y = get_number();
    print!("Should it use a caching layer? y/n ");
    let _ = stdout().flush();
    let use_cache = use_caching();

    calculate_ackermann(x, y, use_cache);
}

fn get_number() -> u32 {
    let mut s = String::new();

    stdin().read_line(&mut s).expect("read error");

    let v = s
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<u32>()
        .expect("Pass a valid integer");
    v
}

fn use_caching() -> bool {
    let mut s = String::new();

    stdin().read_line(&mut s).expect("read error");

    let v = s.split_whitespace().next().unwrap();
    assert!(v == "y" || v == "n", "Pass a valid option");
    return v == "y";
}

fn calculate_ackermann(x: u32, y: u32, cache: bool) -> u32 {
    if cache {
        match File::open("./cache.txt") {
            Ok(mut text) => {
                let mut buffer = String::new();

                text.read_to_string(&mut buffer)
                    .expect("Unable to read file");

                match buffer
                    .split("\n")
                    .find(|s| s.starts_with(&format!("\"{}-{}\"", x, y)))
                {
                    Some(cache_value) => {
                        println!("Cache hit for A({},{})", x, y);
                        cache_value.split(":").collect::<Vec<&str>>()[1]
                            .trim()
                            .parse::<u32>()
                            .expect("Invalid cache found");
                    }
                    None => {}
                }
            }
            Err(_) => {
                File::create("./cache.txt").expect("Unable to create file");
            }
        }
    }

    println!("Calculating ({}, {})", x, y);
    let v: u32;
    match (x, y) {
        (0, _) => v = y + 1,
        (_, 0) => v = calculate_ackermann(x - 1, 1, cache),
        _ => v = calculate_ackermann(x - 1, calculate_ackermann(x, y - 1, cache), cache),
    }

    if cache {
        let mut file = File::open("./cache.txt").expect("Unable to open file");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("Unable to read file");
        buffer.push_str(&format!("\"{}-{}\": {}\n", x, y, v));
        fs::write("./cache.txt", buffer).expect("Unable to write file");
    };
    println!("Found {}, for A({}, {})", v, x, y);
    v
}
