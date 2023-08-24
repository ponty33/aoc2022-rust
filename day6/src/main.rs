use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn check_repetitive(packets: &Vec<char>) -> bool {
    let mut tracked_packets = HashSet::new();
    for &p in packets {
        if !tracked_packets.insert(p) {
            return true;
        }
    }
    false
}

fn find_start_of_packet(filename: &str) -> io::Result<i32> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut position: i32 = 0; // For the position
    let mut packet_in_process: Vec<char> = Vec::new();

    let mut found_packet: bool = false;

    // Get the line data
    for line in reader.lines() {
        if let Ok(line) = line {
            for letter in line.chars() {
                position += 1;
                packet_in_process.push(letter);
                if packet_in_process.len() > 4 {
                    packet_in_process.remove(0);
                }
                if packet_in_process.len() == 4 {
                    let repeat_letter = check_repetitive(&packet_in_process);
                    if repeat_letter == false {
                        found_packet = true;
                    }
                }
                if found_packet == true {
                    break;
                }
            }
        }
    }

    Ok(position)
}

fn find_start_of_message(filename: &str) -> io::Result<i32> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut position: i32 = 0; // For the position
    let mut message_in_process: Vec<char> = Vec::new();

    let mut found_message: bool = false;

    // Get the line data
    for line in reader.lines() {
        if let Ok(line) = line {
            for letter in line.chars() {
                position += 1;
                message_in_process.push(letter);
                if message_in_process.len() > 14 {
                    message_in_process.remove(0);
                }
                if message_in_process.len() == 14 {
                    let repeat_letter = check_repetitive(&message_in_process);
                    if repeat_letter == false {
                        found_message = true;
                    }
                }
                if found_message == true {
                    break;
                }
            }
        }
    }

    Ok(position)
}

fn main() {
    if let Ok(position) = find_start_of_packet("./day6.txt") {
        println!("Found packet in: {}", position);
    } else {
        eprintln!("Error reading file");
    }
    if let Ok(position) = find_start_of_message("./day6.txt") {
        println!("Found message in: {}", position);
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
    fn test_find_start_of_packet_1() {
        let content = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 7;
        let result = find_start_of_packet(temp_path).unwrap();
        assert_eq!(result, expected_result);

        let expected_result2: i32 = 19;
        let result2 = find_start_of_message(temp_path).unwrap();
        assert_eq!(result2, expected_result2);
    }

    #[test]
    fn test_find_start_of_packet_2() {
        let content = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 5;
        let result = find_start_of_packet(temp_path).unwrap();
        assert_eq!(result, expected_result);

        let expected_result2: i32 = 23;
        let result2 = find_start_of_message(temp_path).unwrap();
        assert_eq!(result2, expected_result2);
    }

    #[test]
    fn test_find_start_of_packet_3() {
        let content = "nppdvjthqldpwncqszvftbrmjlhg";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 6;
        let result = find_start_of_packet(temp_path).unwrap();
        assert_eq!(result, expected_result);

        let expected_result2: i32 = 23;
        let result2 = find_start_of_message(temp_path).unwrap();
        assert_eq!(result2, expected_result2);
    }

    #[test]
    fn test_find_start_of_packet_4() {
        let content = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 10;
        let result = find_start_of_packet(temp_path).unwrap();
        assert_eq!(result, expected_result);

        let expected_result2: i32 = 29;
        let result2 = find_start_of_message(temp_path).unwrap();
        assert_eq!(result2, expected_result2);
    }

    #[test]
    fn test_find_start_of_packet_5() {
        let content = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 11;
        let result = find_start_of_packet(temp_path).unwrap();
        assert_eq!(result, expected_result);

        let expected_result2: i32 = 26;
        let result2 = find_start_of_message(temp_path).unwrap();
        assert_eq!(result2, expected_result2);
    }
}
