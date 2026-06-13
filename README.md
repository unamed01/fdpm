# dpm (deterministic password manager)

minimal fully deterministic password manager in rust, this approach fixes two main problems of traditional password managers.

1. someone steals your password vault and tries to brute force it offline (even though this is insanely difficult to pull off if you have a strong master password).
2. backing accessing passwords on any machine without a intricate backup system/accessing passwords in a airgapped machine.

dpm fixes them by fully generating your passwords on the fly derived from the master-password plus service name, the same input will always generate the same output no matter what machine you're on, without any external servers or services needed.

also means no metadata is ever stored even on disk about what services you have an account on. dpm is the **stupidiest password manager on earth by design**. also fixes other problems in other fully deterministic password managers.

1. most other deterministic password mangers usually derive the password directly from the master itself meaning that each password that inevitably gets compromised gives more info about your master password
2. metadata problem, other password managers still store some metadata about the services you have accounts in for convinience while dpm does not by design.

dpm is literally just a rust binary everything gets generated entirely on the fly nothing gets stored on disk at all. It needs nothing to work other than the binary itself, a cpu and ram (optional) also fixes information leakage problem by before deriving the passwords from master password hash + service name it hashes both together with Argon2 then derives the individual password from that. Because of the way dpm is designed an attacker that compromises your system and finds a dpm binary, could not start to brute force your passwords without having compromised at least one password first which raises the bar a lot, and to start to brute force

## usage

### recommended

its recommended you change the salt const

```
const SALT: &[u8] = b"deci5Dzx+PvvvIaS7osBVgUVByBECbOfq5zZRJD8aD8="
```

change to whatever use head -c 32 /dev/urandom | base64 and paste what you get into the ""

test it out by running its an extremely simple app by design

```
cargo run
```
