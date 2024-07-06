use std::collections::HashMap;

fn remove_item(s: &str, arr:&mut Vec<&str>) {
    arr.retain(|x| *x!=s);
}

fn main() {
    // Create a vector of fruits.
    // let fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    // println!("Original fruit salad: {:?}", fruit_salad);

    // Uncommenting the following line will cause a compilation error because fruit_salad is immutable.
    //fruit_salad.push("figs");

    // To mutate the vector, we need to declare it as mutable:
    let mut fruit_salad = vec!["plums","oranges","apple", "banana", "cherry", "dates", "elderberries", "strawberries", "papaya"];
    println!("Initial list of fruits: {:?}", fruit_salad);
    fruit_salad.push("figs");
    println!("Modified fruit salad after adding figs: {:?}", fruit_salad);
    fruit_salad.pop();
    println!("Modified fruit salad after doing pop(): {:?}", fruit_salad);
    remove_item("apple", &mut fruit_salad);
    println!("After trying to remove apple: {:?}", fruit_salad);
    fruit_salad.extend(["banana","cherry", "cherry","papaya","cherry"]);
    fruit_salad.sort();
    let mut stats:HashMap<&str, usize> = HashMap::new();
    println!("Printins items after doing sort()");
    for fruit in fruit_salad {
        println!("{fruit}");
        let stat = stats.entry(fruit).or_insert(0);
        *stat += 1;
    }
    println!("--- after counting occurrences ----");
    for (fruit, stat) in stats {
        println!("{fruit}: {stat}");
    }

}

