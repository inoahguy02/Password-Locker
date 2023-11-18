mod creds;

use std::error::Error;
use std::io::stdin;
use creds::Type;

fn main() {
    println!("Welcom to Password-Locker");
    loop {
        println!("Please choose an option below:\n\
                1. Create account\n\
                2. Log in\n\
                3. Exit");

        let input = get_input();

        if input == "1" {
            println!("Creating an account will delete a previously made account. Continue?(y/n)");
            let input = get_input();
            if input != "Y" && input != "y" {
                continue;
            }

            println!("Please enter a master password:");
            let input = get_input();
            match create_account(input) {
                Ok(()) => println!("Acccount created successfully"),
                Err(e) => {
                    println!("Account creation failed: {}", e.to_string());
                    continue;
                }
            }
        } else if  input == "2" {
            println!("Please enter the master password:");
            let input = get_input();

            match login(input) {
                Ok(()) => {},
                Err(e) => {
                    println!("Login failed: {}", e);
                    continue;
                }
            }
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
            }
        }
    }
}

fn create_account(pass: String) -> Result<(), Box<dyn Error>> {
    let hashedpw = creds::hash(pass)?;
    creds::store(hashedpw, Type::Hash)?;

    Ok(())
}

fn login(pass: String) -> Result<(), Box<dyn Error>> {
    let attempted_hash = creds::hash(pass);
    let stored_hash = creds::get_hash();

    /*if attempted_hash != stored_hash {
        return Err("Password is incorrect".into());
    }*/

    Ok(())
}

fn get_input() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap(); // TODO: handle unwrap
    buf.trim().to_string()
}

fn print_passwords() {
    // let file = (Open file)

    /*let counter = 0;
    for line in file {
        println!("{}: {}", counter, creds::decrypt(line));
        counter += 1;
    }*/
}