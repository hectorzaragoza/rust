#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;
struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap() {
    // Stored in stack
    let p1 = origin();
    // Stored on heap
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    // f64 takes up 8 bytes, so p1 takes up 16 bytes, 8 for each coordinate (usize)
    // p2 takes up 8 bytes because it is storing an address, that is 8 bytes

    // Box value back to the stack?
    // p3 the value of the box (p2) on the stack
    let p3 = *p2;
    println!("{}", p3.x);
}