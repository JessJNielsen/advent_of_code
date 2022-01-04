use std::error::Error;
use csv;

fn read_diagnostic_numbers_from_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let diagnostic_numbers: Vec<String> = reader
        .records()
        .map(|line| line.unwrap().as_slice().parse::<String>().unwrap())
        .collect();

    Ok(diagnostic_numbers)
}

pub fn run() {
    println!("## Task 03 ##");
    println!("\nPart 1");
    part_1();

    println!("\nPart 2");
    part_2();
}

fn part_1() {
    let diagnostic_numbers = read_diagnostic_numbers_from_file("./days/day_03/data/diagnostics.csv").unwrap();

    // Group the chars across numbers by their position
    let mut char_sums: Vec<Vec<char>> = vec![];

    for diag_number in diagnostic_numbers {
        let chars: Vec<char> = diag_number.chars().collect();
        for (position, char) in chars.iter().enumerate() {
            if char_sums.len() <= position {
                char_sums.push(vec![])
            }

            char_sums[position].push(*char);
        }
    }

    let mut gamma_result = String::new();
    let mut epsilon_result = String::new();

    // Calculate both results by finding the most common char at setting that in a binary string, then parsing to decimal.
    for sum in char_sums {
        let one_count = sum.iter().filter(|&n| *n == '1').count();

        gamma_result.push(if one_count > sum.len() / 2 { '1' } else { '0' });
        epsilon_result.push(if one_count > sum.len() / 2 { '0' } else { '1' });
    }
    let gamma_rate = isize::from_str_radix(&*gamma_result, 2).unwrap();
    println!("Gamma Rate: {}", gamma_rate);

    let epsilon_rate = isize::from_str_radix(&*epsilon_result, 2).unwrap();
    println!("Epsilon Rate: {}", epsilon_rate);

    println!("Power Consumption (Gamma * Epsilon): {}", gamma_rate * epsilon_rate)
}

fn part_2() {
    let diagnostic_numbers = read_diagnostic_numbers_from_file("./days/day_03/data/diagnostics.csv").unwrap();

    let oxygen_generator_rating = calculate_rating(diagnostic_numbers.clone(), true);
    let o2_rating = isize::from_str_radix(oxygen_generator_rating.as_str(), 2).unwrap();
    println!("Oxygen Generator Rating - binary: {}, decimal: {}", oxygen_generator_rating, o2_rating);

    let co2_scrubber_rating = calculate_rating(diagnostic_numbers.clone(), false);
    let co2_rating = isize::from_str_radix(co2_scrubber_rating.as_str(), 2).unwrap();
    println!("CO2 scrubber Rating - binary: {}, decimal: {}", co2_scrubber_rating, co2_rating);

    println!("Life support rating (O2 * CO2): {}", o2_rating * co2_rating)
}

fn calculate_rating(diagnostic_numbers: Vec<String>, sort_by_most_common: bool) -> String {
    let length = diagnostic_numbers[0].len();

    let mut filtered_numbers = diagnostic_numbers.to_vec();

    for position in 0..length {
        // println!("position: {}, filtered_numbers: {:?}", position, filtered_numbers);
        let mut one_count = 0;
        let mut zero_count = 0;
        // For each position check most common char (0 or 1) in that position of each number
        for diag_number in &filtered_numbers {
            match diag_number.chars().nth(position).unwrap() {
                '1' => one_count += 1,
                '0' => zero_count += 1,
                _ => {}
            }
        }

        // Calculate bit_criteria
        let bit_criteria = if sort_by_most_common {
            if one_count >= zero_count { '1' } else { '0' } // Most common value or '1' if equal
        } else {
            if zero_count <= one_count { '0' } else { '1' } // Least common value or '0' if equal
        };

        filtered_numbers.retain(|num| num.chars().nth(position).unwrap() == bit_criteria);

        if filtered_numbers.len() == 1 {
            break;
        }
    }

    return filtered_numbers.first().cloned().unwrap();
}