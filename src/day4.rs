use crate::util;
use std::io;

pub fn count_occurences_in_text(file_path: &str) -> io::Result<i32> {
    match util::read_lines(file_path) {
        Ok(lines) => {
                        let text = lines.filter_map(Result::ok)
                                .collect::<Vec<String>>();

                        Ok(full_search(text))
                    },
        Err(e)    => Err(e)
    }
}

fn full_search(text: Vec<String>) -> i32 {
    let horizontal = count_occurrences(&text);
    let matrix = to_matrix(&text);

    let transposed = transpose(&matrix);
    let vertical = count_occurrences(&transposed);

    let diagonals = count_occurrences(&extract_diagonals(&matrix));
    let reversed_lines: Vec<String> = text.iter()
    .map(|l| l.chars().rev().collect::<String>())
    .collect();

    let anti_diagonals = count_occurrences(&extract_diagonals(&to_matrix(&reversed_lines)));
    horizontal + vertical + diagonals + anti_diagonals
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<String> {
    if matrix.is_empty() {
        return vec![];
    }

    let row_count = matrix.len();
    let column_count = matrix[0].len();

    let mut transposed = vec![vec![' '; row_count]; column_count];

    for i in 0..row_count {
        for j in 0..column_count {
            transposed[j][i] = matrix[i][j];
        }
    }

    transposed
    .iter()
    .map(|v| v.into_iter().collect())
    .collect()
}

fn extract_diagonals(matrix: &Vec<Vec<char>>) -> Vec<String> {
    if matrix.is_empty() {
        return vec![];
    }

    let row_count = matrix.len();
    let column_count = matrix[0].len();

    let mut diagonals: Vec<String> = Vec::new();


    for column in 0..column_count {
        let mut diagonal = String::new();
        let mut i = 0;
        let mut j = column;

        while i < row_count && j < column_count {
            diagonal.push(matrix[i][j]);
            i += 1;
            j += 1;
        }
        diagonals.push(diagonal);
    }

    for row in 1..row_count {
        let mut diagonal = String::new();
        let mut i = row;
        let mut j = 0;

        while i < row_count && j < column_count {
            diagonal.push(matrix[i][j]);
            i += 1;
            j += 1;
        }
        diagonals.push(diagonal);
    }

    diagonals
}

fn to_matrix(block: &Vec<String>) -> Vec<Vec<char>> {
    if block.is_empty() || block.iter().all(|s| s.is_empty()) {
        return vec![];
    }

    block.iter()
    .map(|s| s.chars().collect())
    .collect()
}


fn count_occurrences(block: &Vec<String>) -> i32 {
    block.iter()
    .map(|line| line.matches("XMAS").count() + line.matches("SAMX").count())
    .map(|c| c as i32) //TODO. determine if i32 is good enough
    .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec_string(v: Vec<&str>) -> Vec<String> {
        v.iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn count_forward_occurrences() {
        assert_eq!(count_occurrences(&to_vec_string(vec!["XMASSSMAXXMASAASMMSAXMASMSM"])), 3)
    }
    
    #[test]
    fn count_reverse_occurrences() {
        assert_eq!(count_occurrences(&to_vec_string(vec!["SAMXSSMAXSAMXAASMMSASAMXMSM"])), 3)
    }

    #[test]
    fn count_occurrences_in_multiple_lines() {
        assert_eq!(count_occurrences(&to_vec_string(vec!["XMASSSMAXXMASAASMMSAXMASMSM", "XMASSSMAXXMASAASMMSAXMASMSM"])), 6)
    }

    //transposition
    #[test]
    fn test_transposition_for_small_matrix() {
        let initial = to_vec_string(vec!["123", "456", "789"]);
        let expected = vec!["147", "258", "369"];

        assert_eq!(transpose(&to_matrix(&initial)), expected);
    }

    #[test]
    fn test_diagonals_for_small_matrix() {
        let initial = to_vec_string(vec![ "0123", "4567", "89AB", "CDEF"]);
        let expected = vec![ "05AF", "16B", "27", "3", "49E", "8D", "C" ];

        assert_eq!(extract_diagonals(&to_matrix(&initial)), expected);
    }
}