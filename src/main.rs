use std::io;
use std::io::{stdout, Write};
use rand::Rng;

fn main() {
    let rand_num = rand::thread_rng().gen_range(0, 1001);  // [0, 1000];
    println!("子曰く「{}」", rand_num);

    loop {
        let mut input = String::new();
        print!("Please input number: ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\n{} と {} の最大公約数は {} です。", input, rand_num, euclidian_gcd(rand_num, input));
        break;
    };
}

fn euclidian_gcd(mut m: u32, mut n: u32) -> u32 {
    if m <= n {
        let tmp = n;
        n = m;
        m = tmp;
    }
    if n == 0 {
        return m
    }
    euclidian_gcd(n, m % n)
}
