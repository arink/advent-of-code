extern crate clap;
extern crate regex;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Seating {
    happy : HashMap<String, i32>,
}

impl Seating {
    fn new (name : String, happy : i32) -> Seating {
        let mut val = HashMap::new();
        val.insert(name, happy);
        Seating {
            happy : val,
        }
    }

    fn get (&self, name : & String) -> i32 {
        *self.happy.get(name).unwrap()
    }

    fn insert(&mut self, name : String, happiness : i32) {
        self.happy.insert(name, happiness);
    }
}


fn main() {
    let matches = App::new("day13")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<FILE> 'File containing input'")
        .get_matches();

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

    let input = parse_input(s.trim().split('\n').collect());

    // Create permutations of all the keys.  For example:
    //      a b c d
    //      a b d c
    //      a c b d
    //
    // This could be optimized to only create free cyclical
    // permutations instead of brute forcing all possible
    // permutations which leads to lots of duplicates.  For
    // example
    //      'a d b c' is the same as 'a c b d'
    let names = input.keys().map(|v| v.clone()).collect::<Vec<String>>();

    let perms = permutation(&names, 0);

    let max = score_permutations(input, perms);
    println!("Max = {}", max);
}


fn score_permutations(input : HashMap<String, Seating>, perms : Vec<Vec<String>>) -> i32 {
    let mut max : i32 = std::i32::MIN;

    for p in perms {
        let mut happy : i32 = 0;
        for (i, name) in p.iter().enumerate() {

            let mut left_index : isize = (i as isize - 1) % p.len() as isize;
            if left_index < 0 {
                left_index = p.len() as isize - 1;
            }
            let right_index : isize = (i as isize + 1) % p.len() as isize;

            let left = p[left_index as usize].clone();
            let right = p[right_index as usize].clone();

            let relationships = input[name].clone();

            happy += relationships.get(&left);
            happy += relationships.get(&right);
        }
        if happy > max {
            max = happy;
        }
    }

    max
}


fn permutation(available : & Vec<String>, index : usize) -> Vec<Vec<String>> {
    let mut v : Vec<Vec<String>> = Vec::new();

    if index == available.len() - 1 {
        v.push(available.clone());
    } else {
        let mut upd = available.clone();
        for j in index..upd.len() {
            upd.swap(index, j);
            let p : &mut Vec<Vec<String>> = &mut permutation(&upd, index + 1);
            v.append(p);
            upd.swap(index, j);
        }
    }

    v 
}


//fn parse_input(input : Vec<&str>) -> HashMap<String, Seating> {
fn parse_input(input : Vec<&str>) -> HashMap<String, Seating> {
    let mut relationships = HashMap::new();

    for s in input {
        let re = Regex::new(r"^(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).").unwrap();
        for cap in re.captures_iter(s) {
            let name : String = String::from(cap.at(1).unwrap());
            let result : String = String::from(cap.at(2).unwrap());
            let mut happy : i32 = cap.at(3).unwrap().parse::<i32>().unwrap();
            let from : String = String::from(cap.at(4).unwrap());

            if result == "lose" {
                happy *= -1;
            }

            if ! relationships.contains_key(&name) {
                relationships.insert(name, Seating::new(from, happy));
            }
            else {
                relationships.get_mut(&name).unwrap().insert(from, happy);
            }
        }
    }

    relationships
}
