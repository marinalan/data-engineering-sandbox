use rand::seq::SliceRandom;
use rand::thread_rng;
//use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::env;

fn generate_fruit() -> &'static str {
    let fruits = [
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
    ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}

fn main() {
    let exclude = env::args().skip(1).next();
    let discard = match exclude {
        Some(str) => { println!("entered fruit to be excluded: {}", str); str},
        None => { println!("no fruit to be excluded from randomly generated");  "none".to_string()}
    };
    /*
    let fruits = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
    ];
    */
    let amounts = [5, 10, 30, 40];


    //let mut rng = thread_rng();

    for amount in amounts.iter() {
        let mut fruit_set = BTreeMap::new();
        //let mut fruit_set = BTreeSet::new();
        //let mut fruit_set = Vec::new();
        /*
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {
        */
        for _ in 0..*amount {
            //fruit_set.insert(fruit);
            //fruit_set.push(fruit);
            let fruit = generate_fruit();
            if fruit != discard.as_str() {
                let stat = fruit_set.entry(fruit).or_insert(0);
                *stat +=1;
                // if fruit_set.len() >= *amount {
                //     break;
                // }
            }
        }
        //fruit_set.remove(discard.as_str());
        print!("{amount}: [");
        for (index,fruit) in fruit_set.keys().rev().enumerate() {
            print!("{fruit}:{}", fruit_set[fruit] );
            if index < fruit_set.len()-1 {
                print!(", ");
            }
        }
        print!("]");
        println!();
    }
}
