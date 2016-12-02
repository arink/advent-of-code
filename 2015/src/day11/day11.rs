extern crate clap;
use clap::App;

const INVALID : [u8; 3] = ['i' as u8, 'o' as u8, 'l' as u8];
const FIRST : u8 = 'a' as u8;
const LAST: u8 =  'z' as u8;


fn main() {
    let matches = App::new("day11")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("-i [ITERATIONS] 'Number of password iterations'
                         <INPUT> 'Input string'")
        .get_matches();

    let input : String = String::from(matches.value_of("INPUT").unwrap());
    let iterations : usize = matches.value_of("ITERATIONS").unwrap().parse::<usize>().unwrap(); 
    println!("Input: {}", input);
    
    let mut vec : Vec<u8> = input.into_bytes();

    for _ in 0..iterations {
        loop {
            if !process_invalid_chars(&mut vec) {
                increment(&mut vec);
            }

            if increasing_straight_3(&vec) && non_overlapping_pairs(&vec) {
                break;
            }
        }
        println!("New Password: {}", String::from_utf8(vec.clone()).unwrap());
    }
}


fn increment(vec : &mut Vec<u8>) {
    for c in vec.iter_mut().rev() {
        if *c == LAST {
            *c = FIRST;
        } else {
            *c += 1;
            break;
        }
    }
}


fn non_overlapping_pairs(arr : &[u8]) -> bool {
    let mut check : u8 = arr[0];
    let mut cnt = 1;
    let mut num_pairs_found = 0;
    let mut pairs = Vec::new(); 

    for c in arr.iter().skip(1) {
        if *c == check {
            cnt += 1;
        } else {
            cnt = 1;
        }
        check = *c;

        if cnt == 2 && !pairs.contains(&check) {
            num_pairs_found += 1;
            pairs.push(check);
        }

        if num_pairs_found >= 2 {
            break;
        }
    }

    num_pairs_found >= 2
}


fn increasing_straight_3(arr : &[u8]) -> bool {
    let mut check : u8 = arr[0];
    let mut cnt = 1;

    for c in arr.iter().skip(1) {
        if (*c-1) == check {
            cnt += 1;
        } else {
            cnt = 1;
        }
        check = *c;

        if cnt >= 3 {
            break;
        }
    }
    cnt >= 3
}


fn process_invalid_chars(vec : &mut Vec<u8>) -> bool {
    let mut bad_idx : i32 = -1;
    for (i, c) in vec.iter_mut().enumerate() {
        if bad_idx >= 0 {
            *c = FIRST;
        } else {
            for bad in &INVALID {
                if *c == *bad {
                    *c = *c + 1;
                    bad_idx = i as i32;
                    break;
                }
            }
        }

    }
    bad_idx >= 0
}


#[cfg(test)]
mod tests {
    use increment;
    use process_invalid_chars; 
    use increasing_straight_3;
    use non_overlapping_pairs;

    #[test]
    fn test_increment() {
        {
            let mut vec = vec!['a' as u8, 'z' as u8];
            increment(&mut vec);
            assert_eq!(vec!['b' as u8, 'a' as u8], vec);
        }

        {
            let mut vec = vec!['t' as u8, 't' as u8];
            increment(&mut vec);
            assert_eq!(vec!['t' as u8, 'u' as u8], vec);
        }
    }

    #[test]
    fn test_invalid_chars() {
        {
            let mut vec = vec!['b' as u8, 'l' as u8, 'a' as u8, 'h' as u8];
            let res : bool = process_invalid_chars(&mut vec);
            assert_eq!(vec!['b' as u8, 'm' as u8, 'a' as u8, 'a' as u8], vec);
            assert_eq!(true, res);
        }

        {
            let mut vec = vec!['b' as u8, 'a' as u8];
            let res : bool = process_invalid_chars(&mut vec);
            assert_eq!(vec!['b' as u8, 'a' as u8], vec);
            assert_eq!(false, res);
        }

    }

    #[test]
    fn test_increase_check() {
        assert_eq!(true, increasing_straight_3(& vec!['a' as u8, 'b' as u8, 'c' as u8]));
        assert_eq!(false, increasing_straight_3(& vec!['a' as u8, 'b' as u8, 'd' as u8]));
    }

    #[test]
    fn test_non_overlapping_pairs() {
        assert_eq!(false, non_overlapping_pairs(& vec!['a' as u8, 'a' as u8, 'a' as u8]));
        assert_eq!(false, non_overlapping_pairs(& vec!['a' as u8, 'a' as u8, 'a' as u8, 'a' as u8]));
        assert_eq!(true, non_overlapping_pairs(& vec!['a' as u8, 'a' as u8, 'b' as u8, 'b' as u8]));
        assert_eq!(false, non_overlapping_pairs(& vec!['a' as u8, 'a' as u8, 'b' as u8, 'a' as u8, 'a' as u8]));
    }

}
