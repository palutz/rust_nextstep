use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Clone)]
struct Account {
    name : String,
    amount : i32,
}

impl Account {
    pub fn new(name : &str, amount : i32) -> Self {
        Self {
            name : name.to_string(),
            amount
        }
    }

    // Can make some checks and formatting
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    // Can make some checks and formatting
    pub fn amount(&self) -> i32 {
        self.amount
    }
}

impl Default for Account {
    fn default() -> Account {
        Account {
            name : "".to_string(),
            amount : 0,
        }
    }
}

fn parse_files(path: &str) -> std::io::Result<Vec<(i32, String, String)>> {
    let file = File::open(path)?;
    let mut res : Vec<(i32, String, String)> = vec![];

    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        match line {
            Ok(l_str) => {
                let parts : Vec<&str> = l_str.split(" ").collect();
                if parts.len() == 3 {  // checking that we have all the parts we need, otherwise skip the content
                    let num : i32 = parts[0].parse().unwrap();
                    res.push((num, parts[1].to_string(), parts[2].to_string()));
                };
            },
            _ => (),
        }
    };
    Ok(res)
}

fn get_account(accounts: &HashMap<String, Account>, owner : &str) -> Account {
    if let Some(account) = accounts.get(owner) {
        account.clone()
    } else {
        Account::default()
    }
}

fn main() {
    let mut accounts : HashMap<String, Account> = HashMap::new();
    // let vars = std::env::args().collect();
    let txs : Vec<(i32, String, String)> = parse_files("myfile.txt").expect("Error opening file!");

    for tx in txs {
        let sender = get_account(&accounts, tx.1.as_str());
        let receiver = get_account(&accounts,tx.2.as_str());

        let new_sender = Account::new(sender.name(), sender.amount() - tx.0);
        let new_receiver = Account::new(receiver.name(), receiver.amount() + tx.0);

        accounts.insert(new_sender.name().to_string(), new_sender);
        accounts.insert(new_receiver.name().to_string(), new_receiver);

    }

    for (owner, account) in accounts.iter() {
        println!("User {} has {} in their account", owner, account.amount());
    }
}
