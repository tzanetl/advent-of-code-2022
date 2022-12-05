use std::fs;
use std::path::Path;


fn read_input_file() -> String {
    let filepath = Path::new("input.txt");
    println!("Input file: {:?}, exists: {:?}", filepath, filepath.exists());
    let content = fs::read_to_string(filepath).expect("unable to read message file");
    return content;
}


fn main() {
    let input_string = read_input_file();
    let mut total_calories: i32 = 0;
    let mut calories: i32;
    let mut elves: Vec<i32> = Vec::new();

    for line in input_string.lines() {
        calories = match line.parse() {
            Err(..) => {
                elves.push(total_calories);
                total_calories = 0;
                continue;
            }
            Ok(c) => c
        };
        total_calories += calories
    }

    // println!("{:?}", elves);
    let max_value = elves.iter().max();
    match max_value {
        Some(max) => println!("Max calories: {}", max),
        None => println!("No elves found!")
    }
    elves.sort();
    let top_3_calories: i32 = elves.iter().rev().take(3).sum();
    println!("Calories on top 3 elves: {}", top_3_calories)
}

