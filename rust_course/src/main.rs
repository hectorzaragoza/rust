// Structs
struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

// Structures can use structures
fn structures() {
    let p = Point {
        x: 3.0, y: 4.0
    };
    println!("Point p is at ({},{})", p.x, p.y);
    let p2 = Point {
        x: 5.0, y: 10.0
    };

    let myline = Line {start: p, end: p2};
    println!("This is my line, it uses structs within a struct: {}", myline);
}

fn main() {
    structures();
}