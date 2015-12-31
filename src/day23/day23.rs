#[macro_use]
extern crate clap;
use clap::App;

extern crate regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

#[derive(Clone,Debug,PartialEq,Eq)]
enum Instruction {
    Half,
    Triple,
    Increment,
    Jump,
    JumpIfEven,
    JumpIfOne,
    Invalid,
}

#[derive(Clone,Debug)]
struct Operation {
    inst : Instruction,
    register : usize,
    offset : i32, // Offset if jump instruction
}

impl Operation {
    fn new(inst : Instruction, register : usize, offset : i32) -> Operation {
        Operation {
            inst : inst, 
            register : register,
            offset : offset,
        }
    }
}


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

    let instructions = parse_input(s.trim().split('\n').collect());
    /*
    for i in &instructions {
        println!("{:?}", i);
    }
    */

    run_program(&instructions, 0, 0);

    run_program(&instructions, 1, 0);
}


fn run_program(instructions : &Vec<Operation>, a : usize, b : usize) {
    let mut registers : Vec<usize> = vec![a, b];
    let mut pc = 0;

    loop {
        if pc < instructions.len() {
            //println!("PC {:2}.  {:?}.  Registers = {:?}", pc, instructions[pc], registers);
            let mut jumped = false;
            match instructions[pc].inst {
                Instruction::Half => {
                    registers[instructions[pc].register] /= 2;
                }
                Instruction::Triple => {
                    registers[instructions[pc].register] *= 3;
                }
                Instruction::Increment => {
                    registers[instructions[pc].register] += 1;
                }
                Instruction::Jump => {
                    pc = (pc as isize + instructions[pc].offset as isize) as usize; 
                    jumped = true;
                }
                Instruction::JumpIfEven => {
                    if registers[instructions[pc].register] % 2 == 0 {
                        pc = (pc as isize + instructions[pc].offset as isize) as usize; 
                        jumped = true
                    }
                }
                Instruction::JumpIfOne => {
                    if registers[instructions[pc].register] == 1 {
                        pc = (pc as isize + instructions[pc].offset as isize) as usize; 
                        jumped = true
                    }
                }
                _ => { println!("Unknown instruction"); assert!(false)}
            }

            if !jumped {
                pc += 1;
            }

        } else {
            break;
        }
    }
    println!("Registers = {:?}", registers);
}

fn parse_input(input : Vec<&str>) -> Vec<Operation> {

    let mut v = Vec::new();

    for s in input {
        if s.trim().len() > 0 {
            let mut inst = Instruction::Invalid;
            let mut register = 0;
            let mut offset = 0;

            let re = Regex::new(r"^(\w+) (\w)").unwrap();
            for cap in re.captures_iter(s) {
                let instruction_name = cap.at(1).unwrap();
                let register_name = cap.at(2).unwrap();

                if instruction_name == "inc" {
                    inst = Instruction::Increment;
                } else if instruction_name == "hlf" {
                    inst = Instruction::Half;
                } else if instruction_name == "tpl" {
                    inst = Instruction::Triple;
                }

                if register_name == "a" {
                    register = 0;
                } else if register_name == "b" {
                    register = 1;
                }
                assert!(register < 2);
                
            }

            if inst == Instruction::Invalid {
                let re = Regex::new(r"^jmp ([+-]\d+)").unwrap();
                for cap in re.captures_iter(s) {
                    inst = Instruction::Jump;
                    offset = cap.at(1).unwrap().parse::<i32>().unwrap();
                }
            }

            if inst == Instruction::Invalid {
                let re = Regex::new(r"^(\w+) (\w), ([+-]\d+)").unwrap();


                for cap in re.captures_iter(s) {
                    let instruction_name = cap.at(1).unwrap();
                    let register_name = cap.at(2).unwrap();
                    offset = cap.at(3).unwrap().parse::<i32>().unwrap();

                    if instruction_name == "jio" {
                        inst = Instruction::JumpIfOne;
                    } else if instruction_name == "jie" {
                        inst = Instruction::JumpIfEven;
                    } else {
                        assert!(false);
                    }

                    if register_name == "a" {
                        register = 0;
                    } else if register_name == "b" {
                        register = 1;
                    }
                }
            }

            //println!("{} = {:?} register {} offset {}", s, inst, register, offset);

            assert!(register < 2);
            v.push(Operation::new(inst, register, offset));
        }
    }

    v
}

