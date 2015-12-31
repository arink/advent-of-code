#[macro_use]
extern crate clap;
use clap::App;

extern crate regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone,Debug)]
struct GameState {
    player : Character, 
    boss : Character,
    
    players_turn : bool,
    turns : u32,

    minimum_mana_usage : Arc<AtomicUsize>
}

impl GameState {
    fn new(player : Character, boss : Character) -> GameState {
        GameState {
            player : player, 
            boss : boss, 
            players_turn : true,
            turns : 0,
            minimum_mana_usage : Arc::new(AtomicUsize::new(std::usize::MAX))
        }
    }

    fn apply_boss_damage(&mut self) {
        if !self.players_turn {
            let damage = std::cmp::max(1, self.boss.damage - self.player.armor);
            self.player.hp -= damage as i32;
        }
    }

    fn apply_spells(&mut self) {
        self.player.armor = 0; // Init to no shield 
        for s in &mut self.player.active_spells {
            self.player.mana += s.mana_gain;// Recharge
            self.player.armor += s.armor;// Shield
            s.turns -= 1;
        }
        self.player.active_spells.retain(|x| x.turns != 0);
            
        for s in &mut self.boss.active_spells {
            self.boss.hp -= s.damage as i32; // Poison
            s.turns -= 1;
        }
        self.boss.active_spells.retain(|x| x.turns != 0);
    }

    fn spell_in_effect(&self, spell : &Spell) -> bool {
        let mut on = false;
        for s in &self.player.active_spells {
            if s.name == spell.name {
                on = true;
                break;
            }
        }

        if !on {
            for s in &self.boss.active_spells {
                if s.name == spell.name {
                    on = true;
                    break;
                }
            }

        }
        on
    }

    fn add_spell(&mut self, spell : Spell) {
        self.player.mana -= spell.mana_cost;
        self.player.mana_usage += spell.mana_cost;
        if spell.turns > 0 {
            if spell.name == "Shield" || spell.name == "Recharge" {
                self.player.active_spells.push(spell);
            } else {
                self.boss.active_spells.push(spell);
            }
        } else {
            self.player.hp += spell.heal as i32;
            self.boss.hp -= spell.damage as i32;
        }
    }
}


#[derive(Clone,Debug)]
struct Character {
    hp : i32,
    damage : u32,
    armor : u32,
    mana : u32,
    mana_usage : u32,
    active_spells : Vec<Spell>
}

impl Character {
    fn new(hp : i32, damage : u32, mana : u32) -> Character {
        Character {
            hp : hp, 
            damage : damage,
            armor : 0,
            mana : mana,
            mana_usage : 0,
            active_spells : Vec::new()
        }
    }
}


#[derive(Clone,Debug)]
struct Spell {
    name : String,
    mana_cost : u32,

    damage : u32,
    heal : u32,
    armor : u32,
    mana_gain : u32,
    turns : u32, // If turns = 0, instant
}

impl Spell {
    fn new(name : &str, cost : u32, damage : u32, heal : u32, armor : u32, gain : u32, turns : u32) -> Spell {
        Spell {
            name : String::from(name),
            mana_cost : cost, 
            damage : damage,
            heal : heal, 
            armor : armor, 
            mana_gain : gain,
            turns : turns
        }
    }
}


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let hp = matches.value_of("HP").unwrap_or("50").parse::<i32>().unwrap();
    let mana = matches.value_of("MANA").unwrap_or("500").parse::<u32>().unwrap();

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
    let player = Character::new(hp, 0, mana);
    let spells = build_spell_list();

    let easy = GameState::new(player.clone(), boss.clone());
    find_min_mana_usage(easy.clone(), &spells, false);
    println!("Easy Mode: Min mana used: {}", easy.minimum_mana_usage.load(Ordering::SeqCst));    

    let hard = GameState::new(player.clone(), boss.clone());
    find_min_mana_usage(hard.clone(), &spells, true);
    println!("Hard Mode: Min mana used: {}", hard.minimum_mana_usage.load(Ordering::SeqCst));    
}

fn find_min_mana_usage(state : GameState, spells : &Vec<Spell>, hard_mode : bool) -> u32 {
    let mut mana_usage = std::u32::MAX;
    let mut game = state.clone();

    game.turns += 1;

    if hard_mode && game.players_turn {
        game.player.hp -= 1;
    }

    if game.player.hp > 0 {
        game.apply_spells(); //Apply existing spells to player and boss

        if game.boss.hp <= 0 {
            // Player won
            mana_usage = game.player.mana_usage;
        } else if game.players_turn {
            game.players_turn = false;
            for s in spells {
                if !game.spell_in_effect(s) && game.player.mana >= s.mana_cost {
                    // Prune search space if mana usage is already greater than known winner
                    if ((game.player.mana_usage + s.mana_cost) as usize) < game.minimum_mana_usage.load(Ordering::SeqCst) {
                        let mut new_game_state = game.clone();
                        new_game_state.add_spell(s.clone());

                        if (new_game_state.player.mana_usage as usize) < new_game_state.minimum_mana_usage.load(Ordering::SeqCst) {
                            let new_mana_usage = find_min_mana_usage(new_game_state, spells, hard_mode);
                            if new_mana_usage < mana_usage && (new_mana_usage as usize) <  game.minimum_mana_usage.load(Ordering::SeqCst) {
                                mana_usage = new_mana_usage;
                                game.minimum_mana_usage.store(mana_usage as usize, Ordering::SeqCst);
                            }
                        }
                    }
                }
            }
        } else {
            // Run boss code
            game.apply_boss_damage();
            if game.player.hp > 0 {
                game.players_turn = true;
                // If neither player or boss won, start next round
                let new_mana_usage = find_min_mana_usage(game.clone(), spells, hard_mode);
                if new_mana_usage < mana_usage && (new_mana_usage as usize) <  game.minimum_mana_usage.load(Ordering::SeqCst) {
                    mana_usage = new_mana_usage;
                    game.minimum_mana_usage.store(mana_usage as usize, Ordering::SeqCst);
                }
            }
        }
    }
    mana_usage
}


fn build_spell_list() -> Vec<Spell> {
    let mut spells = Vec::new();
    spells.push(Spell::new("Shield",       113, 0, 0, 7,   0, 6));
    spells.push(Spell::new("Poison",       173, 3, 0, 0,   0, 6));
    spells.push(Spell::new("Magic Missle",  53, 4, 0, 0,   0, 0));
    spells.push(Spell::new("Drain",         73, 2, 2, 0,   0, 0));
    spells.push(Spell::new("Recharge",     229, 0, 0, 0, 101, 5));
    spells
}

fn parse_input(input : Vec<&str>) -> Character {
    let mut hp = 0;
    let mut damage = 0;

    for s in input {
        if s.trim().len() > 0 {
            let re = Regex::new(r"(.*): (\d+)").unwrap();
            for cap in re.captures_iter(s) {
                if cap.at(1).unwrap() == "Hit Points" {
                    hp = cap.at(2).unwrap().parse::<i32>().unwrap();
                } else if cap.at(1).unwrap() == "Damage" {
                    damage = cap.at(2).unwrap().parse::<u32>().unwrap();
                }
            }
        }
    }

    let boss = Character::new(hp, damage, 0);
    boss
}

