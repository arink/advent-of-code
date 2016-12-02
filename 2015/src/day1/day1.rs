extern crate clap;
use clap::App;

struct Parenthesis {
	left : i32,
	right : i32,
    negative_index : i32,
}

impl Parenthesis {
	fn new (slice: &str) -> Parenthesis {
        let mut idx : i32 = -1;
        let mut lp : i32 = 0;
        let mut rp : i32 = 0;


        for (i, c) in slice.chars().enumerate() {
            if c == '(' {
                lp += 1;
            } else if c == ')' {
                rp += 1;
            }

            if idx < 0 && lp < rp {
                idx = i as i32;
            }
        }
        
		Parenthesis {
			left : lp,
			right : rp,
            negative_index : idx,
		}
	}
}


fn main() {
    let matches = App::new("day1")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<INPUT> 'Sets the input string to use'")
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    println!("{}", apartment_floor(input));
    println!("{}", below_main_level(input));
}

fn apartment_floor(slice : &str) -> i32 {
    let p = Parenthesis::new(slice);
    p.left - p.right
}

fn below_main_level(slice : &str) -> i32 {
    let p = Parenthesis::new(slice);
    p.negative_index + 1  // Character 0 considered position 1
}

#[cfg(test)]
mod tests {
    use apartment_floor;
    use below_main_level;
    use Parenthesis;

    #[test]
    fn parenthesis_count() {
        assert_eq!(4, Parenthesis::new("((((").left);
        assert_eq!(0, Parenthesis::new("((((").right);

        assert_eq!(0, Parenthesis::new("))))").left);
        assert_eq!(4, Parenthesis::new("))))").right);
    }

    #[test]
    fn negative_check() {
        assert_eq!(1, below_main_level(")"));
        assert_eq!(5, below_main_level("()())"));
    }

    #[test]
    fn floor_check() {
        assert_eq!(0, apartment_floor("(())"));
        assert_eq!(0, apartment_floor("()()"));

        assert_eq!(3, apartment_floor("((("));
        assert_eq!(3, apartment_floor("(()(()("));
        assert_eq!(3, apartment_floor("))((((("));

        assert_eq!(-1, apartment_floor("))("));
        assert_eq!(-1, apartment_floor("())"));

        assert_eq!(-3, apartment_floor(")))"));
    }
}
