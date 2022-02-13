// fn main() {
//     let list = vec![100,34,72,55];
//     let first_two = &list[0..2];
//     println!("first two are {:?}", first_two);
//     println!("list is {:?}", list);
//     println!("again, first two: {:?}", first_two);
// }

// The lifetime for list (value) starts when it is declared and ends at the end of the function at the close bracket.
// The lifetime for the reference to list (first_two) starts when it's declared
// and ends before the end of the function. list lifetime is longer than that for first_two.

fn main() {
    let list = vec![100,34,72,55];
    {
        let first_two = &list[0..2];
        println!("first two are {:?}", first_two);
    }
    
    println!("list is {:?}", list);
    // Lifetime error for line below because first_two below is not found within the scope because
    // it starts and dies within the brackets inside main() brackets.
    // println!("again, first two: {:?}", first_two);
}