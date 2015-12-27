extern crate clap;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let matches = App::new("day17")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("-i <ITERATIONS> 'Num of iterations to run'
                         <FILE> 'File containing sizes'")
        .get_matches();

    let iterations = matches.value_of("ITERATIONS").unwrap().parse::<u32>().unwrap();
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

    let lights = parse_input(s.trim().split('\n').collect());

    change_lights(false, iterations, &lights);
    change_lights(true, iterations, &lights);
}


fn new_state(row : usize, col : usize, curr : & Vec<Vec<u8>>) -> u8 {
    let mut num_on_neighbors = 0;
    let mut curr_state = curr[row][col];

    let row_range = (std::cmp::max(0, row as i32 - 1), 
                     std::cmp::min(curr.len() as i32, row as i32 + 2));

    let col_range = (std::cmp::max(0, col as i32 - 1), 
                     std::cmp::min(curr[0].len() as i32, col as i32 + 2));

    for i in row_range.0..row_range.1 {
        for j in col_range.0..col_range.1 {
            if i as usize != row || j as usize != col {
                num_on_neighbors += curr[i as usize][j as usize];
            }
        }
    }

    if curr_state == 1 && (num_on_neighbors != 2 && num_on_neighbors != 3) {
        curr_state = 0;
    } else if curr_state == 0 && num_on_neighbors == 3 {
        curr_state = 1;
    }
    curr_state
}


fn change_lights(corners_on : bool, iterations : u32, initial_lights : & Vec<Vec<u8>>) {
    let mut lights = initial_lights.clone();
    let rows = initial_lights.len();
    let cols = initial_lights[0].len();

    // If corners are stuck on, change lights
    if corners_on {
        lights[0][0] = 1;
        lights[rows - 1][0] = 1;
        lights[0][cols - 1] = 1;
        lights[rows - 1][cols - 1] = 1;
    }
    
    for _ in 0..iterations {
        let mut updated_lights = lights.clone();
        for (i, row) in lights.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                updated_lights[i][j] = new_state(i, j, &lights);

                // force corners
                if corners_on {
                    updated_lights[0][0] = 1;
                    updated_lights[rows - 1][0] = 1;
                    updated_lights[0][cols - 1] = 1;
                    updated_lights[rows - 1][cols - 1] = 1;
                }

            }
        }
        lights = updated_lights.clone();
    }

    println!("After {} iterations, {} lights are on", iterations, num_lights_on(&lights));
}


fn num_lights_on(lights : &Vec<Vec<u8>>) -> u32 {
    lights.iter().fold(0, |sum, col| sum + 
                       col.iter().fold(0, |x, y| x as u32 + *y as u32))
}


fn parse_input(input : Vec<&str>) -> Vec<Vec<u8>> {
    let mut lights = Vec::new();

    for s in input {
        lights.push(Vec::new());
        for c in s.chars() {
            let mut state = 0;
            if c == '#' {
                state = 1;
            }
            lights.last_mut().unwrap().push(state);
        }
    }
    lights
}
