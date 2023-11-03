mod creds;

use std::io::stdin;
use creds::Type;

fn main() {
    loop {
        println!("Welcome to Password-Locker\n\
                Please choose an option below:\n\
                1. Create account\n\
                2. Log in\n\
                3. Exit");

        let input = get_input();

        if input == "1" {
            println!("Please enter a master password:");
            let input = get_input();
            create_account(input);
        } else if  input == "2" {
            println!("Please enter the master password:");
            let input = get_input();
            login(input);
        } else if input == "3" {
            break;
        } else {
            println!("Please type in either 1, 2, or 3");
            continue;
        }

        println!("Login successful!");

        loop {
            print_passwords();
            println!("Please choose an option below:\n\
                    1. Add password\n\
                    2. Delete password\n\
                    3. Logout");
            
            let input = get_input();

            if input == "1" {
                println!("Please type the password to add:");
                let input = get_input();
                creds::encrypt(input);
            } else if  input == "2" {
                println!("Please enter the number of the password to delete:");
                let input = get_input();
                creds::remove(input);
            } else if input == "3" {
                break;
            } else {
                println!("Please type in either 1, 2, or 3");
                continue;
            }
        }
    }
}

fn create_account(pass: String) {
    let hashedpw = creds::hash(pass);
    creds::store(hashedpw);
}

fn login(pass: String) {
    let attempted_hash = creds::hash(pass);
    let stored_hash = creds::get(Type::Hash);

    // if attempted_hash == stored_hash, Ok(()). Else, print password wrong. 2 attempts remaining. go back to beginning of loop
}

fn get_input() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap(); // TODO: handle unwrap
    buf.trim().to_string()
}

fn print_passwords() {
    // let passwords: Vec<String>
    // for line in file => decrypt(line).push(passwords)
    // for pwd in passwords => println!("{}: {}", counter, pwd);
}