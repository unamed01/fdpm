use arboard::Clipboard;
use argon2::Argon2;
use rand::{Rng, rngs::StdRng};
use rand::{RngExt, SeedableRng};
use std::error::Error;
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::Duration;
use std::{env, io};
use zeroize::{Zeroize, Zeroizing};

// change this with another random string.
// try running head -c 32 | base64 and paste the output below.
const SALT: &[u8] = b"deci5Dzx+PvvvIaS7osBVgUVByBECbOfq5zZRJD8aD8=";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && matches!(args[1].as_str(), "-h" | "--help") {
        print_help(&args[0]);
        return Ok(());
    }
    println!("input pass (won't be echoed)");
    let mut pass = rpassword::prompt_password(">")?;
    let argon2 = Argon2::default();
    let mut seed = [0u8; 32];
    argon2
        .hash_password_into(pass.as_bytes(), SALT, &mut seed)
        .expect("couldn't hash password.");
    pass.zeroize();
    let mut rng = <StdRng as SeedableRng>::from_seed(seed);
    seed.zeroize();

    //verify string is how I choose to solve the problem of inputting wrong password and not ever knowing its only 4 letters so pretty easy to remember
    //this allows for verifying your password is correct while
    let verify_string = get_pass_from_rng(&mut rng, Some(4))?;
    println!("your verify string is:{}", verify_string);
    println!("if this looks correct press enter if not input anything else.");
    let mut ok = String::new();
    io::stdin().read_line(&mut ok)?;
    if ok != "\n" {
        //why rewrite first 40 lines?
        #[allow(clippy::main_recursion)]
        main()?;
        return Ok(());
    }
    // if argv1 is none or cli arg ask user for desired service.
    let mut service = if args.get(1).is_none() || matches!(args[1].as_str(), "-l" | "--loop") {
        let mut service = String::new();
        println!("input desired service");
        io::stdin().read_line(&mut service)?;
        service
    } else {
        println!("using service name from cli argument.\n");
        args[1].clone()
    };
    let mut clipboard = match Clipboard::new() {
        Ok(k) => k,
        Err(e) => {
            seed.zeroize();
            eprintln!("{e}");
            panic!("couldn't get clipboard handle")
        }
    };
    let mut pass_seed = [0u8; 32];
    rng.fill_bytes(&mut pass_seed);

    //clear and overwrite clipboard to make sure password content is gone before exiting.
    ctrlc::set_handler(move || {
        let mut clip = Clipboard::new().expect("couldn't get handle to clear clipboard..");
        clip.set_text(" ").expect("couldn't overwrite clipboard");
        clip.clear().expect("couldn't clean clipboard..");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    if args.len() >= 2 && matches!(args[1].as_str(), "-l" | "--loop") {
        println!("\n    NOTE: press ctrl+c whenever done.\n");
        copy_and_clear_pass(&service, &mut clipboard, &mut pass_seed)?;
        loop {
            println!("\ninput other service");
            let mut service = String::new();
            io::stdin().read_line(&mut service)?;
            copy_and_clear_pass(&service, &mut clipboard, &mut pass_seed)?;
            service.zeroize();
        }
    } else {
        copy_and_clear_pass(&service, &mut clipboard, &mut pass_seed)?;
        service.zeroize();
    };
    Ok(())
}
//gets pass from rng copies to clipboard then clears and overwrites it after a timer.
fn copy_and_clear_pass(
    service: &str,
    clipboard: &mut Clipboard,
    pass_seed: &mut [u8; 32],
) -> Result<(), Box<dyn Error>> {
    let mut password: Zeroizing<String> = match_2_service(service, pass_seed)?.into();
    if let Err(e) = clipboard.set_text(&*password) {
        pass_seed.zeroize();
        Err(e)?
    };
    password.zeroize();
    println!("password copied to clipboard");
    for secs in (0..=6).rev() {
        print!("\rclearing clipboard in: {}", secs);
        stdout().flush()?;
        sleep(Duration::new(1, 0));
    }
    clipboard.clear()?;
    clipboard.set_text(" ")?;
    println!();
    println!("clipboard cleared.");
    Ok(())
}
fn get_pass_from_rng(rng: &mut StdRng, length: Option<i16>) -> Result<String, Box<dyn Error>> {
    let length = match length {
        Some(s) => s,
        // fixes fingerprinting, each password is a different length so server can't distinguish us
        // from anyone else using a traditional password manager.
        None => rng.random_range(44..126),
    };
    let mut string = String::new();
    for _ in 0..length {
        let a = char::from_u32(rng.random_range(32..127)).unwrap();
        string.push(a);
    }
    Ok(string)
}
fn match_2_service(service: &str, pass_seed: &[u8; 32]) -> Result<String, Box<dyn Error>> {
    let argon2 = Argon2::default();
    let mut password_and_service = Vec::new();
    password_and_service.extend_from_slice(pass_seed);
    password_and_service.extend_from_slice(service.as_bytes());
    let mut seed = [0u8; 32];
    argon2
        .hash_password_into(password_and_service.as_slice(), SALT, &mut seed)
        .unwrap();
    let mut rng = <StdRng as SeedableRng>::from_seed(seed);
    seed.zeroize();
    get_pass_from_rng(&mut rng, None)
}
fn print_help(program_name: &String) {
    println!("deterministic password manager (DPM):");
    println!("  takes a password and runs it trough a cryptographically secure RNG.");
    println!(
        "  fully deterministically generating passwords, that never touch non volatile in ANY form."
    );
    println!(
        "  mixes both master password AND service your service name to generate each individual password"
    );
    println!(
        "  stores ZERO metadata at all, so you must provide both master password and service name each time"
    );
    println!(
        "  this password manger is fully deniable. this password manager leaves no traces at all."
    );
    println!(
        "  after this programs closes, nothing on the system (other the binary itself) points to you ever had of your passwords on this system"
    );
    println!(
        "  allowing for use in any computer on earth no vault sharing required (even fully airgaped ones) just needs the binary.."
    );
    println!("USAGE:");
    println!("  {program_name} -h | --help    prints this message");
    println!(
        "  {program_name} <service>      pass on service name as a shell argument (possibly leaks service name to shell history make sure to prefix command by space to prevent it). ",
    );
    println!(
        "  {program_name} -h | --loop    loops instead of exiting useful when migrating stuff over (you must generate entirely new passwords)"
    )
}
