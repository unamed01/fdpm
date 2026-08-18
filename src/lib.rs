use arboard::Clipboard;
use argon2::{Argon2, Params};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use std::error::Error;
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::Duration;
use zeroize::Zeroize;
// maybe change this with another random string
pub const SALT: &[u8] = b"deci5Dzx+PvvvIaS7osBVgUVByBECbOfq5zZRJD8aD8=";
//gets pass from rng copies to clipboard then clears and overwrites it after a timer.
// # Errors
// if fails to write to clipboard
pub fn copy_and_clear_pass(
    service: &str,
    clipboard: &mut Clipboard,
    pass_seed: &mut [u8; 32],
) -> Result<(), Box<dyn Error>> {
    let mut password = match_2_service(service.trim_end_matches('\n'), pass_seed)?;
    if let Err(e) = clipboard.set_text(&*password) {
        password.zeroize();
        pass_seed.zeroize();
        Err(e)?;
    }
    password.zeroize();
    println!("password copied to clipboard");
    for secs in (0..=6).rev() {
        print!("\rclearing clipboard in: {}", secs);
        stdout().flush()?;
        sleep(Duration::from_secs(1));
    }
    clipboard.clear()?;
    clipboard.set_text("")?;
    //make sure clipboard managers see update and overwrite
    sleep(Duration::from_millis(101));
    println!();
    println!("clipboard cleared.");
    Ok(())
}
//hardcode exact argon2 params to make sure we have consistent params accross argon2 crate versions.
// below is current defaults on argon2 crate version 0.5.3
#[must_use]
pub fn get_argon2() -> Argon2<'static> {
    let params = Params::new(19 * 1024, 2, 1, None).expect("failed to build argon2 params");
    Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params)
}
pub fn get_pass_from_rng(rng: &mut StdRng, length: Option<i16>) -> Result<String, Box<dyn Error>> {
    let length = match length {
        Some(s) => s,
        // fixes fingerprinting, each password is a different length so server can't distinguish us
        // from anyone else using a traditional password manager.
        None => rng.random_range(44..126),
    };
    let mut string = String::new();
    for _ in 0..length {
        //will only ever output valid char so unwrap is fine.
        let a = char::from_u32(rng.random_range(32..127)).unwrap();
        string.push(a);
    }
    Ok(string)
}
pub fn match_2_service(service: &str, pass_seed: &[u8; 32]) -> Result<String, Box<dyn Error>> {
    let argon2 = get_argon2();
    let mut password_and_service = Vec::new();
    password_and_service.extend_from_slice(pass_seed);
    password_and_service.extend_from_slice(service.as_bytes());
    let mut seed = [0u8; 32];
    argon2
        .hash_password_into(password_and_service.as_slice(), SALT, &mut seed)
        .expect("failed to hash service + pass_seed together this is a bug.");
    let mut rng = <StdRng as SeedableRng>::from_seed(seed);
    seed.zeroize();
    get_pass_from_rng(&mut rng, None)
}
