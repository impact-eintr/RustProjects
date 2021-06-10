use std::io; // prelude
use rand::Rng; // trait
use std::cmp::Ordering;

fn main() {
    println!("猜数");

    // 定义一个随机不可变量
    let secret_number = rand::thread_rng().gen_range(1..101);


    loop{
        println!("猜测一个数");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        // 类型遮蔽
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal =>{
                println!("You Win");
                break;
            } ,
        }
    }


}
