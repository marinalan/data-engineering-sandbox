/* A linked list is a doubly linke dlist, which means that each element in the list 
 * has a pointer to the next element and to the previous element
 * A great example when to use a LinkedList is when you need to insert or remove elements from the
 * middle of the list
 * */

use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();

    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

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
