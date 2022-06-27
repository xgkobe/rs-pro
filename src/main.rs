use std::fs::File;

enum result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let f = File::open("./lib.rs");
    println!("{}是帅哥", "彭城");
}