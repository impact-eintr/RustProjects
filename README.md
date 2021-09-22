# Rust学习笔记

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

# Rust常识

## 变量与可变性

- 声明变量使用`let`关键字
- 默认情况下，变量是不可变的(Immutable)
- 声明变量时 在变量前加上`mut` 就可以使变量可变

## 常量与变量
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

## Shadowing(屏蔽)

- 可以使用相同名字声明新的变量 新的变量就会屏蔽(shadow)之前声明的同名变量

``` rust
fn main() {
    let x = 5;
    let x = x + 1;
    println!("The value of x is {}", x);
}
```

- shadow和把变量标记为`mut`是不一样的：
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

## 数据类型
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

### Tuple
- 可以将多个类型的多个值放到一个类型中
- Tuple的长度是固定的 一旦声明就无法改变

``` rust
fn main() {
    let t :(i32, f64, u8) = (500, 6.4, 1);
    println!("{}, {}, {}", t.0, t.1, t.2);
}

```

### 数组
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

## 函数
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

### 返回值

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

# 控制结构
## if else

``` rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else {6};

    println!("The value of number is: {}", number);
}

```



``` rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);
}

```

``` rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number -1;
    }

    println!("The result is: {}", number);
}

```


``` rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

```

### 使用for循环遍历集合
- 可以使用while或loop来遍历，但是易错且低效
- 使用for循环更简洁紧凑，它可以针对集合中的每个元素来执行一些代码
- 由于for循环的安全、简洁性，所以它在Rust里用的最多

``` rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("END!")
}

```

# 所有权
- 所有权是Rust最独特的特性，它让Rust无需GC就可以保证内存安全。
- 所有程序在运行时都必须管理计算机内存的方式
  - 有些语言有垃圾收集机制，在程序运行时，他们会不断地寻找不再使用的内存。
  - 在其他语言中，程序员必须显式地分配和释放内存
- Rust采用了第三种方式
  - 内存是通过一个所有权来管理的，其中包含一组编译器在编译时检查的规则。
  - 当程序运行时，所有权特性不会减慢程序的运行速度。

## Stack vs Heap
- 在像Rust这样的系统级编程里，一个值是在stack上还是在heap上对语言的行为和你为什么要做某些决定是有更大的影响的

- Stack按值的接收顺序来存储，按相反的顺序将它们移除
  - 添加数据叫做压入栈
  - 移除数据叫做弹出栈
- 所有存储在stack上的数据必须拥有已知的固定的大小
  - 编译时大小未知的数据或者运行时大小可能发生变化的数据必须存放在heap上
- Heap内存组织性差一些
  - 当你把数据放在heap时，你会请求一定数量的空间
  - 操作系统在heap里找到一块足够大的空间，把它标记为在用，并返回一个指针，也就是这个空间的地址。
  - 这个过程叫做在heap上进行分配，有时仅仅称为`分配`

- 把值压到stack上不叫分配
- 因为指针是已知固定大小的，可以把指针存放到stack上
  - 但如果想要实际数据，必须使用指针来定位
- 把数据压到stack上要比heap上分配快的多
  - 因为操作系统不需要寻找用来存储新数据的空间，那个位置永远在stack的顶端。
- 在heap上分配空间需要要做更多的工作
  - 操作系统首先需要找到一个足够大的空间来存放数据，然后要做好记录方便下次分配

### 访问数据
- 访问heap中的数据要比访问stack中的数据慢，因为需要通过指针才能找到heap中的数据
  - 对于现代的处理器来说，由于缓存的缘故，如果指令在内存中跳转的次数越少，那么速度就越快
- 如果数据存放的距离比较近，那么处理器的处理速度就会更快一些(stack)上
- 如果数据之间的距离比较远，那么处理速度就会慢一些(heap)
  - 在heap上分配大量空间也是需要时间的

### 函数调用
- 当你的代码调用函数时，值被传入到函数(也包括指向heap的指针)。函数本地的变量被压到stack上。当函数结束后，这些值会从strack上弹出。

### 所有权存在的原因
- 所有权解决的问题
  - 跟踪代码的那些部分正在使用heap的那些数据
  - 最小化heap上的重复数据
  - 清理heap上未使用的数据以避免空间不足
