// // Ownership with regards to a FUNCTION
// fn says(s: String) {
//     println!("I say, {}!", s)
// }

// fn main() {
    
//     // Ownership with regards to a STRING variable

//     // println!("Introduction to Ownership and Borrowing in Rust");
//     // let mut a = String::from("hello");
//     // a.push_str(" there!");
//     // println!("I say, {}!", a);
//     // We are creating a string with String::from, making it mutable, and concatenating to our 
//     // initial string via push_str method. At the end of the curly bracket. Variable a goes out of
//     // scope and it gets cleaned up.

//     // In this case, when we call the says function, ownership gets transferred from a to s, and
//     // it gets cleaned up at the end of the says function.
//     // let a = String::from("hello");
//     // says(a);

//     // Cloning
//     let a = String::from("hello");
//     says(a.clone());
//     println!("using a's clone: {}", a)
// }

// fn main() {
//     let s = String::from("book");
    
//     // Add code here that calls the pluralize function
//     let pl = pluralize(s.clone());
    
//     println!(
//         "I have one {}, you have two {}",
//         s,
//         pl
//     );
// }

// // Add appropriate parameters, return values, and implementation to this function
// fn pluralize(singular: String) -> String {
//     singular + "s"
// }

// BORROWING
// struct Person {
//     name: String,
// }

// fn congratulate(person: &Person) {
//     print!("Congratulations, {}!!!", person.name);
// }

// fn main() {
//     let p = Person {
//         name: String::from("Jake"),
//     };

//     congratulate(&p);
//     println!("Can still use p here: {}", p.name);
// }

// SLICES
fn main() {
    let v = vec![10,20,30];
    let v_slice = &v[..];
    println!("V_slice is: {:?}", &v_slice);
}