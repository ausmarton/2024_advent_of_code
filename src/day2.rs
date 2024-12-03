use crate::util;
use std::io;

//day 2 - part 1
pub fn count_safe_reports(file_path: &str) -> io::Result<usize> {
    match util::read_lines(file_path) {
        Ok(lines) => Ok(lines.filter_map(Result::ok)
                        .map(|line| line.split_whitespace()
                        .filter_map(|s| s.parse::<i32>().ok()).collect())
                        .filter(|report| is_safe_report(report)).count()),
        Err(e) => Err(e)

    }
}

//day 2 - part 2
pub fn count_dampened_safe_reports(file_path: &str) -> io::Result<usize> {
    match util::read_lines(file_path) {
        Ok(lines) => Ok(lines.filter_map(Result::ok)
                        .map(|line| line.split_whitespace()
                        .filter_map(|s| s.parse::<i32>().ok()).collect())
                        .filter(|report| is_safe_report_dampened(report)).count()),
        Err(e) => Err(e)

    }
}


fn is_safe_report(report: &Vec<i32>) -> bool {
    let ordered = if is_ascending(report) {
        report
    } else {
        &report.iter().rev().cloned().collect()
    };
    is_increasing_gradually(&ordered)
}

fn is_increasing_gradually(report: &Vec<i32>) -> bool {
    report.windows(2)
    .all(|w| w[0] < w[1] && (w[0]+4) > w[1])
}

fn is_ascending(report: &Vec<i32>) -> bool {
    report.windows(2)
    .filter(|w| w[0] < w[1])
    .count() > report.windows(2)
    .filter(|w| w[0] > w[1])
    .count() 
}

fn is_safe_report_dampened(report: &Vec<i32>) -> bool {
    let ordered = if is_ascending(report) {
        report
    } else {
        &report.iter().rev().cloned().collect()
    };

    if is_increasing_gradually(&ordered) {
        return true;
    }

    (0..ordered.len())
    .map(|i| {
        ordered
            .iter()
            .enumerate()
            .filter(|&(index, _)| index != i)
            .map(|(_, &value)| value)
            .collect::<Vec<i32>>()
    })
    .any(|v| is_increasing_gradually(&v))
}