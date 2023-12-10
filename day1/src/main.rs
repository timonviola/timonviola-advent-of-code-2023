use std::char;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// Read input file line by line
// - take first&last numeric of each line into a dynamic array
// Add numbers in array
fn main() {
    let mut elf_coordinate = 0u32;
    let mut lno = 0u32;
    println!("{}", elf_coordinate);
    if let Ok(lines) = read_lines("./day-1.txt") {
        for line in lines {
            if let Ok(elf_code) = line {
                let current_elf_code = get_coord2(&elf_code);
                //println!("{:?}", current_elf_code);
                println!("{:?}@{:?} - {:?} ",lno,elf_coordinate, current_elf_code);
                elf_coordinate = elf_coordinate+current_elf_code;
                lno += 1;
            }
        }
    }
    println!("{}", elf_coordinate);
}
fn get_coord2(elf_code: &String) -> u32 {
    let mut v = Vec::new();
    let pattern = &["one","two","three","four","five","six","seven","eight","nine" ];
    let replace = &["1","2","3", "4","5","6","7","8","9"];
    for (i,c) in elf_code.chars().enumerate() {
        match c.is_numeric() {
            false => (),
            true => {
                v.push(c);
                continue;
            },
        };
        for (j, pat) in pattern.into_iter().enumerate() {
            match elf_code[i..].starts_with(pat) {
                false => (),
                true => println!("{:?}", v.push(String::from(replace[j]).chars().next().unwrap()))
            }
        }
    }
    let s: String = String::from_iter(v);
    let v: Vec<&str> = s
        .matches(char::is_numeric)
        .collect();
    match v.len() {
        0 => 0u32,
        1 => v[0].parse::<u32>().unwrap()*10u32 + v[0].parse::<u32>().unwrap(),
        _ => v[0].parse::<u32>().unwrap()*10u32 + v.last().unwrap().parse::<u32>().unwrap(),
    }
}

/// get first and last digit and put them together as an integer value
/// if no digit -> 0
/// if 1 digit -> use twice that
/// if >1 digit -> trivial
fn get_coord(elf_code: &String) -> u32 {
    let v: Vec<&str> = elf_code
        .matches(char::is_numeric)
        .collect();
    match v.len() {
        0 => 0u32,
        1 => v[0].parse::<u32>().unwrap()*10u32 + v[0].parse::<u32>().unwrap(),
        _ => v[0].parse::<u32>().unwrap()*10u32 + v.last().unwrap().parse::<u32>().unwrap(),
    }
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use aho_corasick::AhoCorasick;

    use super::*;

    #[test]
    fn test_oneone_get_coord() {
    let t = String::from("one7one");
    assert_eq!(11, get_coord(&t));
    }

    #[test]
    fn test_new_get_coord() {
    let t = String::from("oneight");
    assert_eq!(18, get_coord(&t));
    }

    #[test]
    fn test_get_coord() {
    let t = String::from("1abc2");
    assert_eq!(12, get_coord(&t));

    let t2 = String::from("");
    assert_eq!(0, get_coord(&t2));

    let t3 = String::from("1abc");
    assert_eq!(11, get_coord(&t3));
    
    let t = String::from("two1nine");
    assert_eq!(29, get_coord(&t));
    let t = String::from("eightwothree");
    assert_eq!(83, get_coord(&t));
    let t = String::from("abcone2threexyz");
    assert_eq!(13, get_coord(&t));
    let t = String::from("xtwone3four");
    assert_eq!(24, get_coord(&t));
    let t = String::from("zoneiht234");
    assert_eq!(14, get_coord(&t));
    let t = String::from("7pqrstsixteen");
    assert_eq!(76, get_coord(&t));
    }

    #[test]
    fn test_get_alfanum_coord() {
    let t = String::from("one1itwoabc2four");
    assert_eq!(14, get_coord(&t));

    let t = String::from("ssdxrgjncdxscf8threetfcgknm9three");
    assert_eq!(83, get_coord(&t));
    }

}
