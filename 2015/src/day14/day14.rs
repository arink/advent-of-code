extern crate clap;
extern crate regex;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ReindeerFlight {
    speed : u32, 
    flight_time : u32,
    rest_time : u32
}


impl ReindeerFlight {
    fn new(speed : u32, flight_time : u32, rest_time : u32) -> ReindeerFlight {
        ReindeerFlight {
            speed : speed, 
            flight_time : flight_time, 
            rest_time : rest_time,
        }
    }
}

fn main() {
    let matches = App::new("day14")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("-t <TIME> 'Total flight time'
                         <FILE> 'File containing Reindeer flight data'")
        .get_matches();

    let flight_time = matches.value_of("TIME").unwrap().parse::<u32>().unwrap();

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
    let flight_results = calculate_flight_distance(flight_time, input.clone());
    let distance_winner = reindeer_max_value(&flight_results); 
    println!("Distance Winner: {} with a distance of {}", distance_winner.0, distance_winner.1);

    let points_result = calculate_points_winner(flight_time, input.clone());
    let points_winner = reindeer_max_value(&points_result); 
    println!("Points winner: {} with {} points", points_winner.0, points_winner.1);
}


fn reindeer_max_value(values : & HashMap<String, u32>) -> (String, u32) {
    let mut name_of_max : String = String::new();
    let mut max_value : u32 = std::u32::MIN;
    for (key, value) in values {
        if *value > max_value {
            max_value = *value;
            name_of_max = key.clone();
        }
    }
    (name_of_max, max_value)
}


fn calculate_points_winner(time : u32, flight_data : HashMap<String, ReindeerFlight>) -> HashMap<String, u32> {
    let mut points : HashMap<String, u32> = HashMap::new();
    let mut distance : HashMap<String, u32> = HashMap::new();
   
    // Set cycle time for each reindeer.  Init distance/points to 0
    let mut cycles = HashMap::new();
    for (key, value) in &flight_data {
       let cycle = value.flight_time + value.rest_time;
       cycles.insert(key.clone(), cycle);
       distance.insert(key.clone(), 0);
       points.insert(key.clone(), 0);
    }


    // Find distance for each time
    for t in 1..time+1 {
        for (key, value) in &flight_data {
            let offset = (t-1) % cycles.get(key).unwrap();
            if offset < value.flight_time {
                if let Some(x) = distance.get_mut(key) {
                    *x += value.speed;
                }
            }
        }

        let distance_winner = reindeer_max_value(&distance); 

        // All reindeer in the lead get points (incl. ties)
        for (key, value) in &distance {
            if *value == distance_winner.1 {
                if let Some(x) = points.get_mut(key) {
                    *x += 1;
                }
            }
        }
    }

    points
}


fn calculate_flight_distance(time : u32, flight_data : HashMap<String, ReindeerFlight>) -> HashMap<String, u32> {
    let mut res = HashMap::new();
    for (key, value) in flight_data {
       let cycle = value.flight_time + value.rest_time;
       let full_cycles = time / cycle;
    
       let mut distance : f64 = full_cycles as f64 * value.speed as f64 * value.flight_time as f64;

       let leftover_time = time - full_cycles * cycle;
       if leftover_time >= value.flight_time {
           distance += value.speed as f64 * value.flight_time as f64;
       } else {
           distance += value.speed as f64 * leftover_time as f64;
       }
       res.insert(key, distance as u32);
    }
    res
}


fn parse_input(input : Vec<&str>) -> HashMap<String, ReindeerFlight> {
    let mut flight = HashMap::new();

    for s in input {
        let re = Regex::new(r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
        for cap in re.captures_iter(s) {
            let name : String = String::from(cap.at(1).unwrap());
            let flight_speed = cap.at(2).unwrap().parse::<u32>().unwrap();
            let flight_time = cap.at(3).unwrap().parse::<u32>().unwrap();
            let rest_time = cap.at(4).unwrap().parse::<u32>().unwrap();

            flight.insert(name, ReindeerFlight::new(flight_speed, flight_time, rest_time));
        }
    }

    flight
}
