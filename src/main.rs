use rand::{TryRngCore, rngs::OsRng};
use std::io;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
          abcdefghijklmnopqrstuvwxyz\
          0123456789\
          !@#$%^&*()-_=+[]{}<>?/";

fn main() {
    println!("Enter password length: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let length: usize = input.trim().parse().unwrap_or(12);

    let password = generate_password(length);

    println!("Password: {}", password);
}

fn generate_password(length: usize) -> String {
    let mut rng = OsRng;
    let mut password = String::with_capacity(length);

    for _ in 0..length {
        let random = rng.try_next_u32().unwrap() as usize;
        let idx = random % CHARSET.len();

        password.push(CHARSET[idx] as char);
    }

    password
}
