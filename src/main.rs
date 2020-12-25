#![allow(dead_code)]

extern crate openssl;
extern crate rpassword;
extern crate tokio;

use clap::{App, Arg, SubCommand};
use crypto::init::init_data;
use operations::adders::add_to_file;
use operations::getters::get_password;
use diceware_gen::DicewareGen;

mod crypto;
mod operations;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let matches = App::new("test")
        .about("A rusty vault to store safely your passwords")
        .version("0.0.1")
        .author("HippoLord")
        .subcommand(
            SubCommand::with_name("init")
                .about("initialize the rsa key pair in a path of your choosing"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("add a new password to your rusty vault")
                .arg(Arg::with_name("password_name").required(true))
                .arg(Arg::with_name("auto")
                .required(false)
                .short("a")
                .long("auto")
                .help("automatically generate a passphrase using diceware")),
        )
        .subcommand(
            SubCommand::with_name("update").arg(Arg::with_name("password_name").required(true)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("retrieve password from your rusty vault")
                .arg(
                    Arg::with_name("show")
                        .required(false)
                        .short("s")
                        .long("show")
                        .help("Prints the retrieved password"),
                )
                .arg(Arg::with_name("password_name").required(true)),
        )
        .get_matches();

    // TODO check when everything other than init is called the proper data is initialized, if that is not the case display to the user

    match matches.subcommand() {
        ("init", _init) => init_data().await,
        ("get", get) => match get {
            Some(value) => {
                if value.is_present("show") {
                    return Ok(get_password(value.value_of("password_name").unwrap(), true).await?);
                }
                Ok(get_password(value.value_of("password_name").unwrap(), false).await?)
            }
            _ => Ok(()),
        },
        ("new", new) => match new {
            Some(value) => {
                if value.is_present("auto") {
                    println!("A safe password is now being generated for you :)");
                    let dice = DicewareGen::new().unwrap();
                    let rounds: u8 = 6;
                    let mut pass = String::new();
                    for word in dice.gen(rounds) {
                        pass += &word;
                    }
                    return Ok(add_to_file(
                        false,
                        value.value_of("password_name").unwrap(),
                        pass.trim(),
                    )
                    .await?);
                }
                let pass = rpassword::prompt_password_stdout("Enter new password: ").unwrap();
                let pass_confirm =
                    rpassword::prompt_password_stdout("Confirm new password: ").unwrap();
                if pass == pass_confirm {
                    println!("Adding your password to your rusty vault...");
                    Ok(add_to_file(
                        false,
                        value.value_of("password_name").unwrap(),
                        pass.trim(),
                    )
                    .await?)
                } else {
                    println!("the passwords that you entered were not matching.");
                    Ok(())
                }
            }
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}
