use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    println!("## Day 08: Seven Segment Search ##");

    println!("\nPart 1");
    part_1();

    println!("\nPart 2");
    part_1();
}

fn part_1() {
    let entries = load_entries_from_inputfile("./days/day_08/data/input.txt").unwrap();
    let mut count: i32 = 0;

    for entry in entries {
        for output in entry.output {
            if [2, 4, 3, 7].contains(&output.len()) {
                count += 1
            }
        }
    }

    println!("In the output values, how many times do digits 1, 4, 7, or 8 appear?");
    println!("Answer: {}", count);
}

#[derive(Debug, Clone)]
struct InputEntry {
    input: Vec<String>,
    output: Vec<String>,
}

fn load_entries_from_inputfile(path: &str) -> Result<Vec<InputEntry>, &str> {
    let file = File::open(path).expect("File should be able to open");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line to string"))
        .collect();

    let mut entries: Vec<InputEntry> = vec![];

    // map to Vec of structs
    for line in lines {
        // Split at " | "
        let line_parts: Vec<&str> = line.split(" | ").collect();

        entries.push(InputEntry {
            input: line_parts[0].split(' ').map(|s| s.parse().unwrap()).collect(),
            output: line_parts[1].split(' ').map(|s| s.parse().unwrap()).collect(),
        })
    }

    Ok(entries)
}
