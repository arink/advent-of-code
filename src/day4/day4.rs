extern crate crypto;
extern crate clap;
use clap::App;
use crypto::md5::Md5;
use crypto::digest::Digest;


fn main() {
    let matches = App::new("day4")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<KEY> 'Secret key for MD5 hash'")
        .get_matches();

    let key = matches.value_of("KEY").unwrap();
    println!("For key {}, found {}", key, find_number_leading_zeroes(key, 5));
    println!("For key {}, found {}", key, find_number_leading_zeroes(key, 6));
}


fn find_number_leading_zeroes(key : &str, num_zeroes : usize) -> u64 {
   let mut md5 = Md5::new();
   let mut res = 0;
   let target_string : String = (vec!['0'; num_zeroes]).into_iter().collect();
   for i in 0..std::u64::MAX {
       let mut tst = String::from(key);
       tst.push_str(&i.to_string()); 

       md5.input_str(&tst);
       if md5.result_str().starts_with(&target_string) {
           res = i;
           break;
       }
       md5.reset();
   }
   res
}


#[cfg(test)]
mod tests {
    use find_number_leading_zeroes;
    
    #[test]
    fn number_check() {
        assert_eq!(609043, find_number_leading_zeroes("abcdef", 5));
        assert_eq!(1048970, find_number_leading_zeroes("pqrstuv", 5));
    }
}
