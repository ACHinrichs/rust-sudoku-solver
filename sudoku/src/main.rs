use ndarray::Array2;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // Reads sudoku

    let file = File::open("input.txt").expect("file not found");
    let lines = &mut BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
        .filter(|x| {
            if x.starts_with("//") {
                //Filter comments
                println!("Found comment in input: {}", &x[2..]);
                false
            } else {
                //Filter empty lines
                x != ""
            }
        })
        .collect::<Vec<String>>();
    let mut field = Array2::<i8>::zeros((9, 9));
    for i in 0..9 {
        for j in 0..9 {
            field[(i, j)] = lines[i]
                .get(j..j + 1)
                .unwrap_or(&"0")
                .parse::<i8>()
                .unwrap_or(0);
        }
    }
    println!("{:?}", field);
    solve(&field, 0, 0);
}

fn solve(field: &Array2<i8>, i: usize, j: usize) {
    let finished = finished(&field);
    let possible = verify(&field);
    if finished {
        //println!("{:?}", &field);
        if possible {
            println!("Found solution {:?}", field);
        }
    }
    if !possible {
        //println!("{:?}", &field);
        return;
    }
    let next_i = (i + 1) % 9;
    let next_j = j + if i == 8 { 1 } else { 0 };

    if i > 8 || j > 8 {
        return;
    }

    if field[(i, j)] != 0 {
        solve(field, next_i, next_j);
    } else {
        // Try every possibility
        for n in 1..10 {
            let mut new_field = field.clone();
            new_field[(i, j)] = n;
            solve(&new_field, next_i, next_j);
        }
    }
}

fn finished(field: &Array2<i8>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if field[(i, j)] == 0 {
                return false;
            }
        }
    }
    return true;
}

fn verify(solution_candidate: &Array2<i8>) -> bool {
    // Returns true if a (incomplete) sudoku contians no errors.
    // Empty sudokus is always verified as true

    // go over lines:
    for i in 0..9 {
        let mut numbers: u32 = 0;
        for j in 0..9 {
            if solution_candidate[(i, j)] == 0 {
                continue;
            }
            let cur_num = 1 << solution_candidate[(i, j)] - 1;
            if cur_num & numbers != 0 {
                // a number is contained more than once
                //println!("abort a ({},{}) {:#b} {:#b}", i, j, cur_num, numbers);
                return false;
            }
            numbers = numbers + cur_num;
        }
    } // the other direction:
    for j in 0..9 {
        let mut numbers: u32 = 0;
        for i in 0..9 {
            if solution_candidate[(i, j)] == 0 {
                continue;
            }
            let cur_num = 1 << solution_candidate[(i, j)] - 1;
            if cur_num & numbers != 0 {
                // a number is contained more than once
                //println!("abort b");
                return false;
            }
            numbers = numbers + cur_num;
        }
    }

    // The blocks

    for block_i in 0..3 {
        for block_j in 0..3 {
            let mut numbers: u32 = 0;
            for i in 0..3 {
                for j in 0..3 {
                    if solution_candidate[((block_i * 3) + i, (block_j * 3) + j)] == 0 {
                        continue;
                    }
                    let cur_num =
                        1 << solution_candidate[((block_i * 3) + i, (block_j * 3) + j)] - 1;
                    if cur_num & numbers != 0 {
                        // a number is contained more than once
                        //println!("abort c");
                        return false;
                    }
                    numbers = numbers + cur_num;
                }
            }
        }
    }

    return true;
}
