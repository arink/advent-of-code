#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let row = matches.value_of("ROW").unwrap().parse::<usize>().unwrap();
    let column = matches.value_of("COLUMN").unwrap().parse::<usize>().unwrap();

    let requires_row = row + column - 2;
    let num_calculations = (requires_row * (requires_row+1)) / 2 + column - 1;
    println!("Row {}, Column {}.  {} calculations", row, column, num_calculations);

    let multiply : usize = 252533;
    let divide : usize = 33554393;
    let mut value : usize = 20151125;

    for _ in 0..num_calculations {
        value = (value * multiply) % divide;
    }
    println!("{}", value);
}
