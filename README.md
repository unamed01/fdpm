# dpm (deterministic password manager)

minimal alpha of fully deterministic password manager it fixes two main problems of traditional password managers.

1. someone steals your password vault and tries to brute force it offline (even though this is insanely difficult to pull off if you have a strong master password)
2. backing up password vault/accessing passwords on any machine without having a intricate backup system

it fixes them by fully generating your passwords on the fly derived from the master-password and the service name directly same input will always generate the same output no matter what machine you're on, without any external servers or services needed.

also means no metadata is ever stored even on disk about what services you have an account on its the **stupidiest password manager on earth entirely by design**. also fixes other problems in other fully deterministic password managers.

1. most other deterministic password mangers usually derive the password directly from the master itself meaning that each password that gets compromised gives more info to an attacker about your master password
2. metadata problem, other password managers still store some metadata about the services you have accounts in for convinience while dpm does not by design.

it's just a rust binary everything gets generated entirely on the fly nothing gets stored on disk at all it needs literally nothing to work other than the binary itself, a cpu and ram? also fixes information leakage problem by before deriving the passwords from master password hash + service name it hashes both together with Argon2 then derives the actual password from that.
and because the way its built an attacker cannot even start to brute force your password without having compromised at least one password first which raises the bar significantly.

## alpha build

this is mostly a proof of concept right now this shouldn't be used in production while the concepts are sound it still has rough edges that will get fixed later this is only a testing build for now
