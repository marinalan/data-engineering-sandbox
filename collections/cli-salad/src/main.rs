/// Example of command to run:
/// ``` cargo run -- number 3 "apples oranges mandarins pears"
/// ```
use clap::Parser;
//use cli_salad::create_fruit_salad;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
    items: String
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);

    // Get the number of fruits the user requested
    let num_fruits = opts.number;
    let items = opts.items; 
    let parts:Vec<&str> = items.split_whitespace().collect();

    // Create the fruit salad
    let mut fruits:Vec<String> = Vec::new(); //create_fruit_salad(num_fruits);
    for part in parts {
        fruits.push(part.to_string());
    }

    let mut rng = thread_rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);

    let mut fruits:Vec<String> = fruits.into_iter().take(num_fruits).collect();
    fruits.sort();

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits,
        fruits
    );
}
