extern crate clap;
extern crate regex;
use clap::App;


fn main() {
    let matches = App::new("day20")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<NUM> 'Minimum present number'")
        .get_matches();

    let num = matches.value_of("NUM").unwrap().parse::<usize>().unwrap();

    find_house_number(num);
    find_house_number_part2(num);
}


fn find_house_number_part2(presents : usize) {
    let sz = presents/10;

    let mut v = vec![0; sz];

    for i in 1..sz+1 {
        let mut j = i;
        let mut cnt = 0;

        while j <= sz && cnt < 50 {
            let entry = v.get_mut(j-1).unwrap();
            *entry += i*11;  
            j += i;
            cnt += 1;
        }
    }

    for (i, e) in v.iter().enumerate() {
        if *e >= presents {
            println!("Part 2: House {} received {} presents", i+1, e);
            break;
        }
    }
}


fn find_house_number(presents : usize) {
    let target = presents / 10;

    let mut v = vec![0; target];

    for i in 1..target+1 {
        let mut j = i;
        while j <= target {
            let entry = v.get_mut(j-1).unwrap();
            *entry += i*10;  
            j += i;
        }
    }

    for (i, e) in v.iter().enumerate() {
        if *e >= presents {
            println!("Part 1: House {} received {} presents", i+1, e);
            break;
        }
    }
}
