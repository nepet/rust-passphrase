# rust-passphrase

This is a simplistic passphrase generator inspired by [xkcd](https://xkcd.com/936/) and [hsxkpasswd](https://github.com/bbusschots/hsxkpasswd).

It uses the _ChaCha20Rng_ rng for safe random numbers and the bitcoin memonic word list in english from [bip-0039](https://github.com/bitcoin/bips/blob/master/bip-0039/bip-0039-wordlists.md).

## Install
If you want to build `rust-passphrase` by yourself you need `rustc` and `cargo` installed.

Clone into the repository
`git clone git@github.com:nepet/rust-passphrase.git`
`cd rust-passphrase`

and build the program with cargo:
`cargo build --release`

You can then move the program built in `target/release` to the `bin` files or add the path to the `PATH` environmental variable.

## How to use

command: `rust-passphrase generate [OPTIONS]`
#### Options
|long|short|description|
|-|-|-|
|--num-words|-n|number of words used for passphrase _(default: 3)_|
|--separator|-s|space separated list of separators to choose from _(default: "-")_|
|--num_trailing_numbers|-n|number of trailing numbers appended to the passphrase _(default: 3)_|


### Todo

- [ ] Allow for own wordlists.
- [ ] Add `rust-passphrase inspect [OPTIONS]` command to return insight on entropy and estimated time to crack.
- [ ] Add transformations (capitalizing modes, 1337 mode)
- [ ] Add padding symbols.
- [ ] Allow for config files.
