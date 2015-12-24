extern crate clap;
extern crate regex;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Ingredient {
    capacity : i64, 
    durability : i64,
    flavor : i64,
    texture : i64, 
    calories : i64,
}


impl Ingredient {
    fn new(cap : i64, dur : i64, flav : i64, tex : i64, cal : i64) -> Ingredient {
        Ingredient {
            capacity : cap, 
            durability : dur,
            flavor : flav,
            texture : tex, 
            calories : cal
        }
    }
}


fn main() {
    let matches = App::new("day15")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("-t <TEASPOONS> 'Number of total teaspoons'
                         <FILE> 'File containing ingredient information'")
        .get_matches();

    let tsp = matches.value_of("TEASPOONS").unwrap().parse::<u32>().unwrap();

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

    let ingredients = parse_input(s.trim().split('\n').collect());

    let mut recipes : Vec<Vec<u32>> = Vec::new();
    let mut curr = vec![0; ingredients.len()];
    generate_recipes(tsp, 0, &mut curr, &mut recipes);

    score_recipes(-1, &ingredients, & recipes); 
    score_recipes(500, &ingredients, & recipes); 
}

fn score_recipes(calories : i64, ingredients : &HashMap<String, Ingredient>, recipes : & Vec<Vec<u32>>) {
    let mut max : i64 = std::i64::MIN;
    let mut max_score_recipe : Vec<u32> = Vec::new();
    let ingredient_names = ingredients.keys().cloned().collect::<Vec<String>>();

    for r in recipes {
        let mut score : Vec<i64> = vec![0; 4]; // Capcity, durability, flavor, texture
        let mut counted_calories : i64 = 0;
        for (i, amt) in r.iter().enumerate() {
            score[0] += *amt as i64 * ingredients[&ingredient_names[i]].capacity as i64;
            score[1] += *amt as i64 * ingredients[&ingredient_names[i]].durability as i64;
            score[2] += *amt as i64 * ingredients[&ingredient_names[i]].flavor as i64;
            score[3] += *amt as i64 * ingredients[&ingredient_names[i]].texture as i64;
            counted_calories += *amt as i64 * ingredients[&ingredient_names[i]].calories as i64;
        }

        if calories < 0 || calories == counted_calories {
            for s in &mut score {
                if *s < 0 {
                    *s = 0;
                }
            }

            let total : i64 = score.iter().fold(1, |total, i| total * i);
            if total > max {
                max = total;
                max_score_recipe = r.clone();
            }
        }
    }

    println!("Ingredients: {:?}.  {:?} = {}", ingredient_names, max_score_recipe, max);
}


fn generate_recipes(tsp : u32, index : usize, curr : &mut Vec<u32>, recipes : &mut Vec<Vec<u32>>) {
    if index == curr.len() - 1 {
        curr[index] = tsp;
        recipes.push(curr.clone());
    } else {
        for t in 0..tsp+1 {
            curr[index] = t;
            generate_recipes(tsp - t, index + 1, curr, recipes);
        }
    }
}


fn parse_input(input : Vec<&str>) -> HashMap<String, Ingredient> {
    let mut ingredients = HashMap::new();

    for s in input {
        let re = Regex::new(r"^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
        for cap in re.captures_iter(s) {
            let name = String::from(cap.at(1).unwrap());
            let cpy = cap.at(2).unwrap().parse::<i64>().unwrap();
            let dur = cap.at(3).unwrap().parse::<i64>().unwrap();
            let flv = cap.at(4).unwrap().parse::<i64>().unwrap();
            let tex = cap.at(5).unwrap().parse::<i64>().unwrap();
            let cal = cap.at(6).unwrap().parse::<i64>().unwrap();

            ingredients.insert(name, Ingredient::new(cpy, dur, flv, tex, cal));
        }
    }

    ingredients
}
