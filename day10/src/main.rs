use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::SplitWhitespace;

fn get_x_number(mut instruction: SplitWhitespace) -> i32 {
    if let Some(second_instruction) = instruction.next() {
        if let Ok(parse_int) = second_instruction.parse::<i32>() {
            return parse_int;
        } else {
            panic!("Not able to parse integer")
        }
    }
    panic!("Invalid input")
}

fn process_cycle(cycle_count: i32, x: i32) -> i32 {
    if cycle_count >= 20 && (cycle_count - 20) % 40 == 0 {
        // println!("Cycle {}", cycle_count);
        // println!("X {}", x);
        let current_strength: i32 = cycle_count * x;
        // println!("Signal Strength {}", current_strength);
        return current_strength;
    }
    0
}

fn process_sprite(cycle_count: i32, x: i32) -> String {
    let mut position: i32 = cycle_count - 1;
    if cycle_count > 40 {
        position = cycle_count % 40 - 1
    }
    if position <= x + 1 && position >= x - 1 {
        return "#".to_string();
    } else {
        return ".".to_string();
    }
}

fn read_and_draw(filename: &str) -> io::Result<String> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut x: i32 = 1;
    let mut cycle_count: i32 = 0;
    let mut crt: String = String::new();

    // Iterate through each line
    for line in reader.lines() {
        let instruction_string: String = line.unwrap();
        let mut instruction = instruction_string.split_whitespace();
        let command = instruction.next().unwrap();

        let mut _pixel: String = String::new();

        if command == "noop" {
            cycle_count += 1;
            _pixel = process_sprite(cycle_count, x);
            crt = format!("{}{}", crt, _pixel);
            continue;
        }

        // First cycle of addx done
        cycle_count += 1;
        let x_number = get_x_number(instruction);
        _pixel = process_sprite(cycle_count, x);
        crt = format!("{}{}", crt, _pixel);

        // Second cycle of addx done
        cycle_count += 1;
        _pixel = process_sprite(cycle_count, x);
        crt = format!("{}{}", crt, _pixel);

        x += x_number;
    }

    Ok(crt)
}

fn read_and_accumulate_strength(filename: &str) -> io::Result<i32> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum_signal_strength: i32 = 0;
    let mut x: i32 = 1;
    let mut cycle_count: i32 = 0;

    // Iterate through each line
    for line in reader.lines() {
        let instruction_string: String = line.unwrap();
        let mut instruction = instruction_string.split_whitespace();
        let command = instruction.next().unwrap();
        // println!("Command {} Cycle {}", instruction_string, cycle_count);

        let mut _strength_to_add: i32 = 0;

        if command == "noop" {
            cycle_count += 1;
            _strength_to_add = process_cycle(cycle_count, x);
            sum_signal_strength += _strength_to_add;
            continue;
        }

        // First cycle of addx done
        cycle_count += 1;
        let x_number = get_x_number(instruction);

        _strength_to_add = process_cycle(cycle_count, x);
        sum_signal_strength += _strength_to_add;

        // Second cycle of addx done
        cycle_count += 1;

        _strength_to_add = process_cycle(cycle_count, x);
        sum_signal_strength += _strength_to_add;
        x += x_number;
    }

    Ok(sum_signal_strength)
}

fn main() {
    if let Ok(signal_strength) = read_and_accumulate_strength("./day10.txt") {
        println!("Signal Strength: {}", signal_strength);
    } else {
        eprintln!("Error reading file");
    }
    if let Ok(drawing) = read_and_draw("./day10.txt") {
        println!("{}", &drawing[0..39]);
        println!("{}", &drawing[40..79]);
        println!("{}", &drawing[80..119]);
        println!("{}", &drawing[120..159]);
        println!("{}", &drawing[160..199]);
        println!("{}", &drawing[200..239]);
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
    fn test_program() {
        let content = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 13140;
        let result = read_and_accumulate_strength(temp_path).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_drawing() {
        let content = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: String = "##..##..##..##..##..##..##..##..##..##..".to_string();
        let result: String = read_and_draw(temp_path).unwrap();
        let sliced_result = &result[0..40];
        assert_eq!(sliced_result, expected_result);
    }
}
