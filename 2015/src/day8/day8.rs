extern crate clap;
extern crate regex;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let matches = App::new("day8")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<FILE> 'File containing strings'")
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

   let lines: Vec<&str> = s.trim().split('\n').collect();

   let diff = lines.iter().map(|l| l.len() - num_chars_string(l)).fold(0, |diff, i| diff + i);
   let encode = lines.iter().map(|l| num_chars_encode(l) - l.len()).fold(0, |encode, i| encode + i);
   
   println!("Difference = {}", diff);
   println!("Encode = {}", encode);
}


fn num_chars_encode(s : &str) -> usize {
    let chars : Vec<char>= s.chars().collect();

    let mut cnt = 0;
    for c in &chars {
        if *c == '"' {
            cnt += 2;
        } else if *c == '\\' {
            cnt += 2;
        } else {
            cnt += 1;
        }
    }
    cnt + 2
}


fn num_chars_string(s : &str) -> usize {
    let chars : Vec<char>= s.chars().collect();

    let mut cnt = 0;
    let mut escape = false;
    let mut hex = 0;
    for c in &chars {
        if hex != 0  {
            hex -= 1;
            continue;
        }

        if *c == '\\' && escape == false {
            escape = true;
        } else {
            if escape && *c == 'x' {
                hex = 2;
            }
            cnt += 1;
            escape = false;
        }
    }
    cnt - 2
}


#[cfg(test)]
mod tests {
    use num_chars_string;

    #[test]
    fn num() {
        assert_eq!(0, num_chars_string("\"\""));
        assert_eq!(3, num_chars_string("\"abc\""));
    }
}
