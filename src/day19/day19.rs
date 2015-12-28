extern crate clap;
extern crate regex;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;


fn main() {
    let matches = App::new("day19")
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

    let (molecule, replacement) = parse_input(s.trim().split('\n').collect());

    single_replacement(&molecule, &replacement);
    build_molecule(&molecule);
}

fn build_molecule(molecule : &str) {
    let elements: Vec<&str> = Vec::new();
    let re = Regex::new(r"([A-Z]{1}[a-z]?)").unwrap();

    let mut drop = 0;
    let mut total = 0;

    for cap in re.captures_iter(molecule) {
        if cap.at(1).unwrap() == "Y" {
            drop += 2;
        } else if cap.at(1).unwrap() == "Rn" {
            drop += 1;
        } else if cap.at(1).unwrap() == "Ar" {
            drop += 1;
        }
        total += 1;
    }

    println!("Part 2: Molecule Build: {}", total - drop - 1);
}

fn single_replacement(molecule : &str, replacements : & HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut results = HashSet::new();

    for (i, c) in molecule.chars().enumerate() {
        let mut curr = String::new();
        curr.push(c.clone());

        if replacements.contains_key(&curr) {
            for x in replacements.get(&curr).unwrap() {
                let mut new : String = String::from(molecule);

                new.truncate(i);
                new.push_str(x);
                new.push_str(&molecule[i+1..molecule.len()]);

                results.insert(new);
            }
        }

        if i > 0 {
            let mut combined = String::new(); 
            combined.push_str(&molecule[i-1..i+1]);
            if replacements.contains_key(&combined) {
                for x in replacements.get(&combined).unwrap() {
                    let mut new : String = String::from(molecule);

                    new.truncate(i-1);
                    new.push_str(x);
                    new.push_str(&molecule[i+1..molecule.len()]);

                    results.insert(new);
                }
            }
        }
    }

    println!("Part 1: {} entries", results.len());
    results
}

fn parse_input(input : Vec<&str>) -> (String, HashMap<String, Vec<String>>) {
    let mut molecule = String::new();
    let mut replacements = HashMap::new();

    for s in input {
        if s.trim().len() > 0 {
            let re = Regex::new(r"(\w+) => (\w+)").unwrap();
            if re.is_match(s) {
                let cap = re.captures(s).unwrap();
                let input = String::from(cap.at(1).unwrap());
                let output = String::from(cap.at(2).unwrap());
                let entry = replacements.entry(input).or_insert(Vec::new());
                entry.push(output);
            } else {
                molecule = String::from(s);
            }
        }
    }
    (molecule, replacements)
}

