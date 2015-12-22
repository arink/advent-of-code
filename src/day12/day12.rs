extern crate clap;
extern crate regex;
extern crate rustc_serialize;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use rustc_serialize::json::Json;

fn main() {
    let matches = App::new("day12")
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

    let input : Vec<&str> = s.trim().split('\n').collect();

    let mut sum = 0;
    let mut without_red = 0;
    for x in &input {
        sum += sum_all_numbers(&x);
        let data = Json::from_str(x).unwrap();
        without_red += sum_without_red(&data);
    }

    println!("Sum of all numbers: {}", sum);
    println!("Total without red: {}", without_red);
}




fn sum_without_red(data : &Json) -> i64 {
    let mut sum = 0;

    if data.is_object() {
        for (key, value) in data.as_object().unwrap() {
            if key == "red" || (value.is_string() && value.as_string().unwrap() == "red") {
                sum = 0;
                break;
            } else {
                sum += key.parse::<i64>().unwrap_or(0);
                sum += sum_without_red(value);
            }
        }
    } else if data.is_array() {
        for x in data.as_array().unwrap() {
            if x.is_number() {
                sum += x.as_i64().unwrap_or(0);
            } else {
                sum += sum_without_red(x);
            }
        }
    } else if data.is_number() {
        sum += data.as_i64().unwrap_or(0);
    }

    sum
}


fn sum_all_numbers(input : &str) -> i64 {
    let mut sum : i64 = 0;
    let re = Regex::new(r"(-?\d+)").unwrap();
    for cap in re.captures_iter(&input) {
        sum += cap.at(0).unwrap().parse::<i64>().unwrap();
    }
    sum
}


#[cfg(test)]
mod tests {
    use sum_all_numbers;
    use sum_without_red;
    use rustc_serialize::json::Json;

    #[test]
    fn test_sum() {
        {
            let input = "[1,2,3]";
            let sum = sum_all_numbers(input);
            assert_eq!(6, sum);

            let data = Json::from_str(input).unwrap();
            assert_eq!(6, sum_without_red(&data));
        }

        {
            let input = "{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}";
            let sum = sum_all_numbers(input);
            assert_eq!(15, sum);
       
            let data = Json::from_str(input).unwrap();
            assert_eq!(0, sum_without_red(&data));
        }

        {
            let input = "[1, \"red\", 5]";
            let sum = sum_all_numbers(input);
            assert_eq!(6, sum);
        
            let data = Json::from_str(input).unwrap();
            assert_eq!(6, sum_without_red(&data));
        }

        {
            let input = "[1,{\"c\":\"red\",\"b\":2},3]";
            let sum = sum_all_numbers(input);
            assert_eq!(6, sum);
        
            let data = Json::from_str(input).unwrap();
            assert_eq!(4, sum_without_red(&data));
        }

        {
            let input = "[5,[1,[1,{\"c\":\"red\",\"b\":2},3]]]";
            let sum = sum_all_numbers(input);
            assert_eq!(12, sum);
        
            let data = Json::from_str(input).unwrap();
            assert_eq!(10, sum_without_red(&data));
        }
    }
}
