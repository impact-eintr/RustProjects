use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("创建了一个新文件");
                    fc
                },
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            oe => panic!("Error opening file {:?}", oe),
        }
    };

    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("文件找不到");
}
