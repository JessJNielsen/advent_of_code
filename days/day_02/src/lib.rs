use std::error::Error;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Maneuver {
    direction: String,
    amount: i32,
}

fn read_maneuvers_from_file(path: &str) -> Result<Vec<Maneuver>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
            .delimiter(b' ')
            .from_path(path)?;

    let mut maneuvers: Vec<Maneuver> = vec![];

    for record in reader.records() {
        let maneuver: Maneuver = record?.deserialize(None)?;
        maneuvers.push(maneuver);
    }

    Ok(maneuvers)
}

pub fn run() {
    println!("## Day 02 ##");
    println!("\nPart 1");
    part_1();

    println!("\nPart 2");
    part_2();
}

fn part_1() {
    let maneuvers = read_maneuvers_from_file("./days/day_02/data/maneuvers.csv")
        .unwrap();

    println!("maneuvers: {:?}", maneuvers);

    let mut horizontal_position: i32 = 0;
    let mut depth: i32 = 0;

    for Maneuver { direction, amount } in maneuvers {
        match direction.as_str() {
            "forward" => horizontal_position = horizontal_position + amount,
            "down" => depth = depth + amount,
            "up" => depth = depth - amount,
            _ => {}
        }
    }

    println!("horizontal_position: {}, depth: {}", horizontal_position, depth);
    println!("horizontal_position * depth: {}", horizontal_position * depth);
}

fn part_2() {
    let maneuvers = read_maneuvers_from_file("./days/day_02/data/maneuvers.csv")
        .unwrap();

    println!("maneuvers: {:?}", maneuvers);

    let mut horizontal_position: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for Maneuver { direction, amount } in maneuvers {
        match direction.as_str() {
            "forward" => {
                horizontal_position = horizontal_position + amount;

                if aim > 0 {
                    depth = depth + (aim * amount);
                }
            },
            "down" => aim = aim + amount,
            "up" => aim = aim - amount,
            _ => {}
        }
    }

    println!("horizontal_position: {}, depth: {}", horizontal_position, depth);
    println!("horizontal_position * depth: {}", horizontal_position * depth);
}