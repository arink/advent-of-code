extern crate crypto;
extern crate clap;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


fn main() {
    let matches = App::new("day5")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<FILE> 'File containing naughty and nice strings'")
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

    let vec: Vec<&str> = s.split('\n').collect();

    println!("Nice: {}", count_nice_strings(vec.clone(), string_is_nice));
    println!("Adjusted Nice: {}", count_nice_strings(vec.clone(), string_is_nice_adjusted_rules));
}

fn string_is_nice_adjusted_rules(s : &str) -> bool {
    let mut pair = false;
    let mut repeat = false;

    let mut trip = Vec::new();
    let mut pairs = HashMap::new();
   
    for (i, c) in s.chars().enumerate() {
        trip.push(c);
        if trip.len() > 3 {
            trip.remove(0);
        }

        if trip.len() == 3 {
            if trip[0] == trip[2] {
                repeat = true;
            }

            let mut s = String::new();
            s.push(trip[1]);
            s.push(trip[2]);
            if pairs.contains_key(&s) {
                let idx = pairs.get(&s).unwrap();
                if i - idx != 1 {
                    pair = true;
                }
            } else {
                pairs.insert(s, i);
            }
        } else if trip.len() == 2 {
            let mut s = String::new();
            s.push(trip[0]);
            s.push(trip[1]);
            pairs.insert(s, i);
        }
    }
    pair && repeat
}


fn string_is_nice(s : &str) -> bool {
    let mut vowel_cnt = 0;
    let mut bad_str = false;
    let mut twice = false;

    let bad_dbl = vec!["ab", "cd", "pq", "xy"];
    let mut dbl = String::new();

    for c in s.chars() {
        dbl.push(c);
        if dbl.len() > 2 {
            dbl.remove(0);
        }

        if dbl.len() == 2 {
            let x : &str = &dbl[0..2];
            if bad_dbl.contains(&x) {
                bad_str = true;
                break;
            }

            if dbl.as_bytes()[0] == dbl.as_bytes()[1] {
                twice = true;
            }
        }

        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowel_cnt += 1;
        }
    }
    vowel_cnt >= 3 && twice && !bad_str
}

fn count_nice_strings(vec : Vec<&str>, func : fn(&str) -> bool) -> u64 {
    let mut cnt = 0;
    for x in vec {
        if x.len() > 0 && func(x) {
            cnt += 1;
        }
    }
    cnt
}


#[cfg(test)]
mod tests {
    use string_is_nice;
    use string_is_nice_adjusted_rules;
    
    #[test]
    fn nice_check() {
        assert_eq!(true, string_is_nice("ugknbfddgicrmopn"));
        assert_eq!(true, string_is_nice("aaa"));
        assert_eq!(false, string_is_nice("jchzalrnumimnmhp"));
        assert_eq!(false, string_is_nice("haegwjzuvuyypxyu"));
        assert_eq!(false, string_is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn nice_check_adjusted() {
        assert_eq!(true, string_is_nice_adjusted_rules("qjhvhtzxzqqjkmpb"));
        assert_eq!(true, string_is_nice_adjusted_rules("xxyxx"));
        assert_eq!(false, string_is_nice_adjusted_rules("uurcxstgmygtbstg"));
        assert_eq!(false, string_is_nice_adjusted_rules("ieodomkazucvgmuy"));
    }
}
