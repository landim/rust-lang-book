use std::io;

fn main() {
    println!("This is a Fibonacci calculator");
    loop {
        println!("Input the Nth:");
        let mut nth = String::new();
        io::stdin().read_line(&mut nth).expect("error");
        let nth: i32 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut fib:u32 = 1;
        let mut fib_last:u32 = 0;

        if nth > 48 {
            println!("Nth should be <= 48");
            continue;
        }
        
        if nth == 1 {
            println!("fib({nth}th) = 0");
            continue;
        }
        if nth == 2 {
            println!("fib({nth}th) = 1");
            continue;
        }

        for _ in 2..nth {
            let temp:u32 = fib;
            fib = fib + fib_last;
            fib_last = temp;
        }
        println!("fib({nth}th) = {fib}");
    }
}
