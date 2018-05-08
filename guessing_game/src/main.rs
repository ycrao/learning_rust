extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secrect_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secrect_number);

    loop {  // 循环

        println!("Please input your guess.");

        let mut guess = String::new();  // mutable 可改变的

        io::stdin().read_line(&mut guess)
            .expect("Fail to read line");


        // 转换类型 u32 表示无符号的32位值
        /*
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
        */
        // 异常处理
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);  // {}是占位符

        // 比较
        match guess.cmp(&secrect_number) {  // 枚举类型
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
