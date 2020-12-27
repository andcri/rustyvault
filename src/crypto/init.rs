use crate::add_to_file;
use crate::crypto::helpers::add_diceware_files;
use home::home_dir;
use openssl::rsa::Rsa;
use openssl::symm::Cipher;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Config {
    username: String,
    repository_name: String,
    github_api_token: String,
    password_protected: bool,
}

pub async fn init_data() -> Result<(), std::io::Error> {
    let (path,
        rsa_password,
        username,
        mut repository_name,
        github_api_token,
        password_protected) = get_init_data()?;

    // create folder if doesnt exists
    if !Path::new(path.trim()).exists() {
        println!("Creating the key pair in {}", path);
        std::fs::create_dir(path.trim())?;
    }
    let rsa = Rsa::generate(4096).unwrap();
    let private_key: Vec<u8> = rsa
        .private_key_to_pem_passphrase(Cipher::aes_256_cbc(), rsa_password.as_bytes())
        .unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
    let private_key_save = String::from_utf8(private_key).unwrap();
    let public_key_save = String::from_utf8(public_key).unwrap();
    let mut private_key_file = File::create(format!("{}rustykey", path))?;
    let mut public_key_file = File::create(format!("{}rustykey.pem", path))?;
    let mut user_config = File::create(format!("{}config.json", path))?;

    // create a GitHub repository if user does not provide an existing one
    if repository_name.len() == 0 {
        repository_name = Uuid::new_v4().to_string();
        println!("Creating vault repository {}", repository_name);
        let mut json_to_post = HashMap::new();
        json_to_post.insert("description", "Password vault");
        json_to_post.insert("private", "true");
        json_to_post.insert("name", &repository_name);
        
        let client = reqwest::Client::new();
        client
        .post("https://api.github.com/user/repos")
        .header("Authorization", format!("token {}", github_api_token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "request")
        .json(&json_to_post)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    }
        
    println!("Creating your configuration file in {}", path);
    private_key_file.write_all(private_key_save.as_bytes())?;
    public_key_file.write_all(public_key_save.as_bytes())?;
    let config = Config {
        username,
        repository_name,
        github_api_token,
        password_protected,
    };
    let config_json = serde_json::to_string(&config)?;
    user_config.write_all(config_json.as_bytes())?;
    add_diceware_files().await;

    Ok(add_to_file(true, "default", "welcome").await?)
}

fn get_init_data() -> Result<(String, String, String, String, String, bool), std::io::Error> {
    let path = if let Some(home_path) = home_dir() {
        String::from(format!("{}/.rustyvault/", home_path.to_string_lossy()))
    } else {
        String::from("/")
    };
    if Path::new(format!("{}rustykey", path).trim()).exists()
        && Path::new(format!("{}rustykey.pem", path).trim()).exists()
        && Path::new(format!("{}github", path).trim()).exists()
    {
        print!("The current key value for the program will be overriten, do you want to proceed? (y/N)");
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop().unwrap();
        if buffer != "y" && buffer != "Y" {
            panic!("exited")
        }
    }
    let rsa_password = rpassword::prompt_password_stdout(
        "Enter a password for the keys (leave blank for no password): ",
    )
    .unwrap();
    let rsa_password_confirm = rpassword::prompt_password_stdout(
        "Enter again the password (leave blank for no password): ",
    )
    .unwrap();
    let github_api_token = rpassword::prompt_password_stdout("Enter your GitHub API token: ").unwrap();
    let github_api_token_confirm =
        rpassword::prompt_password_stdout("Enter again your GitHub API token: ").unwrap();
    let username = rpassword::prompt_password_stdout("Enter your GitHub username: ").unwrap();
    let repository_name = rpassword::prompt_password_stdout(
        "Enter the repository name you want to use as your vault (leave blank for automatic creation): ",
    )
    .unwrap();

    if rsa_password == rsa_password_confirm && github_api_token == github_api_token_confirm {
        let password_protected = if rsa_password.len() != 0 { true } else { false };
        return Ok((
            path,
            rsa_password,
            username,
            repository_name,
            github_api_token,
            password_protected,
        ));
    }
    panic!("There was an error")
}
