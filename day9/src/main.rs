use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::SplitWhitespace;

fn calculate_tail_position(head_position: [i32; 2], mut tail_position: [i32; 2]) -> [i32; 2] {
    let horizontal_apart: i32 = (head_position[0] - tail_position[0]).clone();
    let vertical_apart: i32 = (head_position[1] - tail_position[1]).clone();
    if horizontal_apart.abs() <= 1 && vertical_apart.abs() <= 1 {
        return tail_position;
    }
    if (vertical_apart == 2 && horizontal_apart == 1)
        || (vertical_apart == 1 && horizontal_apart == 2)
    {
        tail_position[0] += 1;
        tail_position[1] += 1;
    } else if (vertical_apart == -2 && horizontal_apart == 1)
        || (vertical_apart == -1 && horizontal_apart == 2)
    {
        tail_position[0] += 1;
        tail_position[1] += -1;
    } else if (vertical_apart == 1 && horizontal_apart == -2)
        || (vertical_apart == 2 && horizontal_apart == -1)
    {
        tail_position[0] += -1;
        tail_position[1] += 1;
    } else if (vertical_apart == -1 && horizontal_apart == -2)
        || (vertical_apart == -2 && horizontal_apart == -1)
    {
        tail_position[0] += -1;
        tail_position[1] += -1;
    } else if horizontal_apart.abs() == 2 && vertical_apart == 0 {
        tail_position[0] += horizontal_apart / 2;
    } else if horizontal_apart == 0 && vertical_apart.abs() == 2 {
        tail_position[1] += vertical_apart / 2;
    } else if vertical_apart.abs() == 2 && horizontal_apart.abs() == 2 {
        tail_position[0] += horizontal_apart / 2;
        tail_position[1] += vertical_apart / 2;
    }
    tail_position
}

fn get_movement_number(mut command: SplitWhitespace) -> i32 {
    if let Some(second_command) = command.next() {
        if let Ok(parse_int) = second_command.parse::<i32>() {
            return parse_int;
        } else {
            panic!("Not able to parse integer")
        }
    }
    panic!("Invalid input")
}

