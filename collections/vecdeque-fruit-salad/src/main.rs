/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;
use std::io::{self, Write};

fn parse_cmd<'a>(cmd: String, fruits: &mut VecDeque<String>) {
  let parts:Vec<&str> = cmd.split_whitespace().collect();
  let first = parts[0];
  match first {
      "af" => { match parts.get(1) {
                      Some(str) =>fruits.push_front(str.to_string()),
                      None => panic!("Not fruit provided")
                }; 
              },
      "ab" => { match parts.get(1) {
                      Some(str) =>  fruits.push_back(str.to_string()),
                      None => panic!("No fruit provided")
                };    
              },
      "rf" => { let fruit = fruits.pop_front().unwrap(); println!("{}", fruit); },
      "rb" => { let fruit = fruits.pop_back().unwrap(); println!("{}", fruit); },
      _ => panic!("Invalid command")
  };
}

fn main() {
    let mut fruit: VecDeque<String> = VecDeque::new();
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruits: VecDeque<_> = fruit.into_iter().collect();

    // // Add fruits to the both ends of the queue after shuffling
    // fruit.push_front("Pomegranate");
    // fruit.push_back("Fig");
    // fruit.push_back("Cherry");

    println!("Type \
            1) 'stop' to exit or \
            2) 'af <fruit>' to add to front of deque or\n \
            3) 'ab <fruit>' to add to back of deque or\n \
            4) 'rf' to remove from front or\n \
            5) 'rb' to remove from back\n \
             Examples: \n
             ab apples - to add apples to back end\n \
             af oranges - to add oranges to front end");
    let _ = io::stdout().flush();

    let mut input = String::new();
    loop {
        println!("{:?} > ", &fruits);
        let _ = io::stdout().flush();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed = input.trim();
        if trimmed == "stop" {
            break;
        } else {
            parse_cmd(trimmed.to_string(), &mut fruits);
        }
    }

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
    println!();
    let mut fruit: Vec<_> = fruits.into_iter().collect();
    let choice = fruit.choose(&mut rand::thread_rng()).unwrap();
    println!("selected random fruit: {}", choice);
}
