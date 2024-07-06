/*
Usage:  

cargo run -- fruits.csv
or
cargo run -- --fruits "apple, pear"
or
cargo run -- fruits.csv --dressing "apple cider"
 */

use clap::Parser;
use fruit_salad_maker::create_fruit_salad;
use std::fs::File;
use std::io::{Error, Write};

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Make a Fruit Salad"
)]
struct Opts {
    /// Fruits input as a string of comma separated values
    #[clap(short, long)]
    fruits: Option<String>,
    csvfile: Option<String>,
    #[clap(short, long)]
    dressing: Option<String>
}

// Function that converts a csv file to a vector of strings
fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}
fn display_fruit_salad(fruits: Vec<String>) {
    println!("Your fruit salad contains:");
    for fruit in fruits {
        println!("{}", fruit);
    }
}

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    println!("dressing parameter: {:?}", opts.dressing);
    let dressing = match opts.dressing {
        Some(sauce) => sauce,
        None => "no dressing".to_string()
    };
    println!("dressing parameter: {}", dressing);

    // Use fruits from CSV file or command-line input
    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename)
                .expect("Could not read file");
            csv_to_vec(&fruits)
        },
        None => {
            opts.fruits.unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        },
    };

    // display fruit salad
    let fruit_salad = create_fruit_salad(fruit_list);
    println!("joining vec: {}",fruit_salad.join(", "));
    let line = fruit_salad.join(", ");

    display_fruit_salad(fruit_salad);

    // saving in file
    let path = "results.txt";
    let mut output = File::create(path)?;
    writeln!(output, "{}", line)
}
