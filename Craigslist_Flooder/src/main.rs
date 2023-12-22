use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;
use std::time::Duration;
use std::thread;
use std::fs;
use rand::prelude::*;
use std::error::Error;
use csv::{ReaderBuilder, Trim};
use std::fmt::Display;

fn main() {

   _make_email();

    //println!("test: {}", _ran_nicknames());

    /*
    let username = _ran_name;
    println!("username {}", username);
    let mut fake_email = String::new();


    fake_email.push_str("test");
    println!("fake_email: {}", fake_email);
    */
}

fn _make_email() {
    let mut email: &str;
    let mut rng = thread_rng();

    let mut data_point_amount: u32 = 0;
    let mut loop_counter: u32 = 0;
    let mut name: bool = false;
    let mut date: bool = false;
    let mut hobbie: bool = false;
    let mut nickname: bool = false;
    
    // determins how many data points should be in the email
    let random: u32 = rng.gen_range(0..=100);
    match random {
        0..=50 => data_point_amount = 2,
        51..=75 => data_point_amount = 3,
        76..=100 => data_point_amount = 4,
        101_u32..=u32::MAX => todo!(),
    }
    println!("data_point_amount: {}", data_point_amount);
    while true {
        let data_point: u32 = rng.gen_range(0..=100);
        match data_point {
            0..=40 => {
                if name != true {
                    name = true;
                    loop_counter +=1;
                }
            }
            41..=60 => {
                if date != true {
                    date = true;
                    loop_counter +=1;
                }
            }
            61..=80 => {
                if hobbie != true {
                    hobbie = true;
                    loop_counter +=1;
                }
            }
            81..=100 => {
                if nickname != true {
                    nickname = true;
                    loop_counter +=1;
                }
            }
            101_u32..=u32::MAX => todo!(),
        }
        if data_point_amount == loop_counter {
            println!("name: {} date: {} hobbie: {} nickname: {}", name, date, hobbie, nickname);
            println!("loop_counter: {}", loop_counter);
            break;
        }
    }
    /*
    let email_name: &str = &_ran_name();
    let email_date: &str = &_ran_date();
    let email_hobbie: &str = &_ran_hobbies();
    let email: &str = &(email_name.to_owned() + email_date + email_hobbie);
    println!("email: {:#?}", email);
    */
    
    let mut rng = StepRng::new(2, 13);
    let mut irs = Irs::default();
    
    let mut input = vec![1, 2, 3, 4, 5];
    
    irs.shuffle(&mut input, &mut rng);
    assert_eq!(&input, &[4, 1, 5, 3, 2]);
    for (index, &input) in input.iter().enumerate() {
        // Do something with each index and number
        println!("Index: {}, Number: {}", index, input);
    }
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

fn _ran_nicknames() -> String  {

    let rdr = ReaderBuilder::new()
    .from_path("/home/luke/Rust/cli_programs/Craigslist_Flooder/Craigslist_Flooder/data/Nicknames.csv");
    if let Some(result) = rdr.expect("REASON").records().choose(&mut rand::thread_rng()) {
        let message = result.as_ref().expect("REASON").as_slice().to_owned();
        println!("{}", message);
        message
    } else {
        "No hobbies found".to_owned()
    }
}