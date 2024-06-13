use std::fs;
use rand::prelude::*;
use csv::{ReaderBuilder};

fn main() {
    let mut i = 0;
    while i < 100 {
        println!("email: {}, and password: {}" , _make_email(), _make_password());
        //println!("password: {}" , _make_password());
        i += 1;
    };

}

fn _make_password() -> String {
    let mut password: String = Default::default();
    let mut rng = thread_rng();

    let mut data_point_amount: u32 = 0;
    let mut loop_counter: u32 = 0;
    let mut name: bool = false;
    let mut date: bool = false;
    let mut hobbie: bool = false;
    let mut nickname: bool = false;
    
    // determins how many data points should be in the password
    let random: u32 = rng.gen_range(0..=100);
    match random {
        0..=50 => data_point_amount = 2,
        51..=75 => data_point_amount = 3,
        76..=100 => data_point_amount = 4,
        101_u32..=u32::MAX => todo!(),
    }
    loop {
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
            break;
        }
    }

    let mut rng = thread_rng();
    let mut y = [1, 2, 3, 4];
    y.shuffle(&mut rng);
    for y in y {
        match y {
            1 => 
            if name == true {
                password.push_str(&_ran_name());
            },
            2 =>
            if date == true {
                password.push_str(&_ran_date());
            },
            3 =>
            if hobbie == true {
                password.push_str(&_ran_hobbies());
            },
            4 => 
            if nickname == true {
                password.push_str(&_ran_nicknames());
            },
            i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!(),
        }
    }
    password
}

fn _make_email() -> String {
    let mut email: String = Default::default();
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
    loop {
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
            break;
        }
    }

    let mut rng = thread_rng();
    let mut y = [1, 2, 3, 4];
    y.shuffle(&mut rng);
    for y in y {
        match y {
            1 => 
            if name == true {
                email.push_str(&_ran_name());
            },
            2 =>
            if date == true {
                email.push_str(&_ran_date());
            },
            3 =>
            if hobbie == true {
                email.push_str(&_ran_hobbies());
            },
            4 => 
            if nickname == true {
                email.push_str(&_ran_nicknames());
            },
            i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!(),
        }
    }

    email.push_str(&_ran_domain());
    email
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
    
    let mut rng = thread_rng();
    let style = rng.gen_range(1..4);
    match style {
        1 => name = name.to_uppercase(),
        2..=3 => name = uppercase_first_letter(&name),
        2..=4 => name = name.to_lowercase(),
        i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!(),
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
    date.to_string()
}

fn _ran_domain() -> String {
    let mut domain = Default::default();
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
    let mut hobbie: String = Default::default();
    let rdr = ReaderBuilder::new()
    .from_path("/home/luke/Rust/cli_programs/Craigslist_Flooder/Craigslist_Flooder/data/hobbies.csv");
    if let Some(result) = rdr.expect("REASON").records().choose(&mut rand::thread_rng()) {
        let message = result.as_ref().expect("REASON").as_slice().to_owned();
        hobbie = message;
    };

    let mut rng = thread_rng();
    let style = rng.gen_range(1..=4);
    match style {
        1 => hobbie = hobbie.to_uppercase(),
        2..=3 => hobbie = uppercase_first_letter(&hobbie),
        4 => hobbie = hobbie.to_lowercase(),
        i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!(),
    }
    hobbie
}

fn _ran_nicknames() -> String  {
    let mut nickname: String = Default::default();

    let rdr = ReaderBuilder::new()
    .from_path("/home/luke/Rust/cli_programs/Craigslist_Flooder/Craigslist_Flooder/data/Nicknames.csv");
    if let Some(result) = rdr.expect("REASON").records().choose(&mut rand::thread_rng()) {
        let message = result.as_ref().expect("REASON").as_slice().to_owned();
        nickname = message;
    };

    let mut rng = thread_rng();
    let style = rng.gen_range(1..=4);
    match style {
        1 => nickname = nickname.to_uppercase(),
        2..=3 => nickname = uppercase_first_letter(&nickname),
        4 => nickname = nickname.to_lowercase(),
        i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!(),
    }
    nickname

}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}