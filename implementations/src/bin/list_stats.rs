use std::io::{self, Read};
use std::collections::HashMap;

/*
 * Accepts a set of integers via stdin (one int per line) and prints statistics based on the input.
 * 
 * To run: cargo run --bin list_stats < data/listStatsDataSmall.txt
 */
fn main() {
    println!("Please use stdin to provide a set of integers (one integer on each line)");
    let mut buffer: String = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Failure while reading from stdin");
    
    let lines = buffer.lines();
    let mut nums: Vec<i32> = Vec::new();

    for line in lines {
        let num: i32 = line.parse().expect(&format!("The following line could not be converted to an int: \n{}\n", line)[..]);
        nums.push(num)
    }

    if nums.len() == 0 {
        panic!("You must provide at least one valid integer as input!");
    }

    nums.sort();

    let mut num_count: HashMap<i32, i32> = HashMap::new();
    let mut total: i32 = 0;
    let mut mode: i32 = 0;
    let mut mode_count: i32 = 0;

    for num in nums.iter() {
        total += num;
        let count = num_count.entry(*num).or_insert(0);
        *count += 1;
        
        if *count > mode_count {
            mode_count = *count;
            mode = *num;
        }
    }
    let average = total as f32 / nums.len() as f32;
    let median: i32 = nums[nums.len() / 2];

    println!("\n---STATS---\n");
    println!("Average: {}\nMedian: {}\nMode: {}", average, median, mode);
}