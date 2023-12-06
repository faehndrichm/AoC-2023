use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./src/input_3.txt") {
        println!("{:?}", lines);
        let matrix: Vec<Vec<char>> = lines
            .filter_map(Result::ok)
            .map(|line| line.chars().collect())
            .collect();

        let mut count = 0;
        let mut gear_count = 0;
        let mut partnumber = String::new();
        let mut read_num = false;
        let mut is_part_num = false;
        let mut gear_map: HashMap<(usize, usize), i32> = HashMap::new();
        let mut gear_index = (0, 0);
        for (row_index, row) in matrix.iter().enumerate() {
            for (col_index, &char) in row.iter().enumerate() {
                // I
                if char.is_numeric() {
                    read_num = true;
                    partnumber.push(char);

                    get_adj_indices(140, 140, row_index, col_index)
                        .iter()
                        .filter(|&(r, c)| matrix[*r][*c] == '*')
                        .for_each(|&(r, c)| {
                            gear_index = (r, c);
                        });

                    if get_adj_indices(140, 140, row_index, col_index)
                        .iter()
                        .any(|&(r, c)| check_is_symbol(matrix[r][c]))
                    {
                        is_part_num = true;
                    }
                    // check number adjacent
                } else if read_num {
                    // finished with reading number
                    let mut partnr_value = 0;
                    if is_part_num {
                        // this is a part number add
                        partnr_value = partnumber.parse().unwrap_or(0);
                        count += partnr_value;
                    }
                    if gear_index.0 != 0 && gear_index.1 != 0 {
                        // match contacts.get(&"Daniel") {
                        //     Some(&number) => println!("Calling Daniel: {}", call(number)),
                        //     _ => println!("Don't have Daniel's number."),
                        // }
                        if let Some(&partnr) = gear_map.get(&gear_index) {
                            gear_count += partnr_value * partnr;
                        } else {
                            gear_map.insert(gear_index, partnr_value);
                        }
                    }
                    gear_index = (0, 0);
                    partnumber = String::new();
                    is_part_num = false;
                    read_num = false;
                }
                // II
                if char == '*' {}
            }
        }
        println!("count: {}", count);
        println!("count_gear: {}", gear_count);
    } else {
        println!("No File found");
    }
}

fn check_is_symbol(char: char) -> bool {
    return !char.is_alphanumeric() && char != '.';
}

fn get_adj_indices(max_row: u32, max_col: u32, row: usize, col: usize) -> Vec<(usize, usize)> {
    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    return directions
        .iter()
        .map(|(x, y)| (row as i32 + x, col as i32 + y))
        .filter(|&(r, c)| r >= 0 && c >= 0 && r < max_row as i32 && c < max_col as i32)
        .map(|(r, c)| (r as usize, c as usize))
        .collect();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
