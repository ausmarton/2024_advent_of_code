use regex::Regex;
use std::io;
use crate::util;

pub fn cleanse_and_evaluate_expression(file_path: &str, with_switches: bool) -> io::Result<i32> {
    match util::read_lines(file_path) {
        Ok(lines) => {
                        let instructions = lines.filter_map(Result::ok)
                                .collect::<Vec<String>>()
                                .join("");
                        Ok(exract_multiplicands(&instructions, with_switches)
                        .iter()
                        .map(|(x,y)| x*y)
                        .sum::<i32>())
                    },
        Err(e)    => Err(e)
    }
}

fn exract_multiplicands(expression: &str, with_switches: bool) -> Vec<(i32, i32)> {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";

    let regex = Regex::new(pattern).expect("Invalid regex");

    let filtered_expression = if with_switches {
        apply_instruction_switches(expression)
    } else {
        expression.to_string()
    };

    regex.captures_iter(&filtered_expression)
        .filter_map(|arguments| {
            //matched and captured patterns appear at located at [1]..
            //[0] seems to represent the full matched expr i.e "mul(123,34)" instead of 123 or 34
            let x = arguments[1].parse::<i32>().ok();
            let y = arguments[2].parse::<i32>().ok();
            match (x,y) {
                (Some(x), Some(y)) => Some((x,y)),
                _ => None
            }
        }).collect()
}

fn apply_instruction_switches(instructions: &str) -> String {
    let pattern = r"(don\'t\(\).*?do\(\))|(don\'t\(\).*$)";

    let regex = Regex::new(pattern).expect("Invalid regex");
    regex.replace_all(instructions, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let expression = "";
        let result = exract_multiplicands(expression, false);

        assert!(result.is_empty());
    }

    #[test]
    fn test_no_valid_patterns() {
        let expression = "invalid mul(abc,123) mul(12a,34) mul(1234,5678)";
        let result = exract_multiplicands(expression, false);

        assert!(result.is_empty());
    }

    #[test]
    fn test_single_valid_pattern() {
        let expression = "mul(12,34)";
        let result = exract_multiplicands(expression, false);

        let expected = vec![(12, 34)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_valid_matches() {
        let expression = "mul(1,34)mul(333,555) mul(456,13) mul(22,6)";
        let result = exract_multiplicands(expression, false);

        let expected = vec![(1,34), (333,555), (456,13), (22,6)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_expression_from_aoc_2024() {
        let expression = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = exract_multiplicands(expression, false);

        let expected = vec![(2,4), (5,5), (11,8), (8,5)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_instructions_when_start_is_disabled() {
        let expression = "don't()mul(1,2)";
        let filtered_expression = apply_instruction_switches(&expression);

        assert_eq!(filtered_expression, "");
    }

    #[test]
    fn test_filter_instructions_when_end_is_disabled() {
        let expression = "mul(1,2) don't()mul(4,5)";
        let filtered_expression = apply_instruction_switches(&expression);

        assert_eq!(filtered_expression, "mul(1,2) ");
    }

    #[test]
    fn test_filter_instructions_when_middle_is_disabled() {
        let expression = "mul(1,2) don't()mul(4,5)asdfdo()mul(6,7)";
        let filtered_expression = apply_instruction_switches(&expression);

        assert_eq!(filtered_expression, "mul(1,2) mul(6,7)");
    }

    #[test]
    fn test_filter_instructions_when_multiple_parts_are_disabled() {
        let expression = "mul(1,2) don't()mul(4,5)asdfdo()mul(6,7) do()2kjsdon't()mul(9,0)wfkcmul(333,9)do()mul(111,222)";
        let filtered_expression = apply_instruction_switches(&expression);

        assert_eq!(filtered_expression, "mul(1,2) mul(6,7) do()2kjsmul(111,222)");
    }
}