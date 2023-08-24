use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_and_find_pairs(filename: &str) -> io::Result<i32> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut pairs: i32 = 0; // count the total pairs
    let mut columns: Vec<Vec<char>> = vec![];
    // Iterate through each line to build arrays
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break; // Stop reading after the empty line
            }

            let chars: Vec<char> = line.chars().collect();
            for (col_idx, c) in chars.iter().enumerate() {
                if col_idx >= columns.len() {
                    columns.push(vec![]);
                }

                if c.is_alphabetic() {
                    columns[col_idx].push(*c);
                }
            }
        }
    }

    columns.retain(|col| !col.is_empty()); // Remove empty columns

    for column in columns {
        println!("{:?}", column);
    }

    let file2 = File::open(filename)?;
    let reader2 = BufReader::new(file2);

    // Iterate to process movement
    for line in reader2.lines() {
        if let Ok(line) = line {
            if line.starts_with('m') {
                let movements: Vec<&str> = line.split(' ').collect();
                // Quantity to be moved
                let move_quan: i32 = movements[1].parse().unwrap();
                // From which stack
                let from_stack: i32 = movements[3].parse::<i32>().unwrap() + 1;
                // To which stack
                let to_stack: i32 = movements[5].parse::<i32>().unwrap() + 1;

                if let Some(source) = columns.get_mut(from_stack as usize) {
                    if let Some(destination) = columns.get_mut(to_stack as usize) {
                        for _ in 0..move_quan {
                            if let Some(c) = source.pop() {
                                destination.insert(0, c);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(pairs)
}

fn main() {
    if let Ok(pairs) = read_and_find_pairs("./day5.txt") {
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
        let content = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 2;
        let result = read_and_find_pairs(temp_path).unwrap();
        assert_eq!(result, expected_result);
    }
}
