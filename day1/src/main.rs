use std::char;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;




// Read input file line by line
// - take first&last numeric of each line into a dynamic array
// Add numbers in array
fn main() {
    let mut elf_coordinate = 0u32;
    println!("{}", elf_coordinate);
    if let Ok(lines) = read_lines("./day-1.txt") {
        for line in lines {
            if let Ok(elf_code) = line {
                let current_elf_code = get_coord(&elf_code);
                //println!("{:?}", current_elf_code);
                elf_coordinate = elf_coordinate+current_elf_code;
            }
        }
    }
    println!("{}", elf_coordinate);
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
    use super::*;

    #[test]
    fn test_get_coord() {
    let t = String::from("1abc2");
    assert_eq!(12, get_coord(&t));

    let t2 = String::from("");
    assert_eq!(0, get_coord(&t2));

    let t3 = String::from("1abc");
    assert_eq!(11, get_coord(&t3));
    }


}
