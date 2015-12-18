extern crate clap;
extern crate regex;
use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
enum Operation {
    And,
    Or,
    LShift,
    RShift,
    Not,
    Equal,
    Invalid,
}

#[derive(PartialEq, Debug, Clone)]
struct Instruction {
    operands : Vec<String>,
    result : String,
    value : u16,
    op : Operation,
}


impl Default for Instruction {
  fn default () -> Instruction {
    Instruction {operands : Vec::new(), result : String::new(), value: 0, op : Operation::Invalid,}
  }
}

fn main() {
    let matches = App::new("day7")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<FILE> 'File containing instructions'")
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

   let mut instructions: Vec<Instruction> = s.trim().split('\n').map(|i| parse_instruction(i)).collect();

   solve(&mut instructions, "a");
}


fn solve(instructions : &mut Vec<Instruction>, wire : &str) {

    let mut known : HashMap<String, Instruction> = HashMap::new();

    for i in instructions.iter() {
        if i.op == Operation::Equal && i.operands.is_empty() {
            known.insert(i.result.clone(), i.clone());
        }
    }

    while known.len() < instructions.len() {
        for inst in &mut instructions.iter_mut() {
            if inst.op == Operation::Invalid {
                continue;
            }

            if !inst.operands.is_empty() {
                if inst.operands.len() > 1 && known.contains_key(&inst.operands[1]) {
                    let value : u16 = known.get(&inst.operands[1]).unwrap().value;

                    inst.operands.remove(1);
                    inst.value = value;
                }

                if inst.operands.len() <= 1 && known.contains_key(&inst.operands[0]) {
                    let mut updated = inst.clone();
                    let value : u16 = known.get(&inst.operands[0]).unwrap().value;
                    /*
                    println!("\t{} {:?} {} where {} = {}", 
                             inst.operands[0], inst.op, inst.value, 
                             inst.operands[0], value);
                    */

                    match inst.op {
                        Operation::Not => updated.value = !value,
                        Operation::And => updated.value = updated.value & value,
                        Operation::Or =>  updated.value = updated.value | value,
                        Operation::LShift => updated.value = value << inst.value,
                        Operation::RShift => updated.value = value >> inst.value, 
                        Operation::Equal => updated.value = value,
                        _   => updated.value = updated.value,
                    }
                    //println!("Result: {} = {}", inst.operands[0], updated.value);

                    updated.operands.remove(0);
                    updated.op = Operation::Equal;
                    known.insert(updated.result.clone(), updated.clone());

                    inst.op = Operation::Invalid;
                }
            } else {
                // Must be an equal from initial parsing.  Set invalid
                inst.op = Operation::Invalid;
            }
        }
        /*
        println!("\n\nKnown: {}.  Total: {}.", known.len(), instructions.len());
        for (key, value) in &known {
            println!("\t\t{} = {}", key, value.value);
        }
        println!("\n\n");
        */
    }

    println!("{} = {}", wire, known.get(wire).unwrap().value);
}


