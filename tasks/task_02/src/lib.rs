use std::error::Error;
use csv;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Maneuver {
    direction: String,
    amount: usize,
}

fn read_maneuvers_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
            .delimiter(b' ')
            .from_path(path)?;

    for record in reader.records() {

        let maneuver: Maneuver = record?.deserialize(None)?;
        println!("{:?}", maneuver);
    }


    let maneuvers: Vec<Maneuver> = reader.records()
        .map(|record| record?.deserialize(None) as Maneuver)
        .collect();

    println!("{:?}", maneuvers);
    Ok(())
}

pub fn run() {
    part_1();
}


fn part_1() {
    let maneuvers = read_maneuvers_from_file("./tasks/task_02/data/test_maneuvers.csv")
        .unwrap();

    println!("maneuvers: {:?}", maneuvers);
}