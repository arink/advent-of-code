extern crate clap;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


fn main() {
    let matches = App::new("day17")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("-w <WEIGHT> 'Target weight' 
                         <FILE> 'File containing sizes'")
        .get_matches();

    let weight = matches.value_of("WEIGHT").unwrap().parse::<i32>().unwrap();

    let filename = matches.value_of("FILE").unwrap();
    let mut file = match File::open(filename) {
        Err(why) => panic!("Couldn't open {}: {}", filename, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", filename, Error::description(&why)),
        Ok(_) => println!("Read file {}", filename),
    }

    let sizes = parse_input(s.trim().split('\n').collect());

    println!("Target weight: {}", weight);

    let mut results = HashMap::new();
    num_solutions(sizes, weight, 0, &mut results);

    println!("Total combinations: {}", results.values().fold(0, |a,b| a + b));
    let min = results.keys().min().unwrap();
    println!("Minimum containers: {} ({} combinations)", min, results.get(min).unwrap());
}


fn num_solutions(sizes : Vec<i32>, weight : i32, num : i32, res : &mut HashMap<i32, i32>) {
    for (i, s) in sizes.iter().enumerate() {
        if weight - s == 0 {
            let entry = res.entry(num+1).or_insert(0);
            *entry += 1;
            continue;
        } else if weight - s > 0 {
            let tail = sizes[i+1..sizes.len()].to_vec();
            num_solutions(tail, weight - s, num + 1, res);
        }
    }
}


fn parse_input(input : Vec<&str>) -> Vec<i32> {
    let mut sizes = Vec::new();

    for s in input {
        if s.len() > 0 {
            sizes.push(s.parse::<i32>().unwrap());
        }
    }
    
    sizes.sort_by(|a, b| b.cmp(a));
    sizes
}
