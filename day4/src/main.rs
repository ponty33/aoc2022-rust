use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_and_find_pairs(filename: &str) -> io::Result<i32> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut pairs: i32 = 0; // count the total pairs

    // Iterate through each line
    for line in reader.lines() {
        if let Ok(line) = line {
            let assignments: Vec<&str> = line.split(',').collect();
            let first_assignment: Vec<&str> = assignments[0].split('-').collect();
            let first_lower: i32 = first_assignment[0].parse().unwrap();
            let first_upper: i32 = first_assignment[1].parse().unwrap();

            let second_assignment: Vec<&str> = assignments[1].split('-').collect();
            let second_lower: i32 = second_assignment[0].parse().unwrap();
            let second_upper: i32 = second_assignment[1].parse().unwrap();
            if first_lower <= second_lower && first_upper >= second_upper {
                pairs += 1
            } else if second_lower <= first_lower && second_upper >= first_upper {
                pairs += 1
            } else {
            }
        }
    }

    Ok(pairs)
}

fn read_and_find_pairs_that_overlap_even_at_one_spot(filename: &str) -> io::Result<i32> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut pairs: i32 = 0; // count the total pairs

    // Iterate through each line
    for line in reader.lines() {
        if let Ok(line) = line {
            let assignments: Vec<&str> = line.split(',').collect();
            let first_assignment: Vec<&str> = assignments[0].split('-').collect();
            let first_lower: i32 = first_assignment[0].parse().unwrap();
            let first_upper: i32 = first_assignment[1].parse().unwrap();

            let second_assignment: Vec<&str> = assignments[1].split('-').collect();
            let second_lower: i32 = second_assignment[0].parse().unwrap();
            let second_upper: i32 = second_assignment[1].parse().unwrap();
            if first_upper < second_lower && first_lower < second_lower {
            } else if first_upper > second_upper && first_lower > second_upper {
            } else {
                pairs += 1
            }
        }
    }

    Ok(pairs)
}

fn main() {
    if let Ok(pairs) = read_and_find_pairs("./day4.txt") {
        println!("Total pairs: {}", pairs);
    } else {
        eprintln!("Error reading file");
    }
    if let Ok(pairs) = read_and_find_pairs_that_overlap_even_at_one_spot("./day4.txt") {
        println!("Total pairs: {}", pairs);
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
    fn test_part1() {
        let content = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 2;
        let result = read_and_find_pairs(temp_path).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part2() {
        let content = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let part2_expected_result: i32 = 4;
        let part2_result: i32 =
            read_and_find_pairs_that_overlap_even_at_one_spot(temp_path).unwrap();
        assert_eq!(part2_expected_result, part2_result);
    }
}
