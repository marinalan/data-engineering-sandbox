extern crate rasciigraph;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rasciigraph::{plot, Config};

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    let mut cities:Vec<&str> = Vec::new();
    let mut distances_travelled:Vec<f64> = Vec::new();
    let mut cities_line = String::new();
    let mut distances_line = String::new();

    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (index,line) in lines.flatten().enumerate() {
            println!("{index} - {}", line);
            match index {
                0 => { 
                    cities_line = line.clone(); 
                    cities = cities_line.split_whitespace().collect(); 
                },
                1 => { 
                    distances_line = line.clone();
                    distances_travelled = distances_line
                        .split_whitespace()
                        .map(|x| x.parse::<f64>().unwrap())
                        .collect(); 
                },
                _ => {}
            }
        }
    } else {
       cities = vec!["Lisbon", "Madrid", "Paris", "Berlin", "Copenhagen", "Stockholm", "Moscow"];
       distances_travelled = vec![0.0, 502.56, 1053.36, 2187.27, 2636.42, 3117.23, 4606.35];
    }

    println!("{}", cities.join(" > "));

    println!(
        "{}",
        plot(
            distances_travelled.into_iter().map(|d| d as f64).collect(),
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("Travelled distances (km)".to_string())
        )
    );
}