- 一旦我们明白了所有权，那么就不需要经常去想stack或heap了。
- 单数知道管理heap数据是所有权存在的原因，这有助于解释它为什么会这样工作。

### 所有权规则
- 每个值都有一个变量，这个变量是该值的所有者
- 每个值同时只能有一个所有者
- 当所有者超出作用域(scope)时，该值将被删除

``` rust
fn test() {
    // s不可用
    let s = "hello"; // s可用
    // 可以对s进行相关操作
    println!("{}", s)
} // s作用域到此结束，s不可再用

```

### String类型
- String比那些基础标量数据类型更复杂。
- 字符串字面量:程序里手写的那些字符串值。它们是不可变的。
- Rust的String类型在heap上分配，能够存储在编译阶段未知数据量的文本。

``` rust
fn create_string() {
    let mut s = String::from("Hello");

    s.push_str(", World");

    println!("{}", s)
}

fn main() {
    create_string();
}

```


## 内存和分配
- 字符串字面值，在编译时就知道他的内容了，其文本内容直接被硬编码到最终的可执行文件中
  - 速度快、高效。是因为其不可变性。
  
- String类型，为了支持可变性，需要在heap上分配内存来保存编译时未知的文本内容
  - 操作系统必须在运行时来请求内存
    - 这布通过调用String::from来实现
  - 当用完String之后，需要使用某种方式将内存返回给操作系统 
    - 这步，在拥有GC的语言中，GC会跟踪并清理不再使用的内存
    - 没有GC,就需要我们去识别内存何时不再使用，并调用代码将它返回
      - 如果忘了浪费内存
      - 如果提前做了，变量非法
      - 如果做了两次，同样不允许
- **Rust采用了不同的方式， 对于某个值来说，当拥有它的变量走出作用域范围时，内存会立即自动的还给操作系统**
- drop函数

### 变量与数据交互的方式: Move 移动
**多个变量可以与同一个数据使用一种独特的方式来交互 Move**

- 整数等已知且固定大小的简单的值被压到stack中
- String等复杂类型数据，会发生变量移动

``` rust

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}, {}", s1, s2)
}

```

``` bash
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:14:24
   |
11 |     let s1 = String::from("Hello");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
12 |     let s2 = s1;
   |              -- value moved here
13 |
14 |     println!("{}, {}", s1, s2)
   |                        ^^ value borrowed here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
error: could not compile `var`



```

- 为了保证内存安全：
  - Rust没有尝试复制被分配的内存
  - Rust让s1失效
    - 当s1离开作用域的时候，Rust不需要释放任何东西
    
#### 移动与拷贝
- 浅拷贝
- 深拷贝
- 你也许会将复制指针、长度、容量(上面代码中`let s2 = s1`)，视作浅拷贝，但由于Rust让s1失效了，所以我们使用了一个新的术语：Move
- **隐含的一个设计原则 Rust不会自动创建数据的深拷贝**
  - 就运行时性能而言，任何自动赋值的操作搜是廉价的

### 变量与数据的交互方式 克隆
- 如果真想对heap上的String数据进行深度拷贝，而不仅仅是stack上的数据，可以使用clone方法

``` rust
fn main() {
    create_string();
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{}, {}", s1, s2)
}

```

### Copy trait 
- Copy trait 可以用于像整数这样完全存放在stack上的类型
- 如果一个类型实现了Copy这个trait 那么就的变量在赋值后仍然可用
- 如果一个类型或者该类型的一部分实现了Drop trait，那么Rust不允许让它再去实现Copy trait

#### 一些拥有Copy trait的类型
- 任何简单标量的组合类型都是可以Copy的
- 任何需要分配内存或某种资源的都不是Copy的
- 一些拥有Copy trait的类型
  - 所有的整数类型，例如u32
  - bool类型
  - char
  - 所有的浮点类型，例如f64
  - Tuple(元组)，如果其所有的字段都是Copy的
    - (i32,i32)是
    - (i32,String)不是


### 所有权与函数

``` rust
fn main() {
    let s1 = String::from("Hello");
    take_owership(s1);
    //println!("{}", s1);
    let x = 6;

    makes_copy(x);
    println!("{}", x)

}

fn take_owership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

```

