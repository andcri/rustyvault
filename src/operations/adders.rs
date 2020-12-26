// make a post or put request to add a new value to the file
use super::getters::get_data;
use crate::crypto::helpers::encrypt;
use crate::crypto::helpers::get_api_key_value;
use crate::crypto::helpers::get_username;
use crate::crypto::helpers::get_repository;
use crate::crypto::helpers::split_data;
use base64::encode;
use reqwest;
use serde_json;
use serde_json::value::Value;
use std::collections::HashMap;

pub async fn add_to_file(
    first: bool,
    id: &str,
    password: &str,
) -> Result<(), std::io::Error> {
    let api_key = get_api_key_value();
    let username = get_username();
    let repository = get_repository();
    let mut sha = "".to_string();
    let mut filename = "default".to_string();
    let mut json_file: Value = serde_json::from_str("{}").unwrap();

    if !first {
        let (decoded, s, f) = get_data().await.unwrap();
        sha = s;
        filename = f;
        json_file = serde_json::from_str(decoded.trim_matches(char::from(0))).unwrap();
    }

    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        username.replace('"', ""),
        repository.replace('"', ""),
        filename
    );

    // convert the string into a json object
    // apply add the new data inside it
    json_file[id] = Value::String(String::from(password));
    let splitted_data = split_data(json_file.to_string());
    let mut final_data = String::new();
    // encrypt by chunk, encode by chunk, merge using separator ,
    for data in splitted_data {
        final_data = final_data + encode(&encrypt(data.trim()).unwrap()).trim() + ",";
    }
    let new_final = encode(final_data);

    let mut json_to_post = HashMap::new();
    json_to_post.insert("message", "from_rust");
    json_to_post.insert("content", new_final.trim());
    json_to_post.insert("sha", sha.trim());
    // make the post request
    let client = reqwest::Client::new();
    client
        .put(url.trim())
        .header("Authorization", format!("token {}", api_key))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "request")
        .json(&json_to_post)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Done!");
    Ok(())
}

