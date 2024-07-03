/* VecDeque is a double-ended queue, which means you can push and pop from both ends of the queue
 * */

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    println!("{:?}", fruit);
    /*
    // Shuffle the fruit
    let mut rng = thread_rng();
    let mut fruit:<Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);
    */
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    println!("Fruit salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            print!("{}", item);
        }
    }
    println!();

}
