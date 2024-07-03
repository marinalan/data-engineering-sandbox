/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::LinkedList;
use std::io::{self, Write};

fn parse_cmd<'a>(cmd: String, fruits: &mut LinkedList<String>) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let first = parts[0];
    match first {
        "af" => {
            match parts.get(1) {
                Some(str) => fruits.push_front(str.to_string()),
                None => panic!("Not fruit provided"),
            };
        }
        "ab" => {
            match parts.get(1) {
                Some(str) => fruits.push_back(str.to_string()),
                None => panic!("No fruit provided"),
            };
        }
        "rf" => {
            let fruit = fruits.pop_front().unwrap();
            println!("{}", fruit);
        }
        "rb" => {
            let fruit = fruits.pop_back().unwrap();
            println!("{}", fruit);
        }
        _ => panic!("Invalid command"),
    };
}
fn main() {
    let mut fruit: LinkedList<String> = LinkedList::new();
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList
    isn't a common operation in practice. I included
    it in this example to keep the code as similar as possible
    to the original VecDeque example.
     */

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // // Add fruits to the both ends of the list after shuffling
    // fruit.push_front("Pomegranate");
    // fruit.push_back("Fig");
    // fruit.push_back("Cherry");

    println!(
        "Type \
            1) 'stop' to exit or \
            2) 'af <fruit>' to add to front of deque or\n \
            3) 'ab <fruit>' to add to back of deque or\n \
            4) 'rf' to remove from front or\n \
            5) 'rb' to remove from back\n \
            6) 'd <index>' to remove at index\n \
             Examples: \n
             ab apples - to add apples to back end\n \
             af oranges - to add oranges to front end\n "
    );
    let _ = io::stdout().flush();

    let mut input = String::new();
    loop {
        println!("{:?} > ", &fruit);
        let _ = io::stdout().flush();
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let trimmed = input.trim();
        if trimmed == "stop" {
            break;
        } else {
            parse_cmd(trimmed.to_string(), &mut fruit);
        }
    }

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
    println!();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    let choice = fruit.choose(&mut rand::thread_rng()).unwrap();
    println!("selected random fruit: {}", choice);
}