被注释的行如果执行就会报错
``` bash
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:4:20
  |
2 |     let s1 = String::from("Hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     take_owership(s1);
  |                   -- value moved here
4 |     println!("{}", s1);
  |                    ^^ value borrowed here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
error: could not compile `var`

```

### 返回值与作用域

- 函数在返回值的过程中，同样也会发生所有权的转移

``` rust
fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

```


- 一个变量的所有权总是遵循同样的模式：
  - 把一个值赋给其他变量时就会发生移动
  - 当一个包含heap数据的变量离开作用域时，他的值就会被drop函数清理，除非数据的所有权移动到另一个变量上了
  
## 引用和借用

``` rust
fn main() {
    let s1 = String::from("Hello");
    let len = calcute_length(&s1);

    println!("The length of '{}' is {}", s1, len);
}

fn calcute_length(s: &String) -> usize {
    s.len()
}

```

- 参数的类型是&String 而不是 String
- & 符号就表示引用：允许你引用某些值而不屈的其所有权

### 借用
- 我们把引用作为函数参数这个行为叫做借用
- 不可以修改借用的东西
- 和变量一样，引用默认也是不可变的

### 可变引用

``` rust
fn main() {
    let mut s1 = String::from("Hello");
    let len = calcute_length(&mut s1);

    println!("The length of '{}' is {}", s1, len);
}

fn calcute_length(s: &mut String) -> usize {
    s.push_str("world");
    s.len()
}

```

### 一个重要限制
**可变引用有一个重要的限制：在特定的作用域内，对某一块数据，只能有一个可变的引用。**

``` rust
fn main() {

    let mut s = String::from("Hello");
    let s1 = &mut s;
    let s2 = &mut s;

    println!("The length of '{}' is {}.", s1, s2);
}

```


``` rust
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let s1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let s2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("The length of '{}' is {}.", s1, s2);
  |                                           -- first borrow later used here

error: aborting due to previous error

```

这样做的好处是可以在编译时防止数据竞争

以下三种行为会发生数据竞争:
- 两个或多个指针同时访问同一个数据
- 至少有一个指针用于写入数据
- 没有使用任何机制来同步对数据的访问

**可以通过创建新的作用域，来允许非同时地创建多个可变引用**

``` rust
fn main() {

    let mut s = String::from("Hello");
    {
        let s1 = &mut s;
    println!("The length of '{}' is {}.", s1, s1.len());
    }
    let s2 = &mut s;

    println!("The length of '{}' is {}.", s2, s2.len());
}

```


### 另一个限制
- **不可以同时拥有一个可变引用和一个不可变引用**
  - 多个不可变引用是可以的

``` rust
fn main() {

    let mut s = String::from("Hello");
    let s1 = &s;
    let s2 = &s;
    let r1 = &mut s;


    println!("{} {} {}", s1,s2, r1);
}

```

``` rust
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let s1 = &s;
  |              -- immutable borrow occurs here
5 |     let s2 = &s;
6 |     let r1 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
...
9 |     println!("{} {} {}", s1,s2, r1);
  |                          -- immutable borrow later used here

error: aborting due to previous error

```


### 悬空指针
- 一个指针引用了我内存中的某个地址，而这块内存可能已经释放并分配给其他人用了
- 在Rust中，编译器保证引用永远都不是悬空引用
  - 如果你引用了某些数据，编译器将保证在引用离开作用域之前不会离开作用域
  
``` rust
fn main() {
    let r = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

```

``` rust
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ^^^^^^^^

error: aborting due to previous error
```


### 引用的规则
- 在任何给定的时刻，只能满足下列条件之一
  - 一个可变的引用
  - 任意数量不可变的引用
- 引用必须一直有效


## 切片
- Rust的另一种不持有所有权的数据类型： 切片
- 一道题，编写一个函数
  - 它接收字符串作为参数
  - 返回他在这个字符串中找到的第一个单词
  - 如果函数没找到任何空格，那么整个字符串就被返回

``` rust
fn main() {
    let mut s = String::from("Hello world");
    let word_index = first_world(&s);

    s.clear();
    println!("{}", word_index);
}


fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

```


