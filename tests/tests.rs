use fdpm::*;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

//top secret and very secure
const MASTER_PASS: &str = "password123";

#[test]
fn check_backawrds_compatible() {
    let pass_seed = get_pass_seed(MASTER_PASS);
    let service = "github";
    // NOTE: expected password, self computed at 2026-08-17 23:56 VERSION 0.1.1
    // as long as this test passes this program is considered backwards compatible.
    let expected_pass = "X];x'd}Rn6]9Q@?`*yseJ_2+NM72eFR~n6hf7+oPW[K;sv9Z};23>7xf$$?(!8F&uRa9e[MsA$$)EF:KABDQkIC]SaE.RRk+E>kF<2D|eLg)k&WMY";
    let new_pass = match_2_service(service, &pass_seed).unwrap();
    assert_eq!(expected_pass, new_pass);
}
//does same thing as main.rs to setup.
fn get_pass_seed(master_pass: &str) -> [u8; 32] {
    let argon2 = get_argon2();
    let mut seed = [0u8; 32];
    argon2
        .hash_password_into(master_pass.as_bytes(), SALT, &mut seed)
        .unwrap();
    let mut rng = <StdRng as SeedableRng>::from_seed(seed);
    let _ = get_pass_from_rng(&mut rng, Some(4)).unwrap();
    let mut pass_seed = [0u8; 32];
    rng.fill_bytes(&mut pass_seed);
    pass_seed
}
#[test]
fn different_masterpass_different_derived_passwords() {
    let service = "github";
    let pass1 = match_2_service(service, &[1u8; 32]).unwrap();
    let pass2 = match_2_service(service, &[2u8; 32]).unwrap();
    assert_ne!(pass1, pass2)
}
#[test]
fn same_params_same_pass() {
    let pass_seed = get_pass_seed(MASTER_PASS);
    let service = "github";
    let pass1 = match_2_service(service, &pass_seed).unwrap();
    let pass2 = match_2_service(service, &pass_seed).unwrap();
    assert_eq!(pass1, pass2)
}
#[test]
fn ensure_different_services_same_master_not_equal() {
    let pass_seed = get_pass_seed(MASTER_PASS);
    let pass1 = match_2_service("github", &pass_seed).unwrap();
    let pass2 = match_2_service("gmail", &pass_seed).unwrap();
    assert_ne!(pass1, pass2);
}
