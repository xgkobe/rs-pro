#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

enum IpAddrKind {
    v4(u8, u8, u8, u8),
    v6(String), 
}


fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", area(&rect));
    println!("{:?}", rect);
    println!("{:#?}", rect);
    println!("{:#?}", rect.area());
    println!("{:#?}", rect.can_hold(&rect));
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
    let some_number = Some(5);
    println!("{:?}", some_number);

}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