``` rust
fn main() {
    let s = String::from("Hello world");

    let h = &s[0..5];
    let w = &s[6..11];

    // 或者也可以这样写
    let h = &s[..5];
    let w = &s[6..];
    
    println!("{}, {}", h, w)

}

```



``` rust
fn main() {
    let mut s = String::from("hello world");
    let word_index = first_world(&s);

    s.clear();
    println!("{}", word_index)
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

```

``` rust
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:5
  |
3 |     let word_index = first_world(&s);
  |                                  -- immutable borrow occurs here
4 |
5 |     s.clear();
  |     ^^^^^^^^^ mutable borrow occurs here
6 |     println!("{}", word_index)
  |                    ---------- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `slice` due to previous error

```

#### 字符串字面量是切片
- 字符串字面值是直接存储在二进制出程序中
- let s = "Hello, World";
- 变量s的类型是&str, 所以字符串字面值也是不可变的

#### 将字符串切片作为参数传递
- `fn first_world(s: &String) -> &str`
- 有经验的Rust开发者会采用&str作为参数类型，因为这样就可以同时接收String和&st类型的参数了
  - 使用&str直接调用该函数
  - 使用String 可以创建一个完整的String切片来调用该函数
- 定义函数时使用字符串切片来代替字符串引用会使我们的API更加通用，且不会有任何功能性损失


``` rust
fn main() {
    let s = String::from("hello world");
    let word_index = first_world(&s[..]);

    println!("{}", word_index)
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

```

### 其他类型的切片

``` rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    for (_, item) in slice.iter().enumerate() {
    println!("{}",item);
    }
}

```


# struct
## 定义并实例化struct
- struct 结构体
  - 自定义的数据类型
  - 为相关联的值命名，打包 => 有意义的组合

``` rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user = User{
        email: String::from("66666666@gmail.com"),
        username: String::from("eintr"),
        active: true,
        sign_in_count:1,
    };
    user.active = false;
    println!("Hello, {}", user.username);
}

```

**注意，一旦struct是可变的，那么实例中的所有字段都是可变的**


### Struct作为函数的返回值

``` rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user = new_user(&String::from("23333333@gmail.com")[..], &String::from("Eintr")[..]);
    user.active = false;
    println!("Hello, {}", user.username);
}

fn new_user(email: &str, username: &str) -> User {
    User {
        email.to_string(),
        username.to_string(),
        active:true,
        sign_in_count:1,
    }
}

```

### Struct更新语法

``` rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = new_user(&String::from("23333333@gmail.com")[..], &String::from("Eintr")[..]);
    let user2 = User {
        username:String::from("eintr"),
        ..user1
    };
    println!("Hello, {} and {}", user1.username, user2.username);
}

fn new_user(email: &str, username: &str) -> User {
    User {
        email:email.to_string(),
        username:username.to_string(),
        active:true,
        sign_in_count:1,
    }
}

```

## Tuple struct
- 可定义类似tuple的struct，叫做tuple struct
  - Tuple struct 整体有个名，但里面的元素没有名
  - 适用：想给整个tuple起名，并让它不同于其他tuple,而且又不需要给每个元素起名
- 定义tuple struct:使用struct关键字，后边是名字，以及里面元素的类型


``` rust
fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

```

## Unit-Like Struct (没有任何字段)
- 可以定义没有任何字段的struct，叫做Unit-Like struct 因为与`()` 单元类型类似
- 适用于需要在某个类型上实现某个trait，但是在里面又没有想要存储的数据

## struct数据的所有权
``` rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

```

- 这里的字段使用了String而不是&str
  - 该struct实例拥有其所有的数据
  - 只要struct实例是有效的，那么里面的字段数据也是有效的
- struct里也可以存放引用，但这需要使用生命周期
  - 生命周期保证只要struct实例是有效的，那么里面的引用也是有效的

``` rust
struct User {
    username: &str,
}

fn main() {
    let user = User{
      username: "eintr",
    };
}

```

``` rust
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:15
  |
2 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 | struct User<'a> {
2 |     username: &'a str,
  |

```


## Struct的一个例子

``` rust
#[derive(Debug)]
struct Rectangle {
    width:u32,
    length:u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length:50,
    };
    println!("{}", area(&rect));
    println!("{:#?}", rect);
}

```

