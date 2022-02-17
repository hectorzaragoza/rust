// Control Flow
// fn if_statement() {
//     let temp = 19;
//     if temp > 30 {
//         println!("It is so hot outside!")
//     } else if temp < 10 {
//         println!("Really cold!");
//     } else {
//         println!("Temp is fine...")
//     }
//     // Variable declaration with an if-else statement
//     let day = if temp > 20 {"sunny"} else {"cloudy"};
//     println!("Today is {}", day);
//     println!("It is {}",
//         if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"ok"});
// }

// fn while_and_loop() {
//     let mut x = 1;

//     while x < 1000 {
//         x *=2;
//         // continue will take us out of the loop and back to the loop iteration after this one
//         if x == 64 {
//             continue;
//         }
//         println!("X is = {}", x);
//     }

//     let mut y = 1;
//     loop {
//         y *= 2;
//         println!("y = {}", y);
//         // break statement kicks us out of the loop.
//         // 1 << 10 is the same as 2^10
//         if y == 1 << 10 {
//             break;
//         }
//     }
// }

// fn for_loop() {
//     // Different than other programming languages
//     // 1..11 range, 1 inclusive, 11 exclusive
//     for x in 1..11 {
//         println!("X is {}", x);
//     }
//     // To get values AND index
//     for (pos, y) in (30..41).enumerate() {
//         println!("index is {} and value is {}", pos, y);
//     }
// }
// fn main() {
//     // if_statement();
//     // while_and_loop();
//     for_loop();
// }

// MATCH STATEMENT
// fn main() {
//     let country_code = 44;

//     let country = match country_code {
//         44 => "uk",
//         46 => "Sweden",
//         7 => "Russia",
//         // = makes this range inclusive of 1000
//         1..=1000 => "Unknown",
//         _ => "invalid"
//     };
//     println!("THe country with code {} is {}", country_code, country);
// }

// use rand::Rng;
use std::io::stdin;

// Enumeration, basically a list of possible values
enum State {
    Locked,
    Failed,
    Unlocked
}
// Combination Lock
fn main() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => {
                        continue;
                    }
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("Failed");
                entry.clear(); // Makes it an empty string
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("Unlocked");
                return;
            }
        }
    }
}