extern crate clap;
use clap::App;
use std::cmp;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::str::FromStr;

fn main() {
    let matches = App::new("day1")
        .version("v1.0")
        .author("Andrew Rink <andrewrink@gmail.com>")
        .args_from_usage("<FILE> 'Filename containing list of present dimensions'")
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();
    let mut file = match File::open(filename) {
        Err(why) => panic!("Couldn't open {}: {}", filename, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't open {}: {}", filename, Error::description(&why)),
        Ok(_) => println!("Read file {}", filename),
    }
    
    let vec: Vec<&str> = s.split('\n').collect();

    let (paper, ribbon) = total_wrapping_paper_and_ribbon(vec);
    println!("{} square feet of paper, {} feet of ribbon", paper, ribbon);
}

fn total_wrapping_paper_and_ribbon(vec : Vec<&str>) -> (u32, u32) {
    let mut paper = 0;
    let mut ribbon = 0;
    for v in vec {
        if v.len() < 5 {
            continue;
        }

        let dim: Vec<u32> = v.split('x').map(|n| u32::from_str(n).unwrap()).collect();
        assert_eq!(3, dim.len());
        paper += surface_area(dim[0], dim[1], dim[2]) + extra_paper(dim[0], dim[1], dim[2]);
        ribbon += volume(dim[0], dim[1], dim[2]) + smallest_face_perimeter(dim[0], dim[1], dim[2]);
    }
    (paper, ribbon)
}

fn smallest_face_perimeter(l : u32, w : u32, h : u32) -> u32 {
    let mut faces = vec![l+l, w+w, h+h];
    faces.sort();
    faces[0] + faces[1]
}

fn volume(l : u32, w : u32, h : u32) -> u32 {
    l * w * h
}
    
fn surface_area(l : u32, w : u32, h : u32) -> u32 {
    2*l* w + 2*w* h + 2*h*l
}

fn extra_paper(l : u32, w : u32, h : u32) -> u32 {
    cmp::min(l*w, cmp::min(l*h, w*h))        
}

#[cfg(test)]
mod tests {
    use surface_area;
    use extra_paper;
    use volume;
    use smallest_face_perimeter;

    #[test]
    fn area() {
        assert_eq!(52, surface_area(2,3,4));
        assert_eq!(42, surface_area(1,1,10));
    }

    #[test]
    fn extra() {
        assert_eq!(6, extra_paper(2, 3, 4));
        assert_eq!(1, extra_paper(1, 1, 10));
    }

    #[test]
    fn ribbon_bow() {
        assert_eq!(24, volume(2,3,4));
        assert_eq!(10, volume(1,1,10));
    }

    #[test]
    fn ribbon_wrap() {
        assert_eq!(10, smallest_face_perimeter(2,3,4));
        assert_eq!(4, smallest_face_perimeter(1,1,10));
    }
}
