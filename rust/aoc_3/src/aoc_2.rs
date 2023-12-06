use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Round{
    green: u32,
    red: u32,
    blue: u32,

}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn main() {


    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./src/input_2.txt") {

        let mut games: Vec<Game> = Vec::with_capacity(100);
        // Consumes the iterator, returns an (Optional) String
        let mut index= 0;
        for lines_res in lines {
            if let Ok(line) = lines_res {
                let parts: Vec<&str> = line.split(':').collect();
                index = index + 1;
               
                // Get the last part
                if let Some(last_part) = parts.last() {
                    let rounds:Vec<Round> = last_part.split(";").map(|round_str|{
                       let mut round:Round = Round{
                        green: 0,
                        red: 0,
                        blue: 0,
                    };
                    let colors_string = round_str.split(",");
                    for color in colors_string{
                        if color.contains(" red"){
                            round.red = color.trim_end_matches(" red").trim().parse().unwrap_or(0);
                        }
                        else if color.contains(" green"){
                            round.green = color.trim_end_matches(" green").trim().parse().unwrap_or(0);
                        }
                        else if color.contains(" blue"){
                            round.blue = color.trim_end_matches(" blue").trim().parse().unwrap_or(0);
                        }
                    }
                    return round;
                    }).collect();

                        games.push(Game{
                        id: (index).try_into().unwrap_or(0),
                        rounds
                     });

                } else {

                }
            }
        }
        let sum: u32 = games.iter().map(|s| {
            let min_green = s.rounds.iter().map(|r|r.green).max().unwrap_or(0);
            let min_blue = s.rounds.iter().map(|r|r.blue).max().unwrap_or(0);
            let min_red = s.rounds.iter().map(|r|r.red).max().unwrap_or(0);
            return min_blue * min_green * min_red;
        }).sum();
        println!("sum: {}",sum);

    }else{
        println!("Error Reading File!");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}