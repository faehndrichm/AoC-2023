use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Pipe {
    start: bool,
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

impl Default for Pipe {
    fn default() -> Self {
        Pipe {
            start: false,
            left: false,
            right: false,
            top: false,
            bottom: false,
        }
    }
}

fn pipes_match(cur: &Pipe, next: &Pipe, cords: (i32, i32)) -> bool {
    match cords {
        (0, -1) => cur.left && next.right,
        (-1, 0) => cur.top && next.bottom,
        (0, 1) => cur.right && next.left,
        (1, 0) => cur.bottom && next.top,
        _ => false,
    }
}

fn main() {
    let start = Instant::now();

    let lines = match read_lines("./src/input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut start_pos = (0, 0);
    let mut pipes = Vec::new();
    for (row, line_res) in lines.enumerate() {
        let line = match line_res {
            Ok(line) => line,
            Err(error) => panic!("Error reading line :{:?}", error),
        };
        let sequence: Vec<Pipe> = format!(".{}.", line.chars().collect::<String>())
            .chars()
            .enumerate()
            .filter_map(|(col, f)| match f {
                '|' => Some(Pipe {
                    top: true,
                    bottom: true,
                    ..Default::default()
                }),
                '-' => Some(Pipe {
                    left: true,
                    right: true,
                    ..Default::default()
                }),
                'L' => Some(Pipe {
                    right: true,
                    top: true,
                    ..Default::default()
                }),
                'J' => Some(Pipe {
                    left: true,
                    top: true,
                    ..Default::default()
                }),
                '7' => Some(Pipe {
                    left: true,
                    bottom: true,
                    ..Default::default()
                }),
                'F' => Some(Pipe {
                    right: true,
                    bottom: true,
                    ..Default::default()
                }),
                '.' => Some(Pipe {
                    ..Default::default()
                }),
                'S' => {
                    start_pos = (row + 1, col);
                    Some(Pipe {
                        left: true,
                        right: true,
                        top: false,    // hack: only for my input
                        bottom: false, // hack: only for my input
                        start: true,
                    })
                }
                _ => None,
            })
            .collect();
        pipes.push(sequence);
    }

    let row_length = pipes.len();
    let col_length = pipes.first().unwrap().len();

    println!("row-length {}, col-length {}", row_length, col_length);

    let base_pipe = Pipe {
        ..Default::default()
    };
    let base_pipe_2 = Pipe {
        ..Default::default()
    };
    let padding_start: Vec<Pipe> = vec![base_pipe; col_length];
    let padding_end: Vec<Pipe> = vec![base_pipe_2; col_length];

    pipes = vec![vec![padding_start], pipes, vec![padding_end]].concat();

    let mut has_next = true;
    let mut loop_index = 0;
    let mut res1 = 0;

    let mut current_pos = start_pos;

    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();

    let dirs: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    while has_next {
        has_next = false;
        for cords in dirs {
            let current = &pipes[current_pos.0][current_pos.1];
            let next_0 = (current_pos.0 as i32 + cords.0) as usize;
            let next_1 = (current_pos.1 as i32 + cords.1) as usize;
            let next = &pipes[next_0][next_1];

            if visited.contains_key(&(next_0, next_1)) {
                // do not go back
                continue;
            }

            if next.start {
                // back at begin
                has_next = false;
                break;
            }

            if pipes_match(current, next, cords) {
                has_next = true;
                visited.insert(current_pos, true);
                current_pos.0 = next_0;
                current_pos.1 = next_1;

                break;
            }
        }
        loop_index += 1;
    }
    res1 = loop_index / 2;

    let mut res2 = 0;

    // // create partiotions

    // let mut partitions = Vec::new();
    // println!("pipe size {}", visited.len());

    // // first partition is the pipe
    // partitions.push(visited);

    // for row in 1..row_length {
    //     for col in 1..col_length {
    //         let mut new_partition: HashMap<(usize, usize), bool> = HashMap::new();
    //         let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    //         deque.push_back((row, col)); // add starting point

    //         while let Some(cur_pos) = deque.pop_front() {
    //             let is_partitioned = partitions.iter().any(|p| p.contains_key(&cur_pos))
    //                 || new_partition.contains_key(&cur_pos);
    //             if is_partitioned {
    //                 continue;
    //             }
    //             new_partition.insert(cur_pos, true);

    //             for cords in dirs {
    //                 let next_0 = (cur_pos.0 as i32 + cords.0) as usize;
    //                 let next_1 = (cur_pos.1 as i32 + cords.1) as usize;

    //                 if next_0 > row_length || next_1 > col_length {
    //                     // skip out of bounds
    //                     continue;
    //                 }

    //                 //let current = &pipes[cur_pos.0][cur_pos.1];
    //                 //let next = &pipes[next_0][next_1];
    //                 deque.push_back((next_0, next_1));
    //             }
    //         }
    //         // push partiton, if new elements dsicovered
    //         if !new_partition.is_empty() {
    //             partitions.push(new_partition);
    //         }
    //     }
    // }

    // for (i, p) in partitions.iter().enumerate() {
    //     println!("nr {}", i);
    //     for x in p {
    //         println!("{:?}", x);
    //     }
    // }

    let mut sum = 0;
    for row in 1..row_length {
        let mut inside = false;
        let mut open_top = false;
        let mut open_bottom = false;
        for col in 1..col_length {
            let pipe_cur = &pipes[row][col];

            if visited.contains_key(&(row, col)) {
                if pipe_cur.top && pipe_cur.bottom {
                    // horizontal plane -> go in/out
                    inside = !inside;
                } else if pipe_cur.left && pipe_cur.right {
                    // ok
                } else if pipe_cur.top {
                    if open_bottom {
                        inside = !inside;
                        open_bottom = false;
                    } else {
                        open_top = !open_top;
                    }
                } else if pipe_cur.bottom {
                    if open_top {
                        inside = !inside;
                        open_top = false;
                    } else {
                        open_bottom = !open_bottom;
                    }
                }
                continue;
            }
            if inside {
                println!("{},{}", row, col);
                sum += 1;
            }
        }
    }

    let duration = start.elapsed();
    println!("Result1: {:?}", res1);
    println!("Result2: {:?}", sum);
    println!("Time elapsed: {:?}", duration);
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
