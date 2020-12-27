// here i put the encrypt and decrypt functions
extern crate reqwest;

use home::home_dir;
use openssl::rsa::{Padding, Rsa};
use serde_json::value::Value;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn encrypt(data: &str) -> Result<Vec<u8>, std::io::Error> {
    let path = if let Some(home_path) = home_dir() {
        String::from(format!("{}/.rustyvault/", home_path.to_string_lossy()))
    } else {
        String::from("/.rustyvault/")
    };
    let public = fs::read_to_string(format!("{}rustykey.pem", path))?;

    // Encrypt with public key
    let rsa = Rsa::public_key_from_pem(public.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa
        .public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1)
        .unwrap();
    Ok(buf)
}

pub fn decrypt(buffer: Vec<u8>, passphrase: String) -> Result<String, std::io::Error> {
    let path = if let Some(home_path) = home_dir() {
        String::from(format!("{}/.rustyvault/", home_path.to_string_lossy()))
    } else {
        String::from("/.rustyvault/")
    };
    let private = fs::read_to_string(format!("{}rustykey", path))?;
    let data = buffer;
    let rsa =
        Rsa::private_key_from_pem_passphrase(private.as_bytes(), passphrase.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa
        .private_decrypt(&data, &mut buf, Padding::PKCS1)
        .unwrap();
    let decrypted = String::from_utf8(buf).unwrap();

    Ok(decrypted)
}

pub fn extract_string_from_json(value: &Value) -> String {
    let extracted_string = match value {
        Value::String(a) => String::from(a.trim()),
        _ => String::new(),
    };

    extracted_string
}

pub fn split_data<'a>(data: String) -> Vec<String> {
    // split every 50
    let mut position = 0;
    let mut chunks: Vec<String> = vec![];
    loop {
        if position + 50 >= data.len() {
            chunks.push(String::from(&data[position..]));
            break;
        }
        chunks.push(String::from(&data[position..position + 50]));
        position = position + 50;
    }

    chunks
}

pub fn get_config(name: &str) -> String {
    let path = if let Some(home_path) = home_dir() {
        String::from(format!("{}/.rustyvault/", home_path.to_string_lossy()))
    } else {
        String::from("/.rustyvault/")
    };
    let file = File::open(format!("{}config.json", path)).unwrap();
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");
    let key = json
        .get(name)
        .expect(&format!("file should have {} key", name));
    return key.to_string();
}

pub async fn add_diceware_files() -> () {
    println!("Adding default diceware wordlist");
    let path = if let Some(home_path) = home_dir() {
        String::from(format!(
            "{}/.rustyvault/wordlists",
            home_path.to_string_lossy()
        ))
    } else {
        String::from("/.rustyvault/wordlists")
    };
    if !Path::new(&path).exists() {
        std::fs::create_dir(&path).unwrap();
    }
    // get request and save content in file
    // https://raw.githubusercontent.com/andcri/rustyvault/main/eff_large_wordlist.txt
    let client = reqwest::Client::new();
    let endpoint =
        "https://raw.githubusercontent.com/andcri/rustyvault/main/eff_large_wordlist.txt";
    let body = client
        .get(endpoint)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut dice_word_list = File::create(format!("{}/eff_large_wordlist.txt", path)).unwrap();
    dice_word_list.write_all(body.as_bytes()).unwrap();
}
