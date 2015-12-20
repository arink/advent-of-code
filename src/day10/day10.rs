extern crate clap;
extern crate regex;
use clap::App;
use std::error::Error;


fn main() {
    let matches = App::new("day10")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("-i [ITERATIONS] 'Repeat sequence'
                         <INPUT> 'Input string'")
        .get_matches();

    let mut input : String = String::from(matches.value_of("INPUT").unwrap());
   
    let iterations = matches.value_of("ITERATIONS").unwrap();
    println!("Input: {}.  Run {} times", input, iterations);

    let num = iterations.parse::<usize>().unwrap();
    for _ in 0..num {
        input = convert(&input);
    }
    //println!("After {} iterations, {}", iterations, input);
    println!("Length = {}", input.len());
}

fn convert(s : &str) -> String {
    let mut cnt = 0;
    let mut cur : char = ' ';
    let mut res : String = String::new();

    for c in s.chars() {
        //println!("START: cur = {}, c = {}, cnt = {}", cur, c, cnt);
        if cnt == 0 {
            cur = c;
            cnt = 1;
        } else {
            if c != cur {
                res.push_str(& format!("{}{}", cnt, cur));
                cur = c;
                cnt = 1;
            } else {
                cnt += 1;
            }
        }
        //println!("END:   cur = {}, c = {}, cnt = {}", cur, c, cnt);
    }
    
    res.push_str(& format!("{}{}", cnt, cur));
    res
}

#[cfg(test)]
mod tests {
    use convert;

    #[test]
    fn parse() {
        assert_eq!("11", convert("1"));
        assert_eq!("21", convert("11"));
        assert_eq!("1211", convert("21"));
        assert_eq!("111221", convert("1211"));
    }
}
