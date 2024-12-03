fn main() {
    println!("Advent of Code 2024!");


    let day_1_input_file = "input/day1/input";

    match advent_of_code_2024::day1::total_distance(day_1_input_file) {
        Ok((distance, similarity)) => {
            println!("Total distance: {}", distance);
            println!("Similarity: {}", similarity)
        },
        Err(e) => eprintln!("Error encountered in file {}",e)
    }

    let day_2_input_file = "input/day2/input";

    match advent_of_code_2024::day2::count_safe_reports(day_2_input_file) {
        Ok(count) => println!("Safe reports: {}", count),
        Err(e) => eprintln!("Error encountered in file {}", e)
    }

    match advent_of_code_2024::day2::count_dampened_safe_reports(day_2_input_file) {
        Ok(count) => println!("Safe Dampened reports: {}", count),
        Err(e) => eprintln!("Error encountered in file {}", e)
    }

    let day_3_input_file = "input/day3/input";

    match advent_of_code_2024::day3::cleanse_and_evaluate_expression(day_3_input_file, false) {
        Ok(sum) => println!("Sum of all muls: {}", sum),
        Err(e) => eprintln!("Error encountered in file {}", e)
    }

    match advent_of_code_2024::day3::cleanse_and_evaluate_expression(day_3_input_file, true) {
        Ok(sum) => println!("Sum of all muls (filtered): {}", sum),
        Err(e) => eprintln!("Error encountered in file {}", e)
    }
}