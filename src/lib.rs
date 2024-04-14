pub mod authentication {
    use std::collections::HashMap;
    use std::io;
    use crate::voter::Voter;
    //use bcrypt::{hash, verify};
    use bcrypt::verify;
    use sha2::{Sha256, Digest};
    use hex::encode;
    use std::fs;

    pub fn authenticate_officer() -> bool {
        println!("Enter administrator username:");
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Failed to read input");
        let input_cmd = &username;
        let mut hasher = Sha256::new();
        hasher.update(input_cmd.trim().as_bytes());
        let input_hash = encode(hasher.finalize());

        if input_hash == found_in_tempdb() {
            true
        } else {
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
    }

    pub fn authenticate_voter(voters: &mut HashMap<String, Voter>) -> bool {
        println!("Enter voter name:");
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Failed to read input");

        println!("Enter voter date of birth:");
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed to read input");

        if let Some(voter) = voters.get_mut(username.trim()) {
            voter.dob == password.trim()
        } else {
            false
        }
    }

    fn found_in_tempdb() -> String {
        let hash_path = "filesh.txt";
        let hash_trig = fs::read_to_string(hash_path)
            .expect("Failed to read hash file")
            .trim()
            .to_string();

        hash_trig
    }
}

pub mod election {
    use std::collections::HashMap;
    use std::io;
    use crate::voter::Voter;

    pub struct Election {
        pub name: String,
        pub open: bool,
        pub candidates: Vec<Candidate>,
    }

    pub struct Candidate {
        pub name: String,
        pub party: String,
        pub vote: i32,
    }

    impl Candidate {
        pub fn new(name: String, party: String) -> Self {
            Candidate {
                name,
                party,
                vote: 0,
            }
        }

        pub fn update_vote(&mut self) {
            self.vote += 1;
        }
    }

    impl Election {
        pub fn new(name: String) -> Self {
            Election {
                name,
                open: false,
                candidates: Vec::new(),
            }
        }

        pub fn set_candidates(&mut self, candidate: Candidate) {
            self.candidates.push(candidate);
        }

        pub fn is_open(&self) -> bool {
            self.open
        }
    }

    pub fn create_election(elections: &mut HashMap<String, Election>) {
        // implementation...
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
    
            //let mut candidate = Candidate::new(candidate_name,candidate_party);
            let candidate = Candidate::new(candidate_name,candidate_party);
    
            election.set_candidates(candidate);
        }
    
        elections.insert(name.trim().to_string(), election);
        println!("Election '{}' created successfully!", name.trim());
    }

    pub fn register_voter(voters: &mut HashMap<String, Voter>) {
        // implementation...
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

    pub fn open_election(elections: &mut HashMap<String, Election>) {
        // implementation...
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

    pub fn close_election(elections: &mut HashMap<String, Election>) {
        // implementation...
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

    pub fn tally_votes(elections: &mut HashMap<String, Election>) {
        // implementation...
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
}

pub mod voter {
    pub struct Voter {
        pub name: String,
        pub dob: String,
        pub voted: bool,
    }
}

pub mod ballot {
    use std::collections::HashMap;
    //use crate::election::{Election, Candidate};
    use crate::election::Election;
    //use crate::voter::Voter;
    use std::io;

    //pub fn cast_ballot(elections: &mut HashMap<String, Election>, voters: &mut HashMap<String, Voter>) {
    pub fn cast_ballot(elections: &mut HashMap<String, Election>) {
        
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
}
