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

``` go
package main

import (
	"fmt"
	"math/rand"
)

func main() {
	fmt.Println("猜一个数")
	num := rand.Int() % 100

	for {
		c := 0
		_, err := fmt.Scanf("%d", &c)
		if err != nil {
			fmt.Println(err)
			continue
		}

		if c == num {
			fmt.Println("you win")
			break
		} else if c < num {
			fmt.Println("too small")
		} else if c > num {
			fmt.Println("too big")
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

- 可以使用相同名字声明新的变量 新的变量就会屏蔽(shadow)之前声明的同名变量

``` rust
fn main() {
    let x = 5;
    let x = x + 1;
    println!("The value of x is {}", x);
}
```

- shadow和吧变量标记为`mut`是不一样的：
  - 如果不是用let关键字 那么重新给非mut的变量赋值会导致编译时错误
  - 而如果使用let声明的同名新变量 也是不可变的
  - **使用let声明的同名新变量 它的类型可以与之前的不同**

``` rust
fn main() {
    let s = "test";
    let s = s.len();
    println!("The len of s is {}", s);
}
```
> 以下行为是不被允许的
``` rust
fn main() {
    let mut s = "test";
    s = s.len(); // s.len() 返回 usize 即当前机器字长下的无符号整形(u64)
    println!("the len of s is {}", s);
}

```

### 数据类型
- 标量 和 复合类型
- Rust是静态类型 在编译期间必须知道所有变量的类型
  - 基于使用的值 编译器通常能推断出它的具体类型
  - 但如果可能的类型比较多(String转为整数的parse方法)

``` rust
fn main() {
    let guess = "32".parse().expect("Not a number");
    println!("{}", guess)
}

```

``` bash
 --> src/main.rs:2:9
  |
2 |     let guess = "32".parse().expect("Not a number");
  |         ^^^^^ consider giving `guess` a type


```

``` rust
fn main() {
    let guess : u32 = "3g".parse().expect("Not a number");
    println!("{}", guess)
}

```

### 标量类型
- 一个标量类型代表一个单个的值
- Rust有 4 种主要的标量类型
  - 整数类型
  - 浮点类型
  - bool类型
  - 字符类型

#### 整数类型
|length|signed|unsigned|
|-|-|-|
|8|i8|u8|
|16|i16|u16|
|32|i32|u32|
|64|i64|u64|
|128|i128|u128|
|arch|isize|usize|
> 整数字面值

|进制|例子|
|----|----|
|10|23_444|
|16|0xff|
|8|0o77|
|2|0b1111_0000|
|Byte(u8 olny|b'A'|

- 除了byte类型外 所有的数值字面值都允许使用类型后缀
- 整数默认类型就是i32

> 整数溢出(以下内容存疑)

u8的范围是0 - 255, 如果把一个u8变量的值设为256
- Debug mode 编译会发生 panic
- Relase mode (--release) 编译会发生一般意义上的溢出

> 一定可行的解决方案
``` rust
use std::num::Wrapping;

fn main() {
    let a = Wrapping(255u8);
    let one = Wrapping(1u8);

    let num = a + one;
    println!("{}", num);
    assert_eq!(u8::MIN, (a + one).0);
}

```

#### 浮点类型(IEEE-754)
- f32
- f64 (默认类型 精度高 速度不慢)

``` rust

fn main() {
    let x : f32 = 1.0;
    let y  = 2.0E-10;
}
```

#### bool类型 (1 byte)
- true
- false

#### 字符类型
- char (4 byte)
- '🤔'
- 是Unicode标量值

### 复合类型
- 符合类型可以将多个值放到一个类型里面
- Rust提供了两种基础的复合类型
  - Tuple
  - Array

#### Tuple
- 可以将多个类型的多个值放到一个类型中
- Tuple的长度是固定的 一旦声明就无法改变

``` rust
fn main() {
    let t :(i32, f64, u8) = (500, 6.4, 1);
    println!("{}, {}, {}", t.0, t.1, t.2);
}

```

#### 数组
- 数组也可以将多个值放到一个类型里
- 数组中每个元素的类型必须相同
- 数组的长度也是固定的

``` rust
fn main() {
    let a = [1,2,3,4,5];
    println!("{}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
    let a = [1; 5];
    println!("{}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
}

```

> 数组的作用

**数组放在栈上**

> 越界检查

rust 会在编译和运行时进行越界检查 panic

### 函数
- Rust是一个基于表达式的语言

``` rust
fn main() {
    test_function();
}

fn test_function() {
    println!("Tis is another function")
}
```

``` rust
fn main() {
    test_function(5)
}

fn test_function(x : i32) {
    println!("Tis is another function, The value of x is {}", x)
}

```


``` rust
fn main() {
    test_function();
}

fn test_function() {
    let x = 5;

    let y = {
        let x = 1;
        x + 3;
    };

    println!("The value of ty is {}", y);
}

```

``` bash
error[E0277]: `()` doesn't implement `std::fmt::Display`
  --> src/main.rs:13:39
   |
13 |     println!("The value of ty is {}", y);
   |                                       ^ `()` cannot be formatted with the default formatter
   |

```

> `()` 是一种类型 唯一的值就是`()`

#### 返回值

``` rust
fn main() {
    println!("{}",test_function());
}

fn test_function() -> i32{
    5
}

```

> 以下的写法是不被允许的
``` rust
fn main() {
    println!("{}",test_function());
}

fn test_function() -> i32{
    5;
}


```

``` bash
fn main() {
    println!("{}",test_function());
}

fn test_function() -> i32{
    5;
}

```

``` rust
fn main() {
    println!("{} 看了视频：“嗯，和在家里一样”",test_function());
}

fn test_function() -> char {
    '👽'
}
```
















