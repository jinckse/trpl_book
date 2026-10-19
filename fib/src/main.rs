use std::io;

fn main() {
    println!("Fibonacci Sequence!");
    println!("Please input number:");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

    for n in 0..=num {
        print!("{} ", fib(n));
    }
}

fn fib(n: u32) -> u32 {
    let m: u32;
    match n {
        0 | 1 => m = 1,
        _ => m = fib(n - 1) + fib(n - 2)
    };

    return m;
}
