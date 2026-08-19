# fpdm (fully deterministic password manager)

Minimal fully deterministic password manager in rust, it's approach fixes two main problems of traditional password managers.

1. someone steals your password vault and tries to brute force it offline (even though it's "mostly" solved when using strong passphrase).
2. backing accessing passwords on any machine without a intricate backup system/accessing passwords in a air gapped machine.

fpdm fixes them by fully generating your passwords on the fly derived from the master-password plus service name, the same input will always generate the same output no matter what machine you're on, without any external servers or services needed. This is why you'll never need a backup since there is NEVER anything to backup.

Also means no metadata is ever stored even on disk about what services you have an account on. fpdm is the **stupidest password manager on earth by design**. Also fixes other problems in other fully deterministic password managers.

1. most other deterministic password mangers usually derive the password directly from the master itself meaning that each password that inevitably gets compromised gives more information about your master password and how it was generated.
2. metadata problem, other password managers still store some metadata about the services you have accounts in for convenience while fpdm does not by design.

fpdm is just a rust binary everything gets generated entirely on the fly nothing ever gets stored on disk at all. It needs nothing to work other than the binary itself a cpu and ram no backup is ever necessary other than binary itself except if you change the SALT const.

Which Fixes information leakage problem by before deriving the passwords from master password hash + service name it hashes both together with Argon2 then derives the individual password from that. Because of the way fpdm is designed an attacker that compromises your system and finds a fdpm binary, could not start to brute force your passwords without having compromised at least one password first which raises the bar A LOT. Even if your verification string (4 chars that get printed to the screen to ensure you've typed in your master password correctly) is compromised attacker can't reliably brute force based on that since it's designed to be low entropy and collision prone so an attacker can't dream to use it to brute force your passwords.

## features

Password length randomization: by default fdpm has variable length passwords anywhere from 44 to 126 characters which will differ for every password you have stops user fingerprinting trough password length since one is unlikely to consciously  keep changing up length each time they're generating new passwords.

## usage

### recommended

If rainbow table attacks are in your threat model it is recommended you change the SALT const in lib.rs. (stops pre-computation attacks, but default is fine for most users with strong passwords)

```
const SALT: &[u8] = b"deci5Dzx+PvvvIaS7osBVgUVByBECbOfq5zZRJD8aD8="
```

Test it out by running it. It's an extremely simple app by design 

```bash
cargo run -- -h #for usage
cargo run # 1 password at a time
cargo run -- -l #more than 1 password at a time
```
or run it with an argv to set the service explicitly (be careful with leaking service metadata to shell history prefix the command with a space)
```bash
 cargo run -- github.com # or whatever service.
```

Also easily runnable via cargo install 
```bash
cargo install fdpm 
fdpm -h # .cargo/bin/ must be in $PATH
```

## changes 
It is guaranteed that your passwords will stay the same unless stated otherwise below (current roadmap has no plans to make these kinds of changes outside of any theretical future CVEs):


### changes to date that break backwards compatibility since 1.0
**None.** 

[![CI](https://github.com/unamed01/fdpm/actions/workflows/ci.yml/badge.svg)](https://github.com/unamed01/fdpm/actions/workflows/ci.yml)

The guarantee above isn't one you have to trust me on (nothing on this project is). You can test this yourself by running `cargo test` on project root checking whether check_backawrds_compatible test passes, this is also checked automatically trough github actions on every push and commit (check badge above).
