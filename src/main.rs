use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜数");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("生成的随机数是：{}", secret_number);

    loop {
        let mut guss = String::new();
        io::stdin().read_line(&mut guss).expect("无法读取行");
        println!("你猜测的数是：{}", guss);

        let guss: u32 = match guss.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("输入的格式不正确,请重新输入");
                continue;
            }
        };
        match guss.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}
