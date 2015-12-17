extern crate clap;
extern crate regex;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

#[derive(PartialEq, Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
    Invalid,
}

struct Instruction {
    top_left : (u32,u32),
    bottom_right : (u32,u32),
    op : Action,
}

impl Default for Instruction {
  fn default () -> Instruction {
    Instruction {top_left : (0,0), bottom_right : (0,0), op : Action::Invalid,}
  }
}

fn main() {
    let matches = App::new("day6")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<FILE> 'File containing light instructions'")
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

    
    let instructions: Vec<Instruction> = s.split('\n').map(|i| parse_instruction(i)).collect();
    let mut lights : [[u8; 1000]; 1000] = [[0; 1000]; 1000];
   
    // On/Off Check
    for i in &instructions {
        run_instruction(i, &mut lights);     
    }

    let mut cnt = 0;
    for x in lights.iter() {
        for y in x.iter() {
            if *y != 0 {
                cnt += 1;
            }
        }
    }
    println!("{} lights are on", cnt);
   
    // Reset
    let inst : Instruction = parse_instruction("turn off 0,0 through 999,999");
    run_instruction(&inst, &mut lights);

    // Brightness check
    for i in &instructions {
        run_brightness_instruction(i, &mut lights);     
    }

    let mut bright : u64 = 0;
    for x in lights.iter() {
        for y in x.iter() {
            bright += *y as u64;
        }
    }
    println!("Lights are {} bright", bright);
}


fn parse_instruction(s : &str) -> Instruction {
    let mut inst : Instruction = Default::default();
    let re = Regex::new(r"(.*)\s(\d+),(\d+) through (\d+),(\d+)").unwrap();
    for cap in re.captures_iter(s) {
        let top_left_x = cap.at(2).unwrap_or("0").parse::<u32>().unwrap();
        let top_left_y = cap.at(3).unwrap_or("0").parse::<u32>().unwrap();
        let bottom_right_x = cap.at(4).unwrap_or("0").parse::<u32>().unwrap();
        let bottom_right_y = cap.at(5).unwrap_or("0").parse::<u32>().unwrap();

        match cap.at(1).unwrap_or("") {
            "turn on" => inst.op = Action::TurnOn,
            "turn off" => inst.op = Action::TurnOff,
            "toggle" => inst.op = Action::Toggle,
            _ => println!("Fail!"),
        }
        inst.top_left = (top_left_x, top_left_y);
        inst.bottom_right = (bottom_right_x, bottom_right_y);
    }

    inst
}


fn run_instruction(inst : &Instruction, lights : &mut [[u8; 1000]; 1000]) {
    let start_x = inst.top_left.0;
    let stop_x = inst.bottom_right.0 + 1;
    let start_y = inst.top_left.1;
    let stop_y = inst.bottom_right.1 + 1;

    for x in start_x .. stop_x {
        for y in start_y .. stop_y {
            if inst.op == Action::TurnOn {
                lights[x as usize][y as usize] = 1;
            } else if inst.op == Action::TurnOff {
                lights[x as usize][y as usize] = 0;
            } else if inst.op == Action::Toggle {
                if lights[x as usize][y as usize] == 0 {
                    lights[x as usize][y as usize] = 1;
                } else {
                    lights[x as usize][y as usize] = 0;
                }
            }
        }
    }
}

fn run_brightness_instruction(inst : &Instruction, lights : &mut [[u8; 1000]; 1000]) {
    let start_x = inst.top_left.0;
    let stop_x = inst.bottom_right.0 + 1;
    let start_y = inst.top_left.1;
    let stop_y = inst.bottom_right.1 + 1;

    for x in start_x .. stop_x {
        for y in start_y .. stop_y {
            if inst.op == Action::TurnOn {
                lights[x as usize][y as usize] += 1;
            } else if inst.op == Action::TurnOff {
                if lights[x as usize][y as usize] > 0 {
                    lights[x as usize][y as usize] -= 1;
                }
            } else if inst.op == Action::Toggle {
                lights[x as usize][y as usize] += 2;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use parse_instruction;
    use run_instruction;
    use Instruction;
    use Action;

    #[test]
    fn parser() {
        {
            let tst : Instruction = parse_instruction("turn on 499,498 through 505,507");
            assert_eq!((499,498), tst.top_left); 
            assert_eq!((505,507), tst.bottom_right); 
            assert_eq!(Action::TurnOn, tst.op); 
        }

        {
            let tst : Instruction = parse_instruction("toggle 499,498 through 505,507");
            assert_eq!((499,498), tst.top_left); 
            assert_eq!((505,507), tst.bottom_right); 
            assert_eq!(Action::Toggle, tst.op); 
        }

        {
            let tst : Instruction = parse_instruction("turn off 499,498 through 505,507");
            assert_eq!((499,498), tst.top_left); 
            assert_eq!((505,507), tst.bottom_right); 
            assert_eq!(Action::TurnOff, tst.op); 
        }
    }


    fn check_entire_table(expected : u8, lights : & [[u8; 1000]; 1000]) {
        for x in lights.iter() {
            for y in x.iter() {
                assert_eq!(expected, *y);
            }
        }
    }

    #[test]
    fn instruction() {
        let mut lights : [[u8; 1000]; 1000] = [[0; 1000]; 1000];
        {
            let inst : Instruction = parse_instruction("turn on 0,0 through 999,999");
            run_instruction(&inst, &mut lights);
            check_entire_table(1, &lights);
        } 

        {
            let inst : Instruction = parse_instruction("toggle 0,0 through 999,999");
            run_instruction(&inst, &mut lights);
            check_entire_table(0, &lights);
        } 
        
        {
            let inst : Instruction = parse_instruction("turn on 0,0 through 999,999");
            let inst1 : Instruction = parse_instruction("turn off 0,0 through 999,999");
            run_instruction(&inst, &mut lights);
            run_instruction(&inst1, &mut lights);
            check_entire_table(0, &lights);
        } 
 
        {
            let inst : Instruction = parse_instruction("turn on 0,0 through 999,999");
            let inst1 : Instruction = parse_instruction("toggle 0,0 through 999,999");
            run_instruction(&inst, &mut lights);
            run_instruction(&inst1, &mut lights);
            check_entire_table(0, &lights);
        } 

        {
            let inst : Instruction = parse_instruction("turn on 0,0 through 999,0");
            run_instruction(&inst, &mut lights);
    
            for x in 0..1000 {
                assert_eq!(0x1, lights[x][0]);
                for y in 1..1000 {
                    assert_eq!(0x0, lights[x][y]);
                }
            }
        } 
    }
}
