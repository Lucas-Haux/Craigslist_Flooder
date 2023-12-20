use std::fs;
use rand::prelude::*;

fn main() {
    let username = grab_date;
    println!("username {}", username().to_string());
    let mut fake_email = String::new();

    fake_email.push_str("test");
    println!("fake_email: {}", fake_email);
}

fn _grab_name() {
    let file_path = "/home/luke/Rust/cli_programs/Craigslist_Flooder/Craigslist_Flooder/data/Popular_Baby_Names.csv";

    let name_list = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            println!("Error reading the file '{}'", file_path);
            return;
        }
    };

    // Split the string by commas
    let mut fields = name_list.split(',');

        // Skip everything before the 3rd comma
        for _ in 0..3 {
            if let Some(_) = fields.next() {
                // Continue skipping
            } else {
                // Handle case where there are not enough commas
                println!("Not enough commas in the input string.");
                return;
            }
        }

        // Grab everything until the 4th comma
        if let Some(result) = fields.next() {
            println!("Result: {}", result);
        } else {
            // Handle case where there are not enough commas
            println!("Not enough commas in the input string.");
        }

        for _ in 0..1 {
            if let Some(_) = fields.next() {
                // Continue skipping
            } else {
                // Handle case where there are not enough commas
                println!("Not enough commas in the input string.");
                return;
            }
        }
}

fn grab_date() -> i32 {
    let date: i32;
    let mut rng = thread_rng();
    let millennium: i32;
    let coinflip = rng.gen();
    let decade: i32;
    match coinflip {
        true => 
            millennium = 1900,
        false => 
            millennium = 2000,
    }
    match coinflip {
        true => 
            decade = rng.gen_range(70..100),
        false => 
            decade = rng.gen_range(0..24),
    }
    date = millennium + decade;
    println!("date: {}", date);
    date
}