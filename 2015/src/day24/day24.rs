#[macro_use]
extern crate clap;
use clap::App;

extern crate regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

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

    let packages = parse_input(s.trim().split('\n').collect());
    let sum = packages.iter().fold(0, |sum, p| sum + p);

    split_into_3(&packages, sum / 3);
    split_into_4(&packages, sum / 4);
}


fn split_into_4(packages : &Vec<i32>, target : i32) {
    let mut all : HashSet<Vec<i32>> = HashSet::new();
    let mut min_length = std::usize::MAX;
    let groupings = generate_sum(&packages, target, Vec::new());

    for g in groupings {

        if g.len() <= min_length {

            let available_packages = packages.clone().into_iter().filter(|x| !g.contains(x)).collect();

            let second_grouping = generate_sum(&available_packages, target, Vec::new());

            for g2 in second_grouping {
                let available_packages_3rd : Vec<i32> = packages.clone().into_iter().filter(|x| !g.contains(x) && !g2.contains(x)).collect();

                // Shouldn't generate all 2nd groups...just make sure 1 exists
                let third_group_exists = sum_exists(&available_packages_3rd, target);

                if third_group_exists {
                    all.insert(g.clone());
                    min_length = std::cmp::min(min_length, g.len());
                }
            }
        }
    }

    let mut min_qe = std::usize::MAX;
    for a in all {
        if a.len() == min_length {
            let qe : usize = a.iter().fold(1, |qe, x| qe * *x as usize);
            min_qe = std::cmp::min(qe, min_qe);
        }
    }
    println!("Part 2: Min QE = {}", min_qe);
}


fn split_into_3(packages : &Vec<i32>, target : i32) {
    let mut all : HashSet<Vec<i32>> = HashSet::new();
    let mut min_length = std::usize::MAX;
    let groupings = generate_sum(&packages, target, Vec::new());

    for g in groupings {

        if g.len() <= min_length {

            let available_packages = packages.clone().into_iter().filter(|x| !g.contains(x)).collect();
            if sum_exists(&available_packages, target) {
                all.insert(g.clone());
                min_length = std::cmp::min(min_length, g.len());
            }
        }
    }

    let mut min_qe = std::usize::MAX;
    for a in all {
        if a.len() == min_length {
            let qe : usize = a.iter().fold(1, |qe, x| qe * *x as usize);
            min_qe = std::cmp::min(qe, min_qe);
        }
    }
    println!("Part 1: Min QE = {}", min_qe);
}


fn sum_exists(packages : &Vec<i32>, target : i32) -> bool {
    let mut exists = false;
    for (i,p) in packages.iter().enumerate() {
        if target - p == 0 {
            exists = true;
        } else if target - p > 0{
            let new_vec = packages[i+1..packages.len()].to_vec();
            
            exists = sum_exists(&new_vec, target - p);
        }

        if exists {
            break;
        }
    }


    exists
}

fn generate_sum(packages : &Vec<i32>, target : i32, potential : Vec<i32>) -> Vec<Vec<i32>> {
    let mut groupings = Vec::new();
    for (i,p) in packages.iter().enumerate() {
        if target - p == 0 {
            let mut group = potential.clone();
            group.push(*p);
            groupings.push(group.clone());
            //println!("Found! {:?}", group);
        } else if target - p > 0{
            let new_vec = packages[i+1..packages.len()].to_vec();
            let mut group = potential.clone();
            group.push(*p);
            
            groupings.append(&mut generate_sum(&new_vec, target - p, group)); 
        }
    }
    groupings
}



fn parse_input(input : Vec<&str>) -> Vec<i32> {

    let mut v = Vec::new();

    for s in input {
        if s.trim().len() > 0 {
            v.push(s.parse::<i32>().unwrap());
        }
    }

    v.sort_by(|a,b| b.cmp(a));
    v
}

