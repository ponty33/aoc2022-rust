use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_and_accumulate_numbers(filename: &str) -> io::Result<i32> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum: i32 = 0; // For storing the list of sums

    // Iterate through each line
    for line in reader.lines() {
        if let Ok(line) = line {
            let patterns: Vec<&str> = line.split(' ').collect();
            let opponent: Option<&&str> = patterns.get(0);
            let my: Option<&&str> = patterns.get(1);

            if let Some(&value) = my {
                if value == "X" {
                    sum += 1;
                } else if value == "Y" {
                    sum += 2;
                } else if value == "Z" {
                    sum += 3;
                }
            }

            if let Some(&opponent_value) = opponent {
                if opponent_value == "A" {
                    if let Some(&my_value) = my {
                        if my_value == "X" {
                            sum += 3;
                        } else if my_value == "Y" {
                            sum += 6;
                        }
                    }
                } else if opponent_value == "B" {
                    if let Some(&my_value) = my {
                        if my_value == "Y" {
                            sum += 3;
                        } else if my_value == "Z" {
                            sum += 6;
                        }
                    }
                } else if opponent_value == "C" {
                    if let Some(&my_value) = my {
                        if my_value == "Z" {
                            sum += 3;
                        } else if my_value == "X" {
                            sum += 6;
                        }
                    }
                }
            }
        }
    }

    Ok(sum)
}

fn read_and_accumulate_numbers_part_2(filename: &str) -> io::Result<i32> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum: i32 = 0; // For storing the list of sums

    // Iterate through each line
    for line in reader.lines() {
        if let Ok(line) = line {
            let patterns: Vec<&str> = line.split(' ').collect();
            let opponent: Option<&&str> = patterns.get(0);
            let my: Option<&&str> = patterns.get(1);

            if let Some(&opponent_value) = opponent {
                if opponent_value == "A" {
                    // Rock
                    if let Some(&my_value) = my {
                        if my_value == "X" {
                            // Lose+0: Scissor+3
                            sum += 3;
                        } else if my_value == "Y" {
                            // Draw+3: Rock+1
                            sum += 4;
                        } else if my_value == "Z" {
                            // Win+6: Paper+2
                            sum += 8
                        }
                    }
                } else if opponent_value == "B" {
                    // Paper
                    if let Some(&my_value) = my {
                        if my_value == "X" {
                            // Lose+0: Rock+1
                            sum += 1;
                        } else if my_value == "Y" {
                            // Draw+3: Paper+2
                            sum += 5;
                        } else if my_value == "Z" {
                            // Win+6: Scissor+3
                            sum += 9
                        }
                    }
                } else if opponent_value == "C" {
                    // Scissor
                    if let Some(&my_value) = my {
                        if my_value == "X" {
                            // Lose+0: Paper+2
                            sum += 2;
                        } else if my_value == "Y" {
                            // Draw+3: Scissor+3
                            sum += 6;
                        } else if my_value == "Z" {
                            // Win+6: Rock+1
                            sum += 7
                        }
                    }
                }
            }
        }
    }

    Ok(sum)
}

fn main() {
    if let Ok(sum) = read_and_accumulate_numbers("./day2.txt") {
        println!("Total: {}", sum);
    } else {
        eprintln!("Error reading file");
    }

    if let Ok(sum) = read_and_accumulate_numbers_part_2("./day2.txt") {
        println!("Total (Revised): {}", sum);
    } else {
        eprintln!("Error reading file");
    }
}
