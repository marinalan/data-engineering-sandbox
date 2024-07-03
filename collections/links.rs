fn main() {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");

    // Sets
    println!("\n\tSets:");
    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");

    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let v = vec![0, 2, 4, 6];
    println!(" Getting element at index 1: {}", v[1]);

    use std::collections::VecDeque;
    let mut deq = VecDeque::from([-1, 0, 1]);
    deq.push_back(3);
    deq.push_back(4);
    deq.push_back(5);
    deq.push_back(6);
    assert_eq!(deq.front(), Some(&(-1)));
    deq.push_front(8);
    deq.push_front(10);
    assert_eq!(deq.front(), Some(&10));
    assert_eq!(deq.len(), 9);
    println!("{:?}", deq);

    let mut deque: VecDeque<u32> = VecDeque::new();
    assert_eq!(deque.len(), 0);
    deque.push_front(1);
    deque.push_back(3);
    println!("{:?}", deque);

    let mut buf = VecDeque::new();
    buf.push_back(1);
    buf.push_back(2);
    buf.push_back(3);
    assert_eq!(buf, [1, 2, 3]);

    assert_eq!(buf.remove(1), Some(2));
    assert_eq!(buf, [1, 3]);

    use std::collections::LinkedList;
    // LinkedList can be nitialized from an array
    let mut list = LinkedList::from([1, 2, 3]);
    match list.front_mut() {
        None => {}
        Some(x) => *x = 12,
    };
    assert_eq!(list.front(), Some(&12));
    println!("list after front_mut: {:?}", list);

    // The LinkedList allows pushing and popping elements at either end in constant time.
    // It is almost always better to use Vec or VecDeque because array-based containers are generally faster, more memory efficient, and make better use of CPU cache.
    let mut list1 = LinkedList::new();
    list1.push_back('a');

    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    list1.append(&mut list2);
    let mut iter = list1.iter();
    assert_eq!(iter.next(), Some(&'a'));
    assert_eq!(iter.next(), Some(&'b'));
    assert_eq!(iter.next(), Some(&'c'));
    assert!(iter.next().is_none());

    assert!(list1.contains(&'a'));
    assert_eq!(list1.contains(&'d'), false);

    assert!(list2.is_empty());
    println!(
        "after appending list 2 to list1 : list1 = {:?}, list2 = {:?}",
        list1, list2
    );

    use std::collections::HashMap;
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    for key in map.keys() {
        println!("{key}");
    }
    for val in map.values_mut() {
        *val = *val + 10;
    }

    for val in map.values() {
        println!("{val}");
    }

    let mut stats: HashMap<&str, i32> = HashMap::new();
    stats.insert("health", 65);
    stats.insert("strength", 80);
    stats.insert("caution", 100);
    for (indicator, val) in stats {
        println!("{}: {}", indicator, val);
    }

    use std::collections::BTreeMap;
    let mut movie_reviews = BTreeMap::new();

    movie_reviews.insert("Office Space", "Deals with real issues in the workplace.");
    movie_reviews.insert("Pulp Fiction", "Masterpiece.");
    movie_reviews.insert("The Godfather", "Very enjoyable.");
    movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");

    // check for a specific one.
    if !movie_reviews.contains_key("Les Misérables") {
        println!(
            "We've got {} reviews, but Les Misérables ain't one.",
            movie_reviews.len()
        );
    }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    movie_reviews.remove("The Blues Brothers");
    // Look up the value for a key (will panic if the key is not found).
    println!("Movie review: {}", movie_reviews["Office Space"]);
    for (movie, review) in &movie_reviews {
        println!("{}: {}", movie, review);
    }

    use std::collections::HashSet;
    let mut books = HashSet::new();

    // Add some books.
    books.insert("A Dance With Dragons".to_string());
    books.insert("To Kill a Mockingbird".to_string());
    books.insert("The Odyssey".to_string());
    books.insert("The Great Gatsby".to_string());

    // Check for a specific one.
    if !books.contains("The Winds of Winter") {
        println!(
            "We have {} books, but The Winds of Winter ain't one.",
            books.len()
        );
    }

    // Remove a book.
    books.remove("The Odyssey");

    // Iterate over everything.
    for book in &books {
        println!("{book}");
    }

    use std::collections::BTreeSet;
    let mut set: BTreeSet<i32> = BTreeSet::new();
    set.insert(3);
    set.insert(5);
    set.insert(8);

    use std::collections::BinaryHeap;
    // A priority queue implemented with a binary heap. max-heap
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(2);
    heap.push(5);
    assert_eq!(heap.peek(), Some(&5));

    println!("Heap: {:?}", heap);
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), None);

    // We can clear the heap of any remaining items.
    heap.clear();

    // The heap should now be empty.
    assert!(heap.is_empty())
}

