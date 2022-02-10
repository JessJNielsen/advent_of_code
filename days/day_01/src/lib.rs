use std::error::Error;
use csv;

fn read_depths_from_file(path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let numbers: Vec<i32> = reader
        .records()
        .map(|line| line.unwrap().as_slice().parse::<i32>().unwrap())
        .collect();

    Ok(numbers)
}

pub fn run() {
    println!("##Day 01 ##");
    println!("\nPart 1");
    part_1();

    println!("\nPart 2");
    part_2();
}


fn part_1() {
    let depths = read_depths_from_file("./day_01/data/depths.csv")
        .unwrap();

    let mut incremented: i32 = 0;
    let mut previous_depth: i32 = depths[0];

    for depth in depths {
        if previous_depth < depth {
            incremented = incremented + 1;
        }
        previous_depth = depth;
    }

    println!("increments: {}", incremented);
}

fn part_2() {
    let depths = read_depths_from_file("./day_01/data/depths.csv")
        .unwrap();

    let mut start_index: usize = 0;
    let mut end_index: usize = 2;

    let mut incremented: i32 = 0;
    let mut previous_depths_sum: i32 = 0;

    while end_index < depths.len() {
        // grab 3 values from start_index to end_index
        let grabbed_values = &depths[start_index..=end_index];

        let depths_sum = grabbed_values.iter().sum();

        if previous_depths_sum < depths_sum && previous_depths_sum != 0 {

            println!("start_index: {} end_index: {}", start_index, end_index);
            println!("increment: {} is bigger than {}", depths_sum, previous_depths_sum);
            incremented = incremented + 1;
        }

        previous_depths_sum = depths_sum;
        start_index = start_index + 1;
        end_index = end_index + 1;
    }

    println!("increments: {}", incremented);
}
