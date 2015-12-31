extern crate clap;
extern crate regex;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;


#[derive(Clone,Debug)]
struct Item {
    name : String,
    cost : u32,
    damage : u32,
    cost_per_damage : f64,
    armor : u32,
    cost_per_armor : f64,
}

impl Item {
    fn new(name : &str, cost : u32, damage : u32, armor : u32) -> Item {
        let cost_per_damage =  cost as f64 / damage as f64;
        let cost_per_armor =  cost as f64 / armor as f64;

        Item {
            name : String::from(name),
            cost : cost, 
            damage : damage,
            cost_per_damage : cost_per_damage,
            armor : armor,
            cost_per_armor : cost_per_armor,
        }
    }
}

#[derive(Clone,Debug)]
struct Character {
    hp : i32,
    damage : u32,
    armor : u32,
}

impl Character {
    fn new(hp : i32, damage : u32, armor : u32) -> Character {
        Character {
            hp : hp, 
            damage : damage,
            armor : armor,
        }
    }
}


fn main() {
    let matches = App::new("day21")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<FILE> 'File containing boss stats'")
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

    let boss = parse_input(s.trim().split('\n').collect());
    println!("{:?}", boss);

    let mut turns_given_armor = Vec::new();
    for i in 0..12 {
        let mut damage = boss.damage as f64 - i as f64;
        if damage < 1.0 {
            damage = 1.0;
        }
        let turns = 100.0 / damage; 
        turns_given_armor.push(turns.ceil());
    }

    let mut damage_required_win : Vec<u32> = Vec::new();
    let mut damage_required_lose: Vec<u32> = Vec::new();
    for t in turns_given_armor {
        let min_damage = (boss.hp as f64 / t) + boss.armor as f64;
        damage_required_win.push(min_damage.ceil() as u32);
        damage_required_lose.push(min_damage.floor() as u32);
    }


    let weapons = build_weapon_list();
    let armors = build_armor_list();
    let (damage_ring_cost_low, damage_ring_cost_high, armor_ring_cost_low, armor_ring_cost_high) = build_ring_list();
    
    let mut least_cost = std::u32::MAX;
    for i in 0..boss.damage {
        let armor_cost = find_cheapest(i, &armors, &armor_ring_cost_low);
        let weapon_cost = find_cheapest(damage_required_win[i as usize], &weapons, &damage_ring_cost_low);
        if (armor_cost + weapon_cost) < least_cost {
            least_cost = armor_cost + weapon_cost;
        }
    }
    println!("Part 1: Least spent = {}", least_cost);

    let mut most_cost = std::u32::MIN;
    for i in 0..11 {
        let armor_cost = find_most_expensive_armor(i, &armors, &armor_ring_cost_high);
        let weapon_cost = find_most_expensive_weapon(damage_required_lose[i as usize], &weapons, &damage_ring_cost_high);
        if (armor_cost + weapon_cost) > most_cost && weapon_cost > 0 {
            most_cost = armor_cost + weapon_cost;
        }
    }
    println!("Part 2: Most spent and lost = {}", most_cost);

}


fn find_most_expensive_weapon(target : u32, items : &Vec<Item>, ring_effect_to_cost : &HashMap<u32,u32>) -> u32 {
    let mut cost = std::u32::MIN;

    if target > 0 {
        for a in items {
            let mut potential_cost = a.cost;
            let mut need = target as i32 - a.damage as i32;
            if need > 0 {
                let key = need as u32;
                if ring_effect_to_cost.contains_key(&key) {
                    potential_cost += *ring_effect_to_cost.get(&key).unwrap();        
                    need = 0;
                }
            }
    
            if need >= 0 && potential_cost > cost {
                cost = potential_cost;
            }
        }
    } else {
        cost = 0;
    }

    cost
}


fn find_most_expensive_armor(target : u32, items : &Vec<Item>, ring_effect_to_cost : &HashMap<u32,u32>) -> u32 {
    let mut cost = std::u32::MIN;

    if target > 0 {
        for a in items {
            let mut need = target as i32;
            let key = need as u32;
            let mut potential_cost = 0;

            if ring_effect_to_cost.contains_key(&key) {
                potential_cost += *ring_effect_to_cost.get(&key).unwrap();        
                need = 0;
            } else {
                // based on knowledge of ring input...
                let ring_key = 5;
                potential_cost += *ring_effect_to_cost.get(&ring_key).unwrap();        
                need -= 5;
            }

            if need > 0 && (need - (a.armor as i32 + a.damage as i32)) == 0 {
                potential_cost = potential_cost + a.cost;

            }
            if potential_cost > cost {
                cost = potential_cost;
            }
        }
    } else {
        cost = 0;
    }

    cost
}