fn count_long_tail_trail(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut trail = HashSet::new();
    let mut head_position = [0, 0];
    let mut body1_position = [0, 0];
    let mut body2_position = [0, 0];
    let mut body3_position = [0, 0];
    let mut body4_position = [0, 0];
    let mut body5_position = [0, 0];
    let mut body6_position = [0, 0];
    let mut body7_position = [0, 0];
    let mut body8_position = [0, 0];
    let mut tail_position = [0, 0];

    for line in reader.lines() {
        let movement = line.unwrap();
        let mut command = movement.split_whitespace();
        let direction = command.next().unwrap();
        let movement_amount = get_movement_number(command) + 1;
        if direction == "R" {
            for _i in 1..movement_amount {
                head_position[0] += 1;
                body1_position = calculate_tail_position(head_position, body1_position);
                body2_position = calculate_tail_position(body1_position, body2_position);
                body3_position = calculate_tail_position(body2_position, body3_position);
                body4_position = calculate_tail_position(body3_position, body4_position);
                body5_position = calculate_tail_position(body4_position, body5_position);
                body6_position = calculate_tail_position(body5_position, body6_position);
                body7_position = calculate_tail_position(body6_position, body7_position);
                body8_position = calculate_tail_position(body7_position, body8_position);
                tail_position = calculate_tail_position(body8_position, tail_position);

                trail.insert(tail_position.clone());
            }
        } else if movement.starts_with('L') {
            for _i in 1..movement_amount {
                head_position[0] += -1;
                body1_position = calculate_tail_position(head_position, body1_position);
                body2_position = calculate_tail_position(body1_position, body2_position);
                body3_position = calculate_tail_position(body2_position, body3_position);
                body4_position = calculate_tail_position(body3_position, body4_position);
                body5_position = calculate_tail_position(body4_position, body5_position);
                body6_position = calculate_tail_position(body5_position, body6_position);
                body7_position = calculate_tail_position(body6_position, body7_position);
                body8_position = calculate_tail_position(body7_position, body8_position);
                tail_position = calculate_tail_position(body8_position, tail_position);
                trail.insert(tail_position.clone());
            }
        } else if movement.starts_with('U') {
            for _i in 1..movement_amount {
                head_position[1] += 1;
                body1_position = calculate_tail_position(head_position, body1_position);
                body2_position = calculate_tail_position(body1_position, body2_position);
                body3_position = calculate_tail_position(body2_position, body3_position);
                body4_position = calculate_tail_position(body3_position, body4_position);
                body5_position = calculate_tail_position(body4_position, body5_position);
                body6_position = calculate_tail_position(body5_position, body6_position);
                body7_position = calculate_tail_position(body6_position, body7_position);
                body8_position = calculate_tail_position(body7_position, body8_position);
                tail_position = calculate_tail_position(body8_position, tail_position);
                trail.insert(tail_position.clone());
            }
        } else if movement.starts_with("D") {
            for _i in 1..movement_amount {
                head_position[1] += -1;
                body1_position = calculate_tail_position(head_position, body1_position);
                body2_position = calculate_tail_position(body1_position, body2_position);
                body3_position = calculate_tail_position(body2_position, body3_position);
                body4_position = calculate_tail_position(body3_position, body4_position);
                body5_position = calculate_tail_position(body4_position, body5_position);
                body6_position = calculate_tail_position(body5_position, body6_position);
                body7_position = calculate_tail_position(body6_position, body7_position);
                body8_position = calculate_tail_position(body7_position, body8_position);
                tail_position = calculate_tail_position(body8_position, tail_position);
                trail.insert(tail_position.clone());
            }
        } else {
            panic!("Not a correct movement");
        }
    }
    Ok(trail.len() as i32)
}

fn count_tail_trail(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut trail = HashSet::new();
    let mut head_position = [0, 0];
    let mut tail_position = [0, 0];

    for line in reader.lines() {
        let movement = line.unwrap();
        let mut command = movement.split_whitespace();
        let direction = command.next().unwrap();
        let movement_amount = get_movement_number(command) + 1;
        if direction == "R" {
            for _i in 1..movement_amount {
                head_position[0] += 1;
                tail_position = calculate_tail_position(head_position, tail_position);
                trail.insert(tail_position.clone());
            }
        } else if movement.starts_with('L') {
            for _i in 1..movement_amount {
                head_position[0] += -1;
                tail_position = calculate_tail_position(head_position, tail_position);
                trail.insert(tail_position.clone());
            }
        } else if movement.starts_with('U') {
            for _i in 1..movement_amount {
                head_position[1] += 1;
                tail_position = calculate_tail_position(head_position, tail_position);
                trail.insert(tail_position.clone());
            }
        } else if movement.starts_with("D") {
            for _i in 1..movement_amount {
                head_position[1] += -1;
                tail_position = calculate_tail_position(head_position, tail_position);
                trail.insert(tail_position.clone());
            }
        } else {
            panic!("Not a correct movement");
        }
    }
    Ok(trail.len() as i32)
}

fn main() {
    if let Ok(tail_trail) = count_tail_trail("./day9.txt") {
        println!("Tail visited: {}", tail_trail);
    } else {
        eprintln!("Error reading file");
    }

    if let Ok(tail_trail) = count_long_tail_trail("./day9.txt") {
        println!("Long tail visited: {}", tail_trail);
    } else {
        eprintln!("Error reading file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_find_trail() {
        let content = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 13;
        let result = count_tail_trail(temp_path).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_find_long_trail() {
        let content = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 36;
        let result = count_long_tail_trail(temp_path).unwrap();
        assert_eq!(result, expected_result);
    }
}
