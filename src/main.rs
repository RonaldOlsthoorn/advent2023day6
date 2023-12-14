
use std::{io::{BufReader, BufRead}, fs::File, ops::RangeInclusive};


struct Race {
    total_time: u64,
    record_distance: u64
}

impl Race {

    fn winning_strategies(&self) -> RangeInclusive<u64> {
        let determinant = ((self.total_time.pow(2) - (4 * self.record_distance)) as f64).sqrt();
        let start: u64 = (((self.total_time as f64 - determinant) / 2f64) + 1f64).floor() as u64;
        let end: u64 = (((self.total_time as f64 + determinant) / 2f64) - 1f64).ceil() as u64;
        start..=end 
    }
    
}

fn part1(races: &Vec<Race>) -> usize {
    races.iter().fold(1, |s, race| {
        let strategy = race.winning_strategies();
        s * ((strategy.end() - strategy.start()) as usize + 1)
    })
}


fn main() {
   
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();
    let mut races: Vec<Race> = Vec::new();

    for time in lines[0].split_whitespace().skip(1).map(|s| s.parse().unwrap()) {
        races.push(Race{total_time: time, record_distance: 0});
    }

    for (i, distance) in lines[1].split_whitespace().skip(1).map(|s| s.parse().unwrap()).enumerate() {
        races[i].record_distance = distance;
    }

    println!("part 1: {}", part1(&races));

    let filtered_times: Vec<&str> = lines[0].split_whitespace().skip(1).collect();
    let total_time: u64 = filtered_times.join("").parse().unwrap();

    let filtered_distances: Vec<&str> = lines[1].split_whitespace().skip(1).collect();
    let record_distance: u64 = filtered_distances.join("").parse().unwrap();

    let big_race = Race{total_time, record_distance};

    let winning_strategies = big_race.winning_strategies();

    println!("part 2: {}", winning_strategies.end() - winning_strategies.start() + 1);



}