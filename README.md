---

---

# RustProjects

```rust
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

```

## Rust常识

### 变量与可变性

- 声明变量使用`let`关键字
- 默认情况下，变量是不可变的(Immutable)
- 声明变量时 在变量前加上`mut` 就可以使变量可变

### 常量与变量
- 常量 常量在绑定值以后也是不可变的， 但它与不可变的变量有很多区别
  - 不可以使用`mut` 常量永远不可变
  - 声明常量使用`const`关键字 **其类型必须被标注**
  - 常量可以在任何作用域进行声明 包括全局作用域
  - 常量只可以绑定到常量表达式 无法绑定到函数调用的结果或只能运行时才能计算出来的值

- 在程序运行的期间常量在其声明的作用域中一直有效
- 命名规范

``` rust
const MAX_POINTS:u32 = 100_000;
```

### Shadowing(屏蔽)

- 可以使用相同名字声明新的变量 新的变量就会屏蔽(shadow)之前申明的同名变量























