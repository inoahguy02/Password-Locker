use std::io::stdin;

fn main() {
    loop {
        println!("Welcome to Password-Locker\n\
                Please choose an option below:\n\
                1. Create account\n\
                2. Log in");

        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let input = buf.trim();

        if input == "1" {
            break;
        } else if  input == "2" {
            break;
        } else {
            println!("Please type in either 1 or 2");
            continue;
        }
    }
}
