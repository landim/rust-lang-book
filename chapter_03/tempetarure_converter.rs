use std::io;

fn main() {
    println!("This is a temperature converter from F to C!");
    loop {
        println!("Input a temperature in F:");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("error");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let celsius:f32 = (temp - 32.0) * (5.0 / 9.0);

        println!("{temp}F -> {celsius:.2}C");
    }
}
