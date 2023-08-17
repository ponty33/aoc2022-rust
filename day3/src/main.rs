use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn find_common_letter(s1: &str, s2: &str) -> Option<char> {
    for c1 in s1.chars() {
        if s2.contains(c1) {
            return Some(c1);
        }
    }
    None
}

fn find_common_badge(s1: &str, s2: &str, s3: &str) -> Option<char> {
    for c1 in s1.chars() {
        if s2.contains(c1) && s3.contains(c1) {
            return Some(c1);
        }
    }
    None
}

fn get_alphabet_map() -> HashMap<char, i32> {
    // Build alphabet map
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let mut alphabet_map: HashMap<char, i32> = HashMap::new();
    let mut letter_value: i32 = 0;
    for letter in alphabet.iter() {
        letter_value += 1;
        alphabet_map.insert(*letter, letter_value); // Corrected insert method call
    }
    alphabet_map
}

fn read_and_accumulate_rucksacks(filename: &str) -> io::Result<i32> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum: i32 = 0; // For the total sum

    let alphabet_map: HashMap<char, i32> = get_alphabet_map();

    // Iterate through each line
    for line in reader.lines() {
        if let Ok(line) = line {
            let str_length = line.len() / 2;
            let slice1 = &line[0..str_length];
            let slice2 = &line[str_length..];
            if let Some(common_letter) = find_common_letter(slice1, slice2) {
                let common_btw_two = common_letter;
                let value_num: &i32 = alphabet_map.get(&common_btw_two).unwrap_or(&0);
                sum += value_num
            } else {
                println!("Error! No common letter.");
            }
        }
    }

    Ok(sum)
}

fn read_and_accumulate_group_badges(filename: &str) -> io::Result<i32> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum: i32 = 0; // For the total sum

    let alphabet_map: HashMap<char, i32> = get_alphabet_map();

    let mut group: Vec<String> = Vec::new();
    // Iterate through each line along with its index
    for line in reader.lines() {
        if let Ok(line) = line {
            group.push(line.clone()); // Clone the line before pushing

            if group.len() == 3 {
                let s1 = &group[0];
                let s2 = &group[1];
                let s3 = &group[2];
                if let Some(common_badge) = find_common_badge(s1, s2, s3) {
                    let value_num = alphabet_map.get(&common_badge).unwrap_or(&0);
                    sum += value_num;
                }

                // Clear the group vector for the next iteration
                group.clear();
            }
        }
    }
    Ok(sum)
}

fn main() {
    if let Ok(sum) = read_and_accumulate_rucksacks("./day3.txt") {
        println!("Total: {}", sum);
    } else {
        eprintln!("Error reading file");
    }
    if let Ok(sum) = read_and_accumulate_group_badges("./day3.txt") {
        println!("Total: {}", sum);
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
    fn test_read_and_accumulate_rucksacks() {
        let content = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 157;
        let result = read_and_accumulate_rucksacks(temp_path).unwrap();
        assert_eq!(result, expected_result);
    }
    #[test]
    fn part2_test() {
        let content = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");
        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 70;
        let result = read_and_accumulate_group_badges(temp_path).unwrap();

        assert_eq!(result, expected_result)
    }
}
