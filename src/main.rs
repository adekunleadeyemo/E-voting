use std::collections::HashMap;
use std::io;
//use e_voting::election::{Election, Candidate};
use e_voting::election::Election;
use e_voting::voter::Voter;

fn main() {
    let mut elections: HashMap<String, Election> = HashMap::new();
    let mut voters: HashMap<String, Voter> = HashMap::new();

    loop {
        clear_screen();
        println!("Welcome to E-voting!");
        println!("Choose an option:");
        println!("1. Electoral Officer Login");
        println!("2. Voters Login");
        println!("3. Exit");

        let option: i32 = get_user_input("option");

        match option {
            1 => {
                if authenticate_officer() {
                    println!("Authentication successful!");

                    loop {
                        clear_screen();
                        println!("Choose an option:");
                        println!("1. Create new election");
                        println!("2. Register new voter");
                        println!("3. Open election for voting");
                        println!("4. Close election");
                        println!("5. Tally votes");
                        println!("6. Exit");

                        let officer_option: i32 = get_user_input("officer_option");

                        match officer_option {
                            1 => create_election(&mut elections),
                            2 => register_voter(&mut voters),
                            3 => open_election(&mut elections),
                            4 => close_election(&mut elections),
                            5 => tally_votes(&mut elections),
                            6 => {
                                println!("Exiting...");
                                break;
                            }
                            _ => println!("Invalid option, please try again."),
                        }
                    }
                } else {
                    println!("Authentication failed. Exiting...");
                }
            }
            2 => {
                if authenticate_voter(&mut voters) {
                    println!("Authentication successful!");

                    loop {
                        clear_screen();
                        println!("Choose an option:");
                        println!("1. Cast ballot");
                        println!("2. Exit");

                        let voter_option: i32 = get_user_input("voter_option");

                        match voter_option {
                            //1 => cast_ballot(&mut elections, &mut voters),
                            1 => cast_ballot(&mut elections),
                            2 => {
                                println!("Exiting...");
                                break;
                            }
                            _ => println!("Invalid option, please try again."),
                        }
                    }
                } else {
                    println!("Authentication failed, voter does not exist. Exiting...");
                }
            }
            3 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H"); // Clear screen
}

fn get_user_input(prompt: &str) -> i32 {
    println!("Enter {}: ", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse().expect("Invalid input")
}

extern crate e_voting;
use e_voting::authentication::{authenticate_officer, authenticate_voter};
use e_voting::election::{create_election, register_voter, open_election, close_election, tally_votes};
use e_voting::ballot::cast_ballot;
