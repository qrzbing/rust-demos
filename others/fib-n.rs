use std::io;

fn main() {
    // println!("Please input n");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            std::process::exit(0)
        }
    };
    for i in 0..(n+1) {
        println!("F({}): {}", i, fib(i));
    }
}

fn fib(d: i64) -> i64 {
    if d == 0 || d == 1 {
        d
    } else {
        fib(d - 2) + fib(d - 1)
    }
}
