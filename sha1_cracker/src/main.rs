use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("Sha1_cracker: <wordlist.txt> <sha1_hash>");
    };

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    // file with the list of words
    let word_file = File::open(&args[1])?;
    let reader = BufReader::new(&word_file);

    for line in reader.lines() {
        let line = line?;
        let common_pwd = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_pwd.as_bytes())) {
            println!("Password found: {common_pwd}");
            return Ok(());
        }
    }

    println!("Password not found");

    Ok(())
}
