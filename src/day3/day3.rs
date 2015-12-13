extern crate clap;
use clap::App;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x : i32,
    y : i32,
}

fn main() {
    let matches = App::new("day3")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<FILE> 'Filename containing list of directions'")
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();
    let mut file = match File::open(filename) {
        Err(why) => panic!("Couldn't open {}: {}", filename, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't open {}: {}", filename, Error::description(&why)),
        Ok(_) => println!("Read file {}", filename),
    }

    println!("Santa visited {} houses", 
             num_houses_visited(s.clone(), 1));
    println!("Santa and Robo-Santa visited {} houses", 
             num_houses_visited(s.clone(), 2));
}


fn num_houses_visited(s : String, workers: usize) -> usize {
    assert!(workers > 0);
    let mut houses = HashMap::new();
    let mut xy = Vec::new();
    for _ in 0..workers {
        xy.push(Coordinate { x : 0, y : 0 });
    }
    houses.insert(xy[0], 1);
    for (i, c) in s.chars().enumerate() {
        let idx = i % workers;
        if new_coordinates(c, &mut xy[idx]) {
            let count = houses.entry(xy[idx]).or_insert(0);
            *count = *count + 1;
        }
    }
    houses.len()
}


fn new_coordinates(c : char, coord : &mut Coordinate) -> (bool) {
    let mut valid = true;
    if c == '^' {
        coord.y = coord.y + 1;
    } else if c == 'v' {
        coord.y = coord.y - 1;
    } else if c == '>' {
        coord.x = coord.x + 1;
    } else if c == '<' {
        coord.x = coord.x - 1;
    } else {
        valid = false;
    }

    valid
}


#[cfg(test)]
mod tests {
    use new_coordinates;
    use num_houses_visited;
    use Coordinate;

    #[test]
    fn coordinate_check() {
        let mut coord = Coordinate { x : 0, y : 0 };
        assert_eq!(true, new_coordinates('>', &mut coord));
        assert_eq!(1, coord.x);
        assert_eq!(0, coord.y);

        assert_eq!(true, new_coordinates('^', &mut coord));
        assert_eq!(1, coord.x);
        assert_eq!(1, coord.y);

        assert_eq!(true, new_coordinates('<', &mut coord));
        assert_eq!(0, coord.x);
        assert_eq!(1, coord.y);

        assert_eq!(true, new_coordinates('v', &mut coord));
        assert_eq!(0, coord.x);
        assert_eq!(0, coord.y);
    }

    #[test]
    fn visits() {
        assert_eq!(2, num_houses_visited(String::from(">"), 1));
        assert_eq!(4, num_houses_visited(String::from("^>v<"), 1));
        assert_eq!(2, num_houses_visited(String::from("^v^v^v^v"), 1));
        
        assert_eq!(3, num_houses_visited(String::from("^v"), 2));
        assert_eq!(3, num_houses_visited(String::from("^>v<"), 2));
        assert_eq!(11, num_houses_visited(String::from("^v^v^v^v^v"), 2));

    }

}
