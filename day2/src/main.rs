use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    r: u32,
    g: u32,
    b: u32,
}
// 1371 is not the right answer

impl FromStr for Game {
    type Err = std::num::ParseIntError;

    /// Parses a line of game result into Game
    /// Game 86: 14 red, 1 green, 3 blue; 3 blue; 4 green, 8 red, 2 blue; 10 red, 2 green
    fn from_str(game_line: &str) -> Result<Self, Self::Err> {
        // store only max number of dice over games for each line
        // get id
        let v: Vec<&str> = game_line.split(":").collect(); 
        let v2: Vec<&str> = v[0].split(" ").collect();
        let game_id = v2[1].parse::<u32>().unwrap();
        let games:Vec<&str> = v[1].split(";").collect();
        let mut max_colors: [u32; 3] = [0,0,0];
        for game in games {
        
            let colors:Vec<&str> = game.split(",").collect();
            for color in colors {
                let values:Vec<&str> = color.split(" ").collect();
                let v = values[1].parse::<u32>().unwrap();
                match values[2] {
                    "red" => assign_if_larger(&mut max_colors, 0, v),
                    "green" => assign_if_larger(&mut max_colors, 1, v),
                    "blue" => assign_if_larger(&mut max_colors, 2, v),
                    _ => println!("bad"),
                }
            }
        }
        Ok(Game {
            id:game_id,
            r:max_colors[0],
            g:max_colors[1],
            b:max_colors[2],
        })
    }

}
fn assign_if_larger(max_colors: &mut [u32], idx: usize,v: u32) {
    if v > max_colors[idx] {
        max_colors[idx] = v;
    };
}

fn is_valid(g: &Game) -> bool {
//    let max_red = 20;
//    let max_green = 13;
//    let max_blue = 14;
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    return g.r > max_red || g.g > max_green || g.b > max_blue;
}

fn power(g: &Game) -> u32 {
    return g.r * g.g * g.b;
}


fn main() {
    let mut games: Vec<Game> = Vec::new();
    if let Ok(lines) = read_lines("./day-2.txt") {
        for line in lines {
            if let Ok(current_line) = line {
                games.push(Game::from_str(&current_line).unwrap());
            }
        }
    }
    // check max values to 
//    let mut s = 0u32;
//    for game in games {
//       if !is_valid(&game) {
//           println!("{:?}", game);
//           s = s+game.id;
//       } 
//    }
//    println!("Part 1: {:?}", s);
//
    let mut s2 = 0u32;
    for game in games {
       println!("{:?}", game);
       s2 = s2+power(&game);
    }
    println!("Part 1: {:?}", s2);

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
    fn test_oneone_get_coord() {
    let t = String::from("Game 86: 14 red, 1 green, 3 blue; 3 blue; 4 green, 8 red, 2 blue; 10 red, 2 green");
    assert_eq!(Ok(Game{id:86,
        r:14,
        g:4,
        b:3
    }), 
    Game::from_str(&t));
    }
}