fn parse_instruction(s : &str) -> Instruction {
    let mut inst : Instruction = Default::default();

    if s.len() > 0 {
        // Test for setting result
        //      ex: 1674 -> b
        let re = Regex::new(r"^(\w+) -> (\w+)").unwrap();
        for cap in re.captures_iter(s) {
            inst.result = String::from(cap.at(2).unwrap());
            let value = cap.at(1).unwrap();

            if value.parse::<u16>().is_ok() {
                inst.value = value.parse::<u16>().unwrap();
            } else {
                inst.operands.push(String::from(value));
            }

            inst.op = Operation::Equal;
        }

        if inst.op == Operation::Invalid {
            // Test for Shift
            //      ex: b RSHIFT 3 -> e
            let re = Regex::new(r"^(\w+) (\w)SHIFT (\d+) -> (\w+)").unwrap();
            for cap in re.captures_iter(s) {
                inst.operands.push(String::from(cap.at(1).unwrap()));
                inst.result = String::from(cap.at(4).unwrap());
                inst.value = cap.at(3).unwrap().parse::<u16>().unwrap();

                match cap.at(2).unwrap() {
                    "R" => inst.op = Operation::RShift,
                    "L" => inst.op = Operation::LShift,
                    _   => inst.op = Operation::Invalid,
                }
            }
        }

        if inst.op == Operation::Invalid {
            // Test for NOT
            //      ex: NOT b -> e
            let re = Regex::new(r"^NOT (\w+) -> (\w+)").unwrap();
            for cap in re.captures_iter(s) {
                inst.operands.push(String::from(cap.at(1).unwrap()));
                inst.result = String::from(cap.at(2).unwrap());
                inst.op = Operation::Not;
            }
        }

        if inst.op == Operation::Invalid {
            // Test for AND / OR
            //      ex: b OR e -> f
            let re = Regex::new(r"^(\w+) (\w+) (\w+) -> (\w+)").unwrap();
            for cap in re.captures_iter(s) {
                inst.operands.push(String::from(cap.at(1).unwrap()));
                inst.operands.push(String::from(cap.at(3).unwrap()));

                // If either operand can be parsed as a number, remove it and
                // put into value
                if inst.operands[1].parse::<u16>().is_ok() {
                    inst.value = inst.operands[1].parse::<u16>().unwrap();
                    inst.operands.remove(1);
                }
                
                if inst.operands[0].parse::<u16>().is_ok() {
                    inst.value = inst.operands[0].parse::<u16>().unwrap();
                    inst.operands.remove(0);
                }

                inst.result = String::from(cap.at(4).unwrap());
                match cap.at(2).unwrap() {
                    "AND" => inst.op = Operation::And,
                    "OR"  => inst.op = Operation::Or,
                    _     => inst.op = Operation::Invalid,
                }
            }
        }
    }

    inst
}





#[cfg(test)]
mod tests {
    use parse_instruction;
    use Instruction;
    use Operation;

    #[test]
    fn parser() {
        {
            let tst : Instruction = parse_instruction("1674 -> b");
            assert_eq!(0, tst.operands.len());
            assert_eq!("b", tst.result);
            assert_eq!(1674, tst.value);
            assert_eq!(Operation::Equal, tst.op); 
        }

        {
            let tst : Instruction = parse_instruction("lx -> b");
            assert_eq!(1, tst.operands.len());
            assert_eq!("b", tst.result);
            assert_eq!("lx", tst.operands[0]);
            assert_eq!(Operation::Equal, tst.op); 
        }

        {
            let tst : Instruction = parse_instruction("b RSHIFT 3 -> e");
            assert_eq!(1, tst.operands.len());
            assert_eq!("e", tst.result);
            assert_eq!(3, tst.value);
            assert_eq!(Operation::RShift, tst.op); 
        }

        {
            let tst : Instruction = parse_instruction("b LSHIFT 3 -> e");
            assert_eq!(1, tst.operands.len());
            assert_eq!("e", tst.result);
            assert_eq!(3, tst.value);
            assert_eq!(Operation::LShift, tst.op); 
        }

        {
            let tst : Instruction = parse_instruction("NOT b -> e");
            assert_eq!(1, tst.operands.len());
            assert_eq!("b", tst.operands[0]);
            assert_eq!("e", tst.result);
            assert_eq!(Operation::Not, tst.op); 
        }

        {
            let tst : Instruction = parse_instruction("b AND c -> e");
            assert_eq!(2, tst.operands.len());
            assert_eq!("b", tst.operands[0]);
            assert_eq!("c", tst.operands[1]);
            assert_eq!("e", tst.result);
            assert_eq!(Operation::And, tst.op); 
        }

        {
            let tst : Instruction = parse_instruction("b OR c -> e");
            assert_eq!(2, tst.operands.len());
            assert_eq!("b", tst.operands[0]);
            assert_eq!("c", tst.operands[1]);
            assert_eq!("e", tst.result);
            assert_eq!(Operation::Or, tst.op); 
        }

    }
}
