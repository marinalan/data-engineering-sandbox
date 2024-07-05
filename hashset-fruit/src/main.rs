use rand::seq::SliceRandom;
use rand::thread_rng;
//use std::collections::HashSet;
use std::collections::HashMap;
use clap::Parser;

fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}

#[derive(Debug)]
#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of random fruits to generate"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
    //items: String
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    //let mut fruit_set = HashSet::new();
    let mut fruit_set:HashMap<&str, usize> = HashMap::new();
    println!("Generating {num_fruits} random fruits...");
    for _ in 0..num_fruits {
        let item = generate_fruit();
        let stat = fruit_set.entry(item).or_insert(0);
        *stat +=1;
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());
    //println!("Unique fruits: {:?}", fruit_set.values());
    for (fruit, counter) in fruit_set {
        println!("{fruit}: {counter}");
    }
}
