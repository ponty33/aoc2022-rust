use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

struct DirFile {
    size: i32,
    name: String,
}

struct DirData {
    files: Vec<DirFile>,
    parent: Option<String>,
}

fn add_up_file_sizes(filename: &str) -> io::Result<i32> {
    // Open the file for reading
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum: i32 = 0; // For the total sum

    // Iterate through each line to build dir structure
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("$ cd") {}
        }
    }

    Ok(sum)
}

fn main() {
    if let Ok(sum) = add_up_file_sizes("./day7.txt") {
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
    fn test_add_up_file_sizes() {
        let content = r#"$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k"#
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("\n");
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 95437;
        let result = add_up_file_sizes(temp_path).unwrap();
        assert_eq!(result, expected_result);
    }
}
