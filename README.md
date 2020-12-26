# RUSTYVAULT

Command line utility that lets you programmatically access a github vault (a private repo) where all your passwords are stored.

The passwords are encrypted before storage, using RSA key pairs.

This is very much a work in progress and a nice way to learn different encryption systems (and other concepts) with Rust.

## Get started

### Set up your vault on Github

1. Create a new repo on your github (this will be your personal vault)

2. Get a [GitHub API key](https://github.com/settings/tokens) for your vault repo

### Install Rust and Cargo

```
$ curl https://sh.rustup.rs -sSf | sh
```

### Build

```
$ cargo build or $ cargo run
```

Put the rustyvault bin in your path

```
/usr/bin
```

Then to initialize your public, private key and to add your GitHub API key in order to communicate with
your rustyvault run:

```
$ rustyvault init
```

To create or update a password identifier with a new password run:

```
$ rustyvault new <identifier>
```

To get a password for a specific identifier run:

```
$ rustyvault get <identifier>
```

### Options

| Option | Description                     |
| ------ | ------------------------------- |
| -s     | Print password in your terminal |

Enjoy!

## Contributing

Feel free to write suggestions, fork or make a PR to improve the code :) thanks.

## Licence
