use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies: HashMap<i32, u32> = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result: Vec<(i32, u32)> = Vec::new();
    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }
    result
}

fn main() {
    let numbers: Vec<i32> = vec![1,2,3,4,5,6,4,4,9,1,3];
    let result: Vec<(i32,u32)> = logic(numbers);

    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );

    //let mut fruit_calories = HashMap::new();

    //fruit_calories.insert("apple", 95);

    //println!("Apple calories: {}", fruit_calories["apple"]);
}