## Struct 方法

``` rust
#[derive(Debug)]
struct Rectangle {
    width:u32,
    length:u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length:50,
    };
    println!("{}", rect.area());
    println!("{:#?}", rect);
}

```

- 在impl块里定义方法
- 方法的第一个参数可以之&self，也可以是获得其所有权或可变借用。和其他参数一样
- 更好的代码组织

### 方法调用的运算符
- C/C++ pniect->something() 和 (*object).something()一样
- Rust没有-> 运算符
- Rust会自动引用或解引用
  - 在调用方法时就会发生这种行为
- 在调用方法时，Rust根据情况自动添加&，&mut或者*，以便object可以匹配方法的签名
- 下面两行代码的效果相同
  - p1.distance(&p2)
  - (&p1).distance(&p2)

``` rust
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


}

fn main() {
    let rect = Rectangle {
        width: 30,
        length:50,
    };
    let test = Rectangle {
        width: 20,
        length: 49,
    };
    if rect.can_hold(&test) {
        println!("{:#?}", test);
    }
    println!("{}", rect.area());
    println!("{:#?}", rect);
}

```

### 关联函数
- 可以在impl块里定义不把self作为第一个参数的函数，它们叫关联函数(不是方法)
  - 例如String::from()
- 关联函数通常用于构造器
- ::符号
  - 关联函数
  - 模块创建的命名空间
``` rust
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

```


# 枚举与模式匹配
## 定义枚举

``` rust
enum IpAddrKind{
  v4,
  v6,
}
```

``` rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V6);
}

fn route(_ip_kind: IpAddrKind) {
    println!("Hello, world!");
}

```

### 将数据附加到枚举的变体
- `
enum IpAddr {
  V4(String),
  V6(String),
}
`
- 优点
  - 不需要额外使用struct
  - 每个变体可以拥有不同的类型以及关联的数据量

``` rust
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));

}

```


## Option枚举
- 在其他语言中
  - Null是一个值，它表示”没有值“
  - 一个变量可以处于两种状态：空值(null),非空
- Null引用：Billion Dollar Mistake
- Null的问题在于：当你尝试使用非NULL值那样使用Null值的时候，就会引起某种错误
- Null的概念还是有用的，因某种原因而变为无效或缺失的值

### Rust中类似Null概念的枚举
- 定义在标准库中
- `enum Option<T> {
  Some(T),
  None,}`

``` rust
fn main() {
    let some_number = Some(5); // i32
    let some_string = Some("A String"); // &str
    let absent_number: Option<i32> = None; // 必须制指定类型
    println!("Hello, world!");
}

```


``` rust
fn main() {
    let x: i8 = 5;
    let y:Option<i8> = Some(5);

    let sum = x + y;
}
```


``` rust
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`


```

## 强大的控制流运算符 match
- 允许一个值与一系列模式进行匹配，并执行匹配的模式对应的代码
- 模式可以是字面值、变量名、通配符………

``` rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("test");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 20,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Hello, {}!", value_in_cents(Coin::Penny));
}

```

### 绑定值的模式
- 匹配的分支可以绑定到被匹配对象的部分值
  - 因此可以从enum变体中提取值

``` rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}



fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("test");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 20,
        Coin::Quarter(state) => {
            println!("A quarter from {:?}", state);
            25
        },
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("Hello, {}!", value_in_cents(c));
}

```

### 匹配Option<T>

``` rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    let five = Some(5);
    let _ = plus_one(five);
    let _ = plus_one(None);
}

```

**match匹配必须穷举所有的可能**

``` rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
fn main() {
    let five = Some(5);
    let _ = plus_one(five);
    let _ = plus_one(None);
}

```


``` rust
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:2:11
    |
2   |     match x {
    |           ^ pattern `None` not covered
    |
   ::: /home/eintr/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:501:5
    |
501 |     None,
    |     ---- not covered
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `Option<i32>`

```


## if let
- 处理只关心一种匹配而忽略其他匹配的情况
- 更少的代码，更少的缩进，更少的模板代码
- 放弃了穷举的可能

``` rust
fn main() {
    let mut v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => println!("others"),
    }

    v = Some(3);

    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}

