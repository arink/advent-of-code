extern crate clap;
extern crate regex;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;


#[derive(PartialEq, Debug, Clone)]
struct Route {
    start : String,
    end : String,
    length : usize,
}


impl Route {
  fn new (start : String, end : String, length : usize) -> Route {
    Route {
        start : start,
        end : end,
        length : length
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
struct TSP {
    routes : HashMap<String, Vec<Route>>,

    // Current state
    visited : Vec<String>,
    length : usize
}

impl TSP {
    fn new(map : HashMap<String, Vec<Route>>) -> TSP {
        TSP {
            routes : map, 
            visited : Vec::new(),
            length : 0,
        }
    }
}


fn main() {
    let matches = App::new("day9")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<FILE> 'File containing cities and distances'")
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

    let routes = create_map(s.trim().split('\n').collect());
  
    let tsp = TSP::new(routes);

    let (shortest_len, shortest_route) = generate_path(&tsp, true);
    println!("Shortest: Length = {} for route {:?}", shortest_len, shortest_route);
    
    let (longest_len, longest_route) = generate_path(&tsp, false);
    println!("Longest:  Length = {} for route {:?}", longest_len, longest_route);
}


fn generate_path(tsp : &TSP, shortest : bool) -> (usize, Vec<String>) {
    let mut best_length = std::usize::MAX;
    if !shortest {
        best_length = std::usize::MIN;
    }

    let mut best_route = Vec::new();

    if tsp.visited.len() != tsp.routes.len() {
        for (city, paths) in &tsp.routes {
            // Needs to be the previously visited city if there is one
            if !tsp.visited.is_empty() && tsp.visited.last().unwrap() != city {
                continue;
            }

            for p in paths {
                if ! tsp.visited.contains(&p.end) {
   
                    let mut tsp = tsp.clone();
                    if tsp.visited.is_empty() {
                        // if we haven't logged the current city yet
                        tsp.visited.push(p.start.clone());
                    }
                    tsp.visited.push(p.end.clone());
                    tsp.length += p.length;

                    let (updated_length, updated_route) = generate_path(&tsp, shortest);
                    if (shortest && updated_length < best_length) ||
                       (!shortest && updated_length > best_length) {
                        best_length = updated_length;
                        best_route = updated_route;
                    }
                }
            }
        }
    } else {
        best_length = tsp.length;
        best_route = tsp.visited.clone();
    }
    (best_length, best_route)
}

fn create_map(paths : Vec<&str>) ->  HashMap<String, Vec<Route>> {
    let mut map : HashMap<String, Vec<Route>> = HashMap::new();

    for c in paths {
        let re = Regex::new(r"^(\w+) to (\w+) = (\d+)").unwrap();
        for cap in re.captures_iter(c) {
            let start = String::from(cap.at(1).unwrap());
            let end = String::from(cap.at(2).unwrap());
            let len = cap.at(3).unwrap().parse::<usize>().unwrap();

            if !map.contains_key(&start) {
                map.insert(start.clone(), Vec::new());
            }
            map.get_mut(&start).unwrap().push(Route::new(start.clone(), end.clone(), len));

            if !map.contains_key(&end) {
                map.insert(end.clone(), Vec::new());
            }
            map.get_mut(&end).unwrap().push(Route::new(end.clone(), start.clone(), len));
 
        }
    }

    map
}


#[cfg(test)]
mod tests {
    use create_map;
    use generate_path;
    use TSP;

    #[test]
    fn nothing() {
        let input = vec!["London to Dublin = 464", "London to Belfast = 518", "Dublin to Belfast = 141"];
        let map = create_map(input);
        assert_eq!(3, map.len());

        assert_eq!(true, map.contains_key("London"));
        assert_eq!(true, map.contains_key("Dublin"));
        assert_eq!(true, map.contains_key("Belfast"));

        assert_eq!(2, map.get("London").unwrap().len());
        assert_eq!(2, map.get("Dublin").unwrap().len());

    
        let tsp = TSP::new(map);
        assert_eq!(605, generate_path(&tsp, true).0);
        assert_eq!(982, generate_path(&tsp, false).0);
    }
}
