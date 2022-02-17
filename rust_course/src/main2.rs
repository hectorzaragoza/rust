use std::mem;
fn main() {
// Arithmetic

    let mut a = 2+3*4; // Operators follow precedence (Order of Operations)
    println!("{}", a);
    a = a+1; // Rust does not support -- or ++ operators
    a -= 2;
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a,3);
    println!("{} cubed is {}", a, a_cubed);
    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{}  cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

// Bitwise, only available for integers
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
    println!("1|2 = {}", c); // This results in 3. Because 01 or 10 = 11 == 3_10
    
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

// Logical operators
    let pi_less_4 = std::f64::consts::PI < 4.0;
    // > <= >= ==, there are no === in Rust
    let x = 5;
    let x_is_5 = x == 5;
    println!("Output of x_is_5 = {}", x_is_5);
}