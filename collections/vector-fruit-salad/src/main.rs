use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

fn main() {
    // let mut fruit: Vec<&str> = vec![
    //     "Orange",
    //     "Fig",
    //     "Pomegranate",
    //     "Cherry",
    //     "Apple",
    //     "Pear",
    //     "Peach",
    // ];

    let mut fruit: Vec<String> = Vec::new();

    let mut input = String::new();
    loop {
        input.clear();
        print!("{:?} Add new fruit (type 'stop' to exit): ", fruit);
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed = input.trim();
        if trimmed == "stop" {
            break;
        } else {
            fruit.push(trimmed.to_string());
        }
    }

    // Scramble (shuffle) the fruit
    let mut rng: ThreadRng = thread_rng();
    fruit.shuffle(&mut rng);

    println!("Fruit salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            print!("{}", item);
        }
    }
    println!();
    let choice = fruit.choose(&mut rand::thread_rng()).unwrap();
    println!("selected random fruit: {}", choice);
}