```


# Package, Crate, Module
## Package Crate 定义Module
- 代码组织主要包括
  - 那些细节可以暴露，那些细节是私有
  - 作用域内那些名称有效
- 模块系统
  - Package: Cargo的特性，让你构建、测试、共享crate
  - Crate: 一个模块树，它可产生一个library 或 可执行文件
  - Module: use: 让你控制代码的组织、作用域、私有路径
  - Path: 为struct function 或 module等项命名的方式
  
``` bash
.
├── Cargo.toml
└── src
    └── main.rs

```
  
``` toml
[package]
name = "package_eg"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

```
 
### cargo的惯例
- src/main.rs
  - binary crate的crate root
  - crate名与package名相同
- src/lib.rs
  - package包含一个library crate
  - library crate 的crate roots
  - crate名与package 名相同
- Cargo把crate root 文件交给rustc来构建library或binary
- 一个package可以同时包含src/main.rs和src/lib.rs
  - 一个binary crate,一个library crate
  - 名称与package名相同
- 一个Package可以有多个binary crate
  - 文件放在src/bin
  - 每个文件都是单独的binary crate

### Crate的作用
- 将相关功能组合到一个作用域内，便于在项目间进行共享
  - 防止冲突
- 例如 rand crate 访问它的功能需要通过它的名字 rand

### 定义module来控制作用域和私有性
- Module
  - 在一个crate中，将代码进行分组
  - 增加可读性，易于复用
  - 控制项目(item)的私有性。public、private
- 建立module
  - mod关键字
  - 可嵌套
  - 可包含其他项(struct enum 常量 trait 函数等)的定义

``` rust
mod front_of_hose {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

```

## Path

``` rust
fn serve_order() {}

mod  back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        crate::serve_order();
    }
    fn cook_order() {}
}

```

### pub struct
- pub 放在 struct 前
  - struct是公共的
  - struct的字段默认是私有的
- struct的字段需要单独设置成pub来变成公有

``` rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit:  String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    meal.seasonal_fruit = String::from("test");
}

```

### Pub enum
公共枚举里的所有变体都是公共的
``` rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit:  String::from("peaches"),
            }
        }
    }
}


```

## use关键字
- 可以使用use关键字将路径导入到作用域内
  - 仍遵守私有性规则，只有公共的部分引入才可以使用

``` rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

```

### use的习惯用法
- 函数：将函数的伏击模块引入作用域
- struct enum 其他 指定完整路径(指定到本身)

``` rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map[&1]);
}

```


``` rust
use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IOResult;

fn f1() -> Result {}

fn f2() -> IOResult {}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map[&1]);

}

```

### 使用pub use 重新导出名称
- 使用use将路径导入到作用域内后，该名称在此作用域内是私有的。
- pub use 重导出
  - 将条目引入作用域
  - 该条目了一被外部代码引入到他们的作用域

``` toml
[package]
name = "use_eg"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.5.5"

```

``` rust
use rand::Rng;

fn main() {

}

```

- 标准库也被当作外部包
  - 不需要修改CArgo.toml来包含std
  - 需要使用use将std中的特定条目引入当前作用域

### 嵌套引用

``` rust
use std::{cmp::Ordering, io}

fn main() {}
```

``` rust
use std::io::{self, Write};

fn main() {}
```



## 将模块内容移动到其他文件
- 模块定义时，如果模块名后边是`;`，而不是代码块
  - Rust会从模块同名的文件中加载内容
  - 模块树的结构不会发生变化
- 随着模块逐渐变大，该技术让你可以把模块的内容移动到其他文件中

# 常用的集合
## Vector
- Vector<T> 叫做vector
  - 由标准库提供
  - 可存储多个值
  - 只能存储相同类型的数据
  - 值在内寻中连续存放

``` rust
fn main() {
    let mut v1: Vec <i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v1.push(22);
}

```

``` rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("{}", third),
        None => println!("There is no third element"),
    }
}

```

``` rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    match v.get(100) {
        Some(third) => println!("{}", third),
        None => println!("There is no third element"),
    }
}
```

## String


## HashMap

# 



# 


# 


# 


# 


# 


# 



# 


# 



# 



# 



# 


# 












