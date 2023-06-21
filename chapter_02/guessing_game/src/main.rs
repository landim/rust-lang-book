use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Input a number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess is {guess}");
        match guess.cmp(&secret_number) {
           Ordering::Less => println!("Too small"),
           Ordering::Greater => println!("Too big"),
           Ordering::Equal => {
               println!("Right!");
               break;
           },
        };
    }
    println!("random number = {secret_number}");
}
