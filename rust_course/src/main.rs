// Structs
struct Point {
    x: f64,
    y: f64
}
// Structures can use structures
fn structures() {
    let p = Point {
        x: 3.0, y: 4.0
    };
    println!("Point p is at ({},{})", p.x, p.y);
}

fn main() {
    structures();
}