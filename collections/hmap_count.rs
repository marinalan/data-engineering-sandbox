/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;
use std::io::{self, Write};

fn take_vector() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    return arr;
}

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {
    //let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    println!("Enter list of numbers as string, with numbers separated by spaces");
    let _ = io::stdout().flush();
    let numbers = take_vector();

    let mut result = logic(numbers);
    result.sort_by(|a, b| b.1.cmp(&a.1));
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
}

