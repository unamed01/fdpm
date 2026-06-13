use argon2::Argon2;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use rpassword::read_password;
use std::io::Write;
use std::{env, io};
use zeroize::Zeroize;

#[allow(clippy::main_recursion)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("input pass (won't be echoed)");
    print!(">");
    io::stdout().flush().unwrap();
    let mut pass = read_password()?;
    let argon2 = Argon2::default();
    let mut seed = [0u8; 32];
    // you should definetly change the salt..
    argon2
        .hash_password_into(
            pass.as_bytes(),
            b"deci5Dzx+PvvvIaS7osBVgUVByBECbOfq5zZRJD8aD8=",
            &mut seed,
        )
        .unwrap();
    pass.zeroize();
    let mut rng = <StdRng as SeedableRng>::from_seed(seed);
    seed.zeroize();
    let verify_string = get_pass_from_rng(&mut rng, Some(4));
    println!("your verify string is:{verify_string}");
    println!("if this looks correct press enter if not input anything else.");
    let mut ok = String::new();
    io::stdin().read_line(&mut ok).unwrap();
    if ok != "\n" {
        let _ = main();
        return Ok(());
    }
    let service = if args.get(1).is_none() {
        let mut service = String::new();
        println!("input desired service");
        io::stdin().read_line(&mut service).unwrap();
        service
    } else {
        args[1].clone()
    };
    let mut password = match_2_service(service.as_str(), &seed);
    println!("your pasword is {}", password);
    password.zeroize();
    Ok(())
}
fn get_pass_from_rng(rng: &mut StdRng, length: Option<i16>) -> String {
    let length = length.unwrap_or(64);
    let mut string = String::new();
    for _ in 0..length {
        let a = char::from_u32(rng.random_range(32..127)).unwrap();
        string = string + &a.to_string();
    }
    string
}
fn match_2_service(service: &str, pass_seed: &[u8; 32]) -> String {
    let argon2 = Argon2::default();
    let mut password_and_service = Vec::new();
    password_and_service.extend_from_slice(pass_seed);
    password_and_service.extend_from_slice(service.as_bytes());
    let mut seed = [0u8; 32];
    // you should definetly change the salt
    argon2
        .hash_password_into(
            password_and_service.as_slice(),
            b"deci5Dzx+PvvvIaS7osBVgUVByBECbOfq5zZRJD8aD8=",
            &mut seed,
        )
        .unwrap();
    let mut rng = <StdRng as SeedableRng>::from_seed(seed);
    get_pass_from_rng(&mut rng, None)
}
