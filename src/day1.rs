use crate::util;
use std::io;
use std::collections::HashMap;

pub fn total_distance(file_path: &str) -> io::Result<(i32, i32)> {
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    if let Ok(lines) = util::read_lines(file_path) {
        for line in lines {
            if let Ok(record) = line {
                let parts: Vec<&str> = record.split_whitespace().collect();
                if parts.len() == 2 {
                    if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        column1.push(num1);
                        column2.push(num2);
                    }
                }
            } 
        }
    }

    column1.sort_unstable();
    column2.sort_unstable();

    //part 1
    let total_distance: i32 = column1
        .iter()
        .zip(column2.iter())
        .map(|(c1, c2)| (c1 - c2).abs())
        .sum();

    //part 2
    let mut column2_counts = HashMap::new();
    for &num in &column2 {
        *column2_counts.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = column1
        .iter()
        .map(|&c1| column2_counts.get(&c1).cloned().unwrap_or(0)*c1)
        .sum();

    
    Ok((total_distance, similarity_score))
}
