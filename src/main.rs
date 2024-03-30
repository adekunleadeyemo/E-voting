use std::collections::{HashMap, HashSet};
use std::io;
use std::process::Command;
use bcrypt::{hash, verify};

struct Election {
    name: String,
    open: bool,
    // office: Offices,
    candidates: Vec<Candidate>
}

// enum Offices {
//     President,
//     Governor,
//     Secretary
// }

struct Candidate {
    name: String,
    party: String,
    vote: i32
}

struct Voter {
    name: String,
    dob: String,
    voted: bool,
}

impl Candidate {
    fn new(name: String, party: String) -> Self {
        Candidate {
            name,
            party,
            vote: 0
        }
    }


    fn update_vote(&mut self) {
        self.vote = self.vote +1;
    }
}

impl Election {
    fn new(name: String) -> Self {
        Election {
            name,
            open: false,
            // office: Offices::President,
            candidates:Vec::new()
        }
    }

    // fn set_office(&mut self, office: Offices) {
    //     self.office = office;
    // }

    fn set_candidates(&mut self, candidate: Candidate) {
        self.candidates.push(candidate)
    }
    fn is_open(&self) -> bool {
        self.open
    }
}

fn main() {
    let mut elections: HashMap<String, Election> = HashMap::new();
    let mut voters: HashMap<String, Voter> = HashMap::new();
    loop {
   	print!("\x1B[2J\x1B[1;1H"); // Clear screen
        println!("Welcome to E-voting!");
        println!("Choose an option:");
        println!("1. Electoral Officer Login");
        println!("2. Voters Login");
        println!("3. Exit");

        let mut option = String::new();
                io::stdin().read_line(&mut option).expect("Failed to read input");
                let option: i32 = option.trim().parse().expect("Invalid input");
    
        match option {
            1 => {
                if authenticate_officer() {
                    println!("Authentication successful!");
    
                    loop {
	                print!("\x1B[2J\x1B[1;1H");
                        println!("Choose an option:");
                        println!("1. Create new election");
                        println!("2. Register new voter");
                        println!("3. Open election for voting");
                        println!("4. Close election");
                        println!("5. Tally votes");
                        println!("6. Exit");
            
                        let mut option = String::new();
                        io::stdin().read_line(&mut option).expect("Failed to read input");
                        let option: i32 = option.trim().parse().expect("Invalid input");
            
                        match option {
                            1 => {
                                create_election(&mut elections);
                            }
                            2 => {
                                register_voter(&mut voters);
                            }
                            3 => {
                                open_election(&mut elections);
                            }
                            4 => {
                                close_election(&mut elections);
                            }
                            5 => {
                                tally_votes(&mut elections);
                            }
                            6 => {
                                println!("Exiting...");
                                break;
                            }
                            _ => {
                                println!("Invalid option, please try again.");
                            }
                        }
                    }
                }
                else {
                    println!("Authentication failed. Exiting...");
                }
            }
            2 => {
                if authenticate_voter(&mut voters) {
                    println!("Authentication successful!");
    
                    loop {
                        print!("\x1B[2J\x1B[1;1H");
                        println!("Choose an option:");
                        println!("1. Cast ballot");
                        println!("2. Exit");
            
                        let mut option = String::new();
                        io::stdin().read_line(&mut option).expect("Failed to read input");
                        let option: i32 = option.trim().parse().expect("Invalid input");
            
                        match option {
                            1 => {
                                cast_ballot(&mut elections, &mut voters);
                            }
                            2 => {
                                println!("Exiting...");
                                break;
                            }
                            _ => {
                                println!("Invalid option, please try again.");
                            }
                        }
                    }
                }
                else {
                    println!("Authentication failed, voter does not exist Exiting...");
                }
            }
            3 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }


    
}
fn authenticate_officer() -> bool {
    println!("Enter administrator username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read input");

    println!("Enter administrator password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read input");

    let admin_username = "admin";
    let admin_password = "$2y$10$nKH4AeD7fM3E9B42MJyuJ.EnHC2LEMb8pNTX0zvIVYIt3dlKhPTOe";


if username.trim() == admin_username {
        match verify(password.trim(), admin_password) {
            Ok(valid) => valid,
            Err(_) => false,
        }
    } else {
        false
    }
}

fn authenticate_voter(voters:&mut HashMap<String, Voter>) -> bool {
    println!("Enter voter name:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read input");

    println!("Enter voter date of birth:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read input");

    // Add authentication logic here
    // For simplicity, let's assume a hardcoded username and password
    if let Some(voter) = voters.get_mut(username.trim()) {
        voter.dob ==  password.trim()
    }
    else {
        false
    }
    
}

fn create_election(elections: &mut HashMap<String, Election>) {
    println!("Enter the name of the election:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    let mut election = Election::new(name.trim().to_string());

    println!("Enter the number of candidates:");
    let mut no_candidate = String::new();
    io::stdin().read_line(&mut no_candidate).expect("Failed to read input");

    while election.candidates.len() < no_candidate.trim().parse().unwrap() {
        println!("Enter the name of Candidate:");
        let mut candidate_name = String::new();
        io::stdin().read_line(&mut candidate_name).expect("Failed to read input");

        println!("Enter the name of Candidate Party:");
        let mut candidate_party = String::new();
        io::stdin().read_line(&mut candidate_party).expect("Failed to read input");

        let mut candidate = Candidate::new(candidate_name,candidate_party);

        election.set_candidates(candidate);
    }

    elections.insert(name.trim().to_string(), election);
    println!("Election '{}' created successfully!", name.trim());
}
//making voter a global variable to change voted
    //let mut name = String::new();
    //let mut dob = String::new();
    //let mut voter = Voter {
    //name: name.trim().to_string(),
    //dob: dob.trim().to_string(),
   // voted: false,
//};

fn register_voter(voters: &mut HashMap<String, Voter>) {
    println!("Enter voter's name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter voter's date of birth (YYYY-MM-DD):");
    let mut dob = String::new();
    io::stdin().read_line(&mut dob).expect("Failed to read input");

    let voter = Voter {
        name: name.trim().to_string(),
        dob: dob.trim().to_string(),
        voted: false,
    };

    voters.insert(name.trim().to_string(), voter);
    println!("Voter '{}' registered successfully!", name.trim());
}

fn open_election(elections: &mut HashMap<String, Election>) {
    println!("Enter the name of the election to open:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    if let Some(election) = elections.get_mut(name.trim()) {
        election.open = true;
        println!("Election '{}' opened for voting.", name.trim());
    } else {
        println!("Election '{}' not found.", name.trim());
    }
}

fn close_election(elections: &mut HashMap<String, Election>) {
    println!("Enter the name of the election to close:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    if let Some(election) = elections.get_mut(name.trim()) {
        election.open = false;
        println!("Election '{}' closed.", name.trim());
    } else {
        println!("Election '{}' not found.", name.trim());
    }
}

fn tally_votes(elections: &mut HashMap<String, Election>) {
    println!("Enter the name of the election to tally votes:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    if let Some(election) = elections.get(name.trim()) {
        // Add logic to tally votes here
        for candidate in election.candidates.iter() {
            print!("\nCandidate: {} Votes: {} \n", candidate.name, candidate.vote);
        }
        println!("Votes tallied for election '{}'.", name.trim());
        
    } else {
        println!("Election '{}' not found.", name.trim());
    }
}

fn cast_ballot(elections: &mut HashMap<String, Election>, voters: &mut HashMap<String, Voter>) {
    //bd
    //println!("Confirm your name:");
    //let mut voter_name = String::new();
    //io::stdin().read_line(&mut voter_name).expect("Failed to read input");

    // Check if the voter exists or add them to the voters list
    //let voter = voters.entry(voter_name.trim().to_string()).or_insert(Voter { 
      //  name: String::new(),
        //dob: String::new(),
        //voted: false });

    //if voter.voted {
      //  println!("You have already cast your vote.");
        //return;
    //}

    println!("Enter the name of the election you want to vote in:");
    let mut election_name = String::new();
    io::stdin().read_line(&mut election_name).expect("Failed to read input");

    if let Some(election) = elections.get_mut(election_name.trim()) {
        if !election.is_open() {
            println!("Election '{}' is not open for voting.", election_name.trim());
            return;
        }

        // Display the ballot
        println!("Election: {}", election_name.trim());
        for (index, candidate) in election.candidates.iter().enumerate() {
            println!("{}. {} ({})", index + 1, candidate.name, candidate.party);
        }
        println!();
        

        // After making selections, cast the ballot
        println!("Enter the candidate numbers you want to vote:");
        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read input");
        let  candidate_index = selection.trim().parse().unwrap();

        for (index, candidate) in election.candidates.iter_mut().enumerate() {
            
            if index + 1 == candidate_index {
                candidate.update_vote();
            }
        }
        
        // Store the selected candidates in the election (for simplicity, we're not checking for duplicates)
        //voters.voted = true;
        println!("Ballot casted successfully!");
    } else {
        println!("Election '{}' not found.", election_name.trim());
    }
}
