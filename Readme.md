***RUSYVAULT***

Command line utility that lets you programaticaly access a github vault (a private repo) where all your passwords are stored.

The passwords are encrypted before storage, using rsa key pairs.

This is very much a work in progress and a nice way to learn different encryption systems with Rust

**How to use it**

**Set up your vault on Github**

1. Create a new repo on your github(this will be your personal vault)

2. Get a github api key https://github.com/settings/tokens for your vault repo

**Interact with your rustyvault**

0. build with cargo build or try it with cargo run

1. Put the rustyvault bin in your path
es: add it to your /usr/bin

2. Run rustyvault init to initialize your public, private key and to add your github api key in order to communicate to your rusty vault

3. Run rustyvault new <password_identifier> to create or update a password_identifier with a new password
es: rustyvault new google

4. Run rustyvaul get <password_identifier> to get the password pasted on your clipboard (run with the flag -s to have it printed in the terminal)

Enjoy!

Feel free to write suggestions, fork or make a pr to improve the code :) thanks.
