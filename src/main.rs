use std::collections::HashMap;
// String是对Vec<u8>的包装
/*
push_str: 添加字符串
push:添加单个字符
+：字符串拼接
*/
fn main() {
    panic!("crash and burn");
    let mut v = vec![1,2,3];
    for i in &mut v {
        *i += 50;
        println!("{:?}", i);
    };
    let He = String::from("qeq");
    let name = String::from("xuguang");
    let s = format!("{}-{}", He, name);
    println!("{}", s);
    let age = String::from("h").len();
    println!("{}", age); 
    // let row = vec![
    //     SpreadssheetCell::Int(3),
    //     SpreadssheetCell::Text(String::from("blue")),
    //     SpreadssheetCell::Float(10.12),
    // ];
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Bulue"), 10); 
    hash_map(); 
}

fn hash_map() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}

enum SpreadssheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}