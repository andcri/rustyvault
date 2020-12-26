extern crate base64;
extern crate copypasta_ext;
extern crate reqwest;

use crate::crypto::helpers::decrypt;
use crate::crypto::helpers::extract_string_from_json;
use crate::crypto::helpers::get_api_key_value;
use crate::crypto::helpers::get_username;
use crate::crypto::helpers::get_repository;
use base64::decode;
use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use serde_json::value::Value;

pub async fn get_data() -> Result<(String, String, String), std::io::Error> {
    let api_key = get_api_key_value();
    let username = get_username();
    let repository = get_repository();
    let client = reqwest::Client::new();
    let endpoint = format!(
        "https://api.github.com/repos/{}/{}/contents/default",
        username.replace('"', ""),
        repository.replace('"', "")
    );
    let body = client
        .get(&endpoint)
        .header("Authorization", format!("token {}", api_key))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "request")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let json: serde_json::Value =
        serde_json::from_str(body.trim()).expect("JSON was not well-formatted");
    let content = extract_string_from_json(&json["content"]);
    let sha = extract_string_from_json(&json["sha"]);
    let file_name = extract_string_from_json(&json["name"]);

    let mut final_str = String::new();

    let decoded = String::from_utf8(decode(content.replace("\n", "")).unwrap()).unwrap();
    let parts = decoded.split(",");
    for part in parts {
        if part != "" {
            let decrypted = decrypt(decode(part).unwrap())
                .unwrap()
                .trim_matches(char::from(0))
                .to_string();
            final_str = final_str + decrypted.trim();
        }
    }

    Ok((final_str, sha, file_name))
}

pub async fn get_password(args: &str, show: bool) -> Result<(), std::io::Error> {
    let (decrypted, _, _) = get_data().await.unwrap();
    let cleaned_decrypted = decrypted.trim_matches(char::from(0));
    let content: serde_json::Value =
        serde_json::from_str(cleaned_decrypted).expect("JSON was not well formatted");
    let value = match &content[args.to_string()] {
        Value::String(result) => String::from(result),
        _ => String::from(""),
    };
    if value == "" {
        return Ok(println!(
            "You do not have any password for this key, you can add one use the command 'new'"
        ));
    }
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(value.clone().into()).unwrap();
    if show {
        println!("Your password for {} is {}", args.to_string(), value);
    }
    println!(
        "Your password for {} is now copied on your clipboard",
        args.to_string()
    );
    Ok(())
}