fn find_cheapest(target : u32, items : &Vec<Item>, ring_effect_to_cost : &HashMap<u32,u32>) -> u32 {
    let mut cost = std::u32::MAX;

    if target > 0 {
        for a in items {
            let mut potential_cost = a.cost;
            let mut need = target as i32 - (a.armor as i32 + a.damage as i32);

            if need > 0 {
                let key = need as u32;
                if ring_effect_to_cost.contains_key(&key) {
                    potential_cost += *ring_effect_to_cost.get(&key).unwrap();        
                    need = 0;
                }
            }
    
            if need <= 0 && potential_cost < cost {
                cost = potential_cost;
            }
        }
    } else {
        cost = 0;
    }

    cost
}


fn choose_ring (rings : &Vec<Item>, lowest_cost : bool) -> HashMap<u32, u32> {
    let mut effect_to_cost = HashMap::new();

    for (i, el1) in rings.iter().enumerate() {
        let key = el1.damage + el1.armor; // Simplification because rings are either armor or damage, not both
        {
            let mut entry = effect_to_cost.entry(key).or_insert(el1.cost);
            if lowest_cost && *entry > el1.cost {
                *entry = el1.cost;
            } else if !lowest_cost && *entry < el1.cost {
                *entry = el1.cost;
            }
        }
        for el2 in rings.iter().skip(i+1) {
            let combined_key = key + el2.damage + el2.armor;
            let mut entry = effect_to_cost.entry(combined_key).or_insert(el2.cost + el1.cost);
            if lowest_cost && *entry > (el1.cost + el2.cost) {
                *entry = el1.cost + el2.cost;
            } else if !lowest_cost && *entry < (el1.cost + el2.cost) { 
                *entry = el1.cost + el2.cost;
            }
        }
    }

    effect_to_cost
}


fn build_weapon_list() -> Vec<Item> {
    let mut weapons = Vec::new();
    weapons.push(Item::new("Dagger",      8, 4, 0));
    weapons.push(Item::new("Shortsword", 10, 5, 0));
    weapons.push(Item::new("Warhammer",  25, 6, 0));
    weapons.push(Item::new("Longsword",  40, 7, 0));
    weapons.push(Item::new("Greataxe",   74, 8, 0));
    weapons
}


fn build_armor_list() -> Vec<Item> {
    let mut armors = Vec::new();
    armors.push(Item::new("Leather",     13, 0, 1));
    armors.push(Item::new("Chainmail",   31, 0, 2));
    armors.push(Item::new("Splintmail",  53, 0, 3));
    armors.push(Item::new("Bandedmail",  75, 0, 4));
    armors.push(Item::new("Platemail",  102, 0, 5));
    armors
}


fn build_ring_list() -> (HashMap<u32,u32>, HashMap<u32,u32>, HashMap<u32,u32>, HashMap<u32,u32>) {
    let mut damage_rings = Vec::new();
    let mut armor_rings = Vec::new();
    damage_rings.push(Item::new("Damage +1",   25, 1, 0));
    damage_rings.push(Item::new("Damage +2",   50, 2, 0));
    damage_rings.push(Item::new("Damage +3",  100, 3, 0));
    armor_rings.push(Item::new("Defense +1",  20, 0, 1));
    armor_rings.push(Item::new("Defense +2",  40, 0, 2));
    armor_rings.push(Item::new("Defense +3",  80, 0, 3));
    let damage_ring_cost_low = choose_ring(&damage_rings, true);
    let damage_ring_cost_high = choose_ring(&damage_rings, false);
    let armor_ring_cost_low = choose_ring(&armor_rings, true);
    let armor_ring_cost_high = choose_ring(&armor_rings, false);
    (damage_ring_cost_low, damage_ring_cost_high, armor_ring_cost_low, armor_ring_cost_high)
}


fn parse_input(input : Vec<&str>) -> Character {
    let mut hp = 0;
    let mut damage = 0;
    let mut armor = 0;

    for s in input {
        if s.trim().len() > 0 {
            let re = Regex::new(r"(.*): (\d+)").unwrap();
            for cap in re.captures_iter(s) {
                if cap.at(1).unwrap() == "Hit Points" {
                    hp = cap.at(2).unwrap().parse::<i32>().unwrap();
                } else if cap.at(1).unwrap() == "Damage" {
                    damage = cap.at(2).unwrap().parse::<u32>().unwrap();
                } else if cap.at(1).unwrap() == "Armor" {
                    armor = cap.at(2).unwrap().parse::<u32>().unwrap();
                }
            }
        }
    }

    Character::new(hp, damage, armor)
}

