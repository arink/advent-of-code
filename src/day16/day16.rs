extern crate clap;
extern crate regex;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;


fn main() {
    let matches = App::new("day16")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<FILE> 'File containing ingredient information'")
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

    let aunts = parse_input(s.trim().split('\n').collect());

    let mut known = HashMap::new();
    known.insert("children", 3);
    known.insert("cats", 7);
    known.insert("samoyeds", 2);
    known.insert("pomeranians", 3);
    known.insert("akitas", 0);
    known.insert("vizslas", 0);
    known.insert("goldfish", 5);
    known.insert("trees", 3);
    known.insert("cars", 2);
    known.insert("perfumes", 1);

    find_match(&aunts, &known);
    find_match_adjusted(&aunts, &known);
}


fn find_match_adjusted(aunts : & HashMap<u32, HashMap<String, u32>>, known : & HashMap<&str, u32>) {

    for (key, value) in aunts {
        let mut found = true;
        for (obj_key, obj_value) in known {
            if value.contains_key(&obj_key.to_string()) {
                let aunts_num = value.get(&obj_key.to_string()).unwrap();
                if *obj_key == "cats" || *obj_key == "trees" {
                    if aunts_num <= obj_value {
                        found = false;
                        break;
                    } 
                } else if *obj_key == "pomeranians" || *obj_key == "goldfish" {
                    if aunts_num >= obj_value {
                        found = false;
                        break;
                    }
                } else if aunts_num != obj_value {
                    found = false;
                    break
                }
            }
        }
        if found {
            println!("(Adjusted) Aunt {} matches: {:?}", key, value);
            break;
        }
    }
}


fn find_match(aunts : & HashMap<u32, HashMap<String, u32>>, known : & HashMap<&str, u32>) {

    for (key, value) in aunts {
        let mut found = true;
        for (obj_key, obj_value) in known {
            if value.contains_key(&obj_key.to_string()) {
                if value.get(&obj_key.to_string()).unwrap() != obj_value {
                    found = false;
                    break
                }
            }
        }

        if found {
            println!("Aunt {} matches: {:?}", key, value);
            break;
        }
    }
}


fn parse_input(input : Vec<&str>) -> HashMap<u32, HashMap<String, u32>> {
    let mut aunts = HashMap::new();

    for s in input {
        let re = Regex::new(r"^Sue (\d+): (.*)").unwrap(); 
        for cap in re.captures_iter(s) {
            let number = cap.at(1).unwrap().parse::<u32>().unwrap();
            let owns = String::from(cap.at(2).unwrap());

            aunts.insert(number, HashMap::new()); 

            let own_re = Regex::new(r"(\w+): (\d+),?").unwrap();
            for own_cap in own_re.captures_iter(&owns) {
                let obj = String::from(own_cap.at(1).unwrap());
                let num_obj = own_cap.at(2).unwrap().parse::<u32>().unwrap();
                aunts.get_mut(&number).unwrap().insert(obj, num_obj);
            }
        }
    }

    aunts
}
