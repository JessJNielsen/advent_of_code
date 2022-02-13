use std::time::{Instant};

pub fn run() {
    println!("## Day 06: Lanternfish ##");

    // Part 1
    println!("Running Part 1\n");


    let start = Instant::now();
    fish_simulator(
        "3,4,3,1,2",
        80
    ); // Take sonly 591.916µs
    let duration = start.elapsed();
    println!("Part 1 completed in: {:?}", duration);

    // Part 2
    println!("Running Part 2\n");

    let start = Instant::now();
    fish_simulator(
        "1,3,3,4,5,1,1,1,1,1,1,2,1,4,1,1,1,5,2,2,4,3,1,1,2,5,4,2,2,3,1,2,3,2,1,1,4,4,2,4,4,1,2,4,3,3,3,1,1,3,4,5,2,5,1,2,5,1,1,1,3,2,3,3,1,4,1,1,4,1,4,1,1,1,1,5,4,2,1,2,2,5,5,1,1,1,1,2,1,1,1,1,3,2,3,1,4,3,1,1,3,1,1,1,1,3,3,4,5,1,1,5,4,4,4,4,2,5,1,1,2,5,1,3,4,4,1,4,1,5,5,2,4,5,1,1,3,1,3,1,4,1,3,1,2,2,1,5,1,5,1,3,1,3,1,4,1,4,5,1,4,5,1,1,5,2,2,4,5,1,3,2,4,2,1,1,1,2,1,2,1,3,4,4,2,2,4,2,1,4,1,3,1,3,5,3,1,1,2,2,1,5,2,1,1,1,1,1,5,4,3,5,3,3,1,5,5,4,4,2,1,1,1,2,5,3,3,2,1,1,1,5,5,3,1,4,4,2,4,2,1,1,1,5,1,2,4,1,3,4,4,2,1,4,2,1,3,4,3,3,2,3,1,5,3,1,1,5,1,2,2,4,4,1,2,3,1,2,1,1,2,1,1,1,2,3,5,5,1,2,3,1,3,5,4,2,1,3,3,4",
        256
    );
    let duration = start.elapsed();
    println!("Part 2 completed in: {:?}", duration);
}

fn fish_simulator(input: &str, days: i16) {
    let initial_fish: Vec<i8> = input.split(',').map(|s| s.parse().unwrap()).collect();

    // We make a bucket for each lifetime for fish, 8 days + a 0-day
    let mut fish_buckets = [0; 9];

    // Fill buckets with initial fish
    for bucket in 0..fish_buckets.len() {
        fish_buckets[bucket] = initial_fish.iter().filter(|fish| usize::try_from(**fish).unwrap() == bucket).count();
    }

    for day in 1..=days {
        // Get dead fish from 0 and clear bucket afterwards
        let dead_fish = fish_buckets[0];
        fish_buckets[0] = 0;

        // Iterate all buckets moving contents 'forwards', and clearing bucket
        for bucket in 1..fish_buckets.len() {
            fish_buckets[bucket - 1] = fish_buckets[bucket];
            fish_buckets[bucket] = 0;
        }

        // Lastly add dead_fish amount to fish_buckets[6] and bucket 8,
        fish_buckets[6] += dead_fish;
        fish_buckets[8] = dead_fish;

        println!("Fish after {} day(s): {}", day, fish_buckets.iter().sum::<usize>());
    }

    println!("Fish after {} days: {}", days,  fish_buckets.iter().sum::<usize>());
}
