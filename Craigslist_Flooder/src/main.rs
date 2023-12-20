use std::fs;
use rand::prelude::*;
use std::error::Error;
use csv::{ReaderBuilder, Trim};
use std::fmt::Display;

fn main() {

   // _ran_hobbies();

    println!("test: {}", _ran_hobbies());

    /*
    let username = _ran_name;
    println!("username {}", username);
    let mut fake_email = String::new();


    fake_email.push_str("test");
    println!("fake_email: {}", fake_email);
    */
}

fn _ran_name() -> String {
    let mut name = String::new();
    let file_path = "/home/luke/Rust/cli_programs/Craigslist_Flooder/Craigslist_Flooder/data/Popular_Baby_Names.csv";

    let name_list = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            println!("Error reading the file '{}'", file_path);
            return String::new();
        }
    };

    // Split the string by commas
    let mut fields = name_list.split(',');

    let mut rng = thread_rng();
    let num = rng.gen_range(0..57580) * 5;
    // Skip lines
    for _ in 0..num {
        if let Some(_) = fields.next() {
            // Continue skipping
        } else {
            // Handle case where there are not enough commas
            println!("Not enough commas in the input string.");
            return String::new();
        }
    }

    // Skip everything before the 3rd comma
    for _ in 0..3 {
        if let Some(_) = fields.next() {
            // Continue skipping
        } else {
            // Handle case where there are not enough commas
            println!("Not enough commas in the input string.");
            return String::new();
        }
    }

    // Grab everything until the 4th comma
    if let Some(temp_name) = fields.next() {
        name = temp_name.to_string();
        println!("Result: {}", name);
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
            return String::new();
        }
    }
    name
}

fn _ran_date() -> String {
    let date: i32;

    let mut rng = thread_rng();

    let millennium: i32;
    let decade: i32;

    let coinflip = rng.gen();
    match coinflip {
        true => 
            millennium = 1900,
        false => 
            millennium = 2000,
    }
    match coinflip {
        true => 
            decade = rng.gen_range(70..=99),
        false => 
            decade = rng.gen_range(0..=23),
    }

    date = millennium + decade;
    println!("date: {}", date);
    date.to_string()
}

fn _ran_domain() -> String {
    let mut domain = String::new();
    let mut rng = thread_rng();

    let random: u32 = rng.gen_range(0..=100);
    match random {
        0..=7 => domain = String::from("@comcast.net"), // 7%
        8..=17 => domain = String::from("@aol.com"), // 11%
        18..=63 => domain = String::from("@gmail.com"), // 47% %
        64..=86 => domain = String::from("@yahoo.com"), // 24%%
        87..=100 => domain = String::from("@outlook.com"), // 10%
        101_u32..=u32::MAX => panic!(),
    }
    domain
}

fn _ran_hobbies() -> String {

    let rdr = ReaderBuilder::new()
    .from_path("/home/luke/Rust/cli_programs/Craigslist_Flooder/Craigslist_Flooder/data/hobbies.csv");
    if let Some(result) = rdr.expect("REASON").records().choose(&mut rand::thread_rng()) {
        let message = result.as_ref().expect("REASON").as_slice().to_owned();
        println!("{}", message);
        message
    } else {
        "No hobbies found".to_owned()
    }
    //println!("hobbie: {}", hobbie);
}

