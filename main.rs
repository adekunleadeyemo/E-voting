// importing necessary libraries and modules
use argon2::password_hash::{PasswordHash, PasswordVerifier};
use argon2::Argon2;
use csv::ReaderBuilder;
use std::{env, io, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // check if correct number of arguments was passed
    if args.len() != 2 {
        eprintln!("Usage: {} db.csv", args[0]);
        std::process::exit(1);
    }

    // check if csv file is valid or exists
    let file_path = &args[1];
    if !fs::metadata(file_path).is_ok() {
        eprintln!("Error! Password database not found!");
        std::process::exit(1);
    }

    // creates variable to read CSV
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(file_path)?;

    // prompts user for username and password and stores them
    // also takes out any trailing newline or whitespace
    let mut username = String::new();
    println!("Enter username: ");
    io::stdin().read_line(&mut username).expect("Enter Username fail");
    username = username.trim_end().to_owned();
    //let username = username.trim();

    let mut password = String::new();
    println!("Enter password: ");
    io::stdin().read_line(&mut password).expect("Enter Password fail");
    password = password.trim_end().to_owned();
    //let password = password.trim();

    // create variable to check if username was found or not
    let mut found = false;

    // iterates through the CSV and extracts username and password hash string
    for result in rdr.records() {
        let record = result?;
        
        if record[0] == username {
            found = true;

            // gets the password hash string for the username and 
            // verifies input password against it using Argon2
            let parsed_hash = match PasswordHash::new(&record[1]) {
                Ok(hash) => hash,
                Err(_) => {
                    println!("Temporary system issue preventing login. Please try again later. If the issue persists, contact support.");
                    std::process::exit(1);
                },
            };
            if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() {
                println!("Access granted!");
            } else {
                println!("Error! Access denied!");
            }
            break;
        }
    }

    
    if !found {
        println!("Error! Access denied");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, io, fs};
    //use std::env::args;

#[test]
fn csv_exists() {
    assert!(fs::metadata("db.csv").is_ok());
}
#[test]
fn username_saved() {
    let db_user = "admin";
    //let db_pass = "$argon2id$v=19$m=19456,t=2,p=1$difPUw5AhyFN/URJZ0IY8g$VDC5PPK0Lx8IeI6LttXQ90zL3BuH/AAQV1ndGEovpPY
    //";
    println!("Enter username: ");    
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Enter Username fail");
    username = username.trim_end().to_owned();
    assert_eq!(db_user,username);
}
// #[test]
// fn get_password_hash() {
//     let db_user = "admin";
//     let db_pass = "$argon2id$v=19$m=19456,t=2,p=1$difPUw5AhyFN/URJZ0IY8g$VDC5PPK0Lx8IeI6LttXQ90zL3BuH/AAQV1ndGEovpPY
//     ";
//     let parsed_hash = match PasswordHash::new(db_pass) {
//         Ok(hash) => hash,
//         Err(_) => {
//             println!("Temporary system issue preventing login. Please try again later. If the issue persists, contact support.");
//             std::process::exit(1);
//         }

        
//     };
//     assert!(parsed_hash);
// }
}


