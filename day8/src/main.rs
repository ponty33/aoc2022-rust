use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn iterate_tree_map(tree_map: &Vec<Vec<u32>>) -> i32 {
    let mut visible_trees: i32 = 0;
    let n = tree_map.len();

    for i in 1..n - 1 {
        for j in 1..n - 1 {
            let mut left_larger: bool = false;
            let mut right_larger: bool = false;
            let mut up_larger: bool = false;
            let mut down_larger: bool = false;

            let current_element = tree_map[i][j];

            // println!("Now at {}", current_element);

            for k in 0..j {
                if k != j && tree_map[i][k] >= current_element {
                    left_larger = true;
                    break;
                }
            }
            for k in j + 1..n {
                if k != j && tree_map[i][k] >= current_element {
                    right_larger = true;
                    break;
                }
            }
            for k in 0..i {
                if k != i && tree_map[k][j] >= current_element {
                    up_larger = true;
                    break;
                }
            }
            for k in i + 1..n {
                if k != i && tree_map[k][j] >= current_element {
                    down_larger = true;
                    break;
                }
            }

            if !left_larger || !right_larger || !up_larger || !down_larger {
                visible_trees += 1;
            } else {
            }
        }
    }
    visible_trees
}

fn find_visible_tree(filename: &str, n: usize) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut tree_map = vec![vec![0; n]; n];

    let mut row = 0;
    let mut col = 0;

    for line in reader.lines() {
        let string_line = line.unwrap();
        for digit in string_line.chars() {
            if digit.is_digit(10) {
                tree_map[row][col] = digit.to_digit(10).unwrap();
                col += 1;
            }
        }
        row += 1;
        col = 0;
    }
    let edge_trees: i32 = (n as i32 - 1) * 4;
    println!("{}", edge_trees);
    let visible_trees: i32 = iterate_tree_map(&tree_map) + edge_trees;

    Ok(visible_trees)
}

fn iterate_tree_map_and_find_best_tree(tree_map: &Vec<Vec<u32>>) -> i32 {
    let mut best_tree_score: i32 = 0;
    let n = tree_map.len();

    for i in 0..n {
        for j in 0..n {
            let mut left_score: i32 = 0;
            let mut right_score: i32 = 0;
            let mut up_score: i32 = 0;
            let mut down_score: i32 = 0;

            let current_element = tree_map[i][j];

            // println!("Now at {}", current_element);

            for k in (0..j).rev() {
                if k != j && tree_map[i][k] >= current_element {
                    left_score += 1;
                    break;
                } else {
                    left_score += 1;
                }
            }
            for k in j + 1..n {
                if k != j && tree_map[i][k] >= current_element {
                    right_score += 1;
                    break;
                } else {
                    right_score += 1;
                }
            }
            for k in (0..i).rev() {
                // println!("Check {}", tree_map[k][j]);
                if k != i && tree_map[k][j] >= current_element {
                    up_score += 1;
                    break;
                } else {
                    up_score += 1;
                }
            }
            for k in i + 1..n {
                if k != i && tree_map[k][j] >= current_element {
                    down_score += 1;
                    break;
                } else {
                    down_score += 1;
                }
            }
            let tree_score: i32 = left_score * right_score * up_score * down_score;
            if tree_score > best_tree_score {
                best_tree_score = tree_score.clone();
            }
            // println!("=================")
        }
    }
    best_tree_score
}

fn find_best_viewing_tree(filename: &str, n: usize) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut tree_map = vec![vec![0; n]; n];

    let mut row = 0;
    let mut col = 0;

    for line in reader.lines() {
        let string_line = line.unwrap();
        for digit in string_line.chars() {
            if digit.is_digit(10) {
                tree_map[row][col] = digit.to_digit(10).unwrap();
                col += 1;
            }
        }
        row += 1;
        col = 0;
    }

    let best_tree_score = iterate_tree_map_and_find_best_tree(&tree_map);
    Ok(best_tree_score)
}

fn main() {
    if let Ok(visible_trees) = find_visible_tree("./day8.txt", 99) {
        println!("Visible trees: {}", visible_trees);
    } else {
        eprintln!("Error reading file");
    }

    if let Ok(best_tree_score) = find_best_viewing_tree("./day8.txt", 99) {
        println!("Best tree score: {}", best_tree_score);
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
    fn test_find_visible_tree() {
        let content = "30373\n25512\n65332\n33549\n35390";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 21;
        let result = find_visible_tree(temp_path, 5).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_find_visible_tree_2() {
        let content = "66666\n67776\n67876\n67776\n66666";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 25;
        let result = find_visible_tree(temp_path, 5).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_find_best_viewing_tree() {
        let content = "30373\n25512\n65332\n33549\n35390";
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().expect("Invalid temp path");

        let mut file = File::create(temp_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content");

        let expected_result: i32 = 8;
        let result = find_best_viewing_tree(temp_path, 5).unwrap();
        assert_eq!(result, expected_result);
    }
}
