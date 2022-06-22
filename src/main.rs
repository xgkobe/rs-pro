// use std::io;
use std::cmp::Ordering;
// use rand::Rng;

fn main() {
//    let tup: (i32, f64, u8) = (500, 6.4, 1);
//    let (x, y, z) = tup;

   let s = String::from("hhah");
   compare_big(21);
   cons_str(&s);
   println!("{},", s);
}

fn cons_str (str: &String) {
    println!("{}", str);
}


fn compare_big (x: i32) {
   let mut number = 3;
   while number != 0 {
       println!("{}!", number);
       number = number - 1;
    }
    let i = 32;
    match x.cmp(&i) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You win!");
        }
    }

    // let a = [1,2,3];
    for ele in (i..33).rev() {
        println!("the value is: {}", ele);
    }

    let s1 = String::from("xuguang");
    let s2 = s1.clone();
    println!("s1:{}, s2:{}", s1, s2);
}
