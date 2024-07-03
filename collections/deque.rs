fn main() {
    use std::collections::VecDeque;

    // Create deque
    let mut fruit_deque = VecDeque::new();

    // Push back
    fruit_deque.push_back("apple");

    // Push front
    fruit_deque.push_front("cherry");

    println!("Fruit deque: {:?}", fruit_deque);
}
