use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct SumData {
    largest_number: i32,
    total_sum: i32,
}

fn read_and_accumulate_numbers(filename: &str) -> io::Result<Vec<HashMap<i32, i32>>> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sums: Vec<HashMap<i32, i32>> = Vec::new(); // For storing the list of sums
    let mut sum_data = SumData {
        largest_number: i32::MIN, // Initialize with the smallest possible value
        total_sum: 0,
    };

    // Iterate through each line
    for line in reader.lines() {
        if let Ok(line) = line {
            // Check if the line is empty or contains only whitespace
            if line.trim().is_empty() {
                // Add the current sum_data to the list of sums
                let mut sum_map = HashMap::new();
                sum_map.insert(sum_data.largest_number, sum_data.total_sum);
                sums.push(sum_map);

                // Reset the sum_data for the next sequence
                sum_data = SumData {
                    largest_number: i32::MIN,
                    total_sum: 0,
                };
            } else {
                // Parse the line as an integer and update the sum_data
                if let Ok(number) = line.trim().parse::<i32>() {
                    sum_data.total_sum += number;
                    sum_data.largest_number = sum_data.largest_number.max(number);
                }
            }
        }
    }

    // Sort the sums list by total sum in ascending order
    sums.sort_by(|a, b| a.values().next().unwrap().cmp(b.values().next().unwrap()));

    Ok(sums)
}

fn main() {
    if let Ok(sums) = read_and_accumulate_numbers("./day1.txt") {
        for sum in sums {
            for (largest, total) in sum {
                println!("Largest: {}, Total: {}", largest, total);
            }
        }
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
    fn test_read_and_accumulate_numbers() {
        let content = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: Vec<HashMap<i32, i32>> = vec![
            [(4000, 4000)].iter().cloned().collect(),
            [(3000, 6000)].iter().cloned().collect(),
            [(6000, 11000)].iter().cloned().collect(),
            [(9000, 24000)].iter().cloned().collect(),
        ];
        let result = read_and_accumulate_numbers(temp_path).unwrap();
        assert_eq!(result, expected_result);
    }
}
