# dpm (deterministic password manager)

Minimal fully deterministic password manager in rust, it's approach fixes two main problems of traditional password managers.

1. someone steals your password vault and tries to brute force it offline (even though it's mostly solved when using strong passphrase).
2. backing accessing passwords on any machine without a intricate backup system/accessing passwords in a airgapped machine.

dpm fixes them by fully generating your passwords on the fly derived from the master-password plus service name, the same input will always generate the same output no matter what machine you're on, without any external servers or services needed. This is why you'll never need a backup since there is NEVER anything to backup.

Also means no metadata is ever stored even on disk about what services you have an account on. dpm is the **stupidiest password manager on earth by design**. Also fixes other problems in other fully deterministic password managers.

1. most other deterministic password mangers usually derive the password directly from the master itself meaning that each password that inevitably gets compromised gives more info about your master password
2. metadata problem, other password managers still store some metadata about the services you have accounts in for convinience while dpm does not by design.

dpm is just a rust binary everything gets generated entirely on the fly nothing ever gets stored on disk at all. It needs nothing to work other than the binary itself a cpu and ram no backup is ever necessary other than binary itself only if you change the SALT const.

Fixes information leakage problem by before deriving the passwords from master password hash + service name it hashes both together with Argon2 then derives the individual password from that. Because of the way dpm is designed an attacker that compromises your system and finds a dpm binary, could not start to brute force your passwords without having compromised at least one password first which raises the bar A LOT. Even if your verification string (4 chars that get printed to the screen to ensure you've typed in your master password correctly) is compromised attacker can't reliably brute force based on that since it's designed to be low entropy and collision prone so an attacker can't dream to use it to brute force your passwords.

## usage

### recommended

It's recommended you change the salt const for personal use

```
const SALT: &[u8] = b"deci5Dzx+PvvvIaS7osBVgUVByBECbOfq5zZRJD8aD8="
```

Change to some random string use head -c 32 /dev/urandom | base64 and paste what you get into the "" as a sane default

Test it out by running it. It's an extremely simple app by design 

```
cargo run
```
or run it with an argv to set the service explicitly (be careful with leaking service metadata to shell history prefix the command with a space)
```bash
 cargo run -- github.com # or whatever service.
```
