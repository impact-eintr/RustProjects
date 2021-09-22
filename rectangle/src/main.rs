#[derive(Debug)]
struct Rectangle {
    width:u32,
    length:u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width:size,
            length:size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length:50,
    };
    let test= Rectangle::square(20);

    if rect.can_hold(&test) {
        println!("{:#?}", test);
    }
    println!("{}", rect.area());
    println!("{:#?}", rect);
}
