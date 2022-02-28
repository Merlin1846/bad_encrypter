use std::env;
use std::fs::File;
use std::io::{Write, Read};
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Print the help menu if asked for.
    if args[1] == "-h" || args[1] == "--help" {
        println!("
A simple encrypter made in rust.\n
USAGE:\n
    ./bad_encrypter [SOURCE] [DESTINATION] [SEED] [OPTIONS]\n
OPTIONS:\n
    -d, --decrypt    Decrypts the file using SEED instead of encrypting\n
    -h, --help    Print help information\n")
    }
    
    let mut bytes:Vec<u8>;
    match read_file(&args[1]) {
        Ok(data) => {bytes = data;},
        Err(err) => {panic!("Error read from file at path:{:?} error:{:?}",&args[1],err)}
    };

    let seed_parsed:usize;
    match args[3].parse::<usize>() {
        Ok(seed) => {seed_parsed = seed;},
        Err(err) => {panic!("Error parseing SEED from call arguments, did you use a non whole number? SEED:{:?} error:{:?}",&args[3],err)}
    };

    let speed:Instant = Instant::now();
    // If one of the arguments is `--decrypt` or `-d` then decrypt the file, else encrypt it.
    if args.contains(&"-d".to_owned()) || args.contains(&"--decrypt".to_owned()) {
        // Do everthing in reverse to decrypt it.
        bytes.rotate_right(seed_parsed);
        bytes.reverse();
        for byte in bytes.iter_mut() {
            *byte = byte.reverse_bits();
            *byte = byte.rotate_left(seed_parsed as u32);
        }
    } else {
        for byte in bytes.iter_mut() {
            *byte = byte.rotate_right(seed_parsed as u32);
            *byte = byte.reverse_bits();
        }
        bytes.reverse();
        bytes.rotate_left(seed_parsed);
    }

    match write_file(&args[2], bytes.as_slice()) {
        Ok(_) => {println!("Finished in {:?}",speed.elapsed());},
        Err(error) => println!("Error writing to file, error:{:?}",error)
    };
}

/// Writes to the file at `path` creating it if necessary.
fn write_file(path:&str, data:&[u8]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

/// Reads the file at `path` and returns the entire file as a single String.
/// This is to allow things such as having the DESTINATION file being the same
/// as the `SOURCE` file, it also protects the file contents from modification during encodeing.
/// And unless the file is larger than the free memory there should be 0 problems with this.
fn read_file(path:&str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents:Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}
