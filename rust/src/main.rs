use drive_v3::Credentials;

use std::io::{BufRead, BufReader};
use std::fmt;
use std::fs::File;
use std::str;

fn init_gdrive_v3() -> Result<Credentials, drive_v3::Error>{


    // This is the file downloaded in the Setup section
    let client_secrets_path = "client_secret.json";
    // let client_id = String::from("927638576198-47dem4qc3q9009obg2m1ri5nj2trfk4c.apps.googleusercontent.com");

    // The OAuth scopes you need
    let scopes: [&'static str; 2] = [
        "https://www.googleapis.com/auth/drive.metadata.readonly",
        "https://www.googleapis.com/auth/drive.file",
    ];

    let credentials = Credentials::from_client_secrets_file(&client_secrets_path, &scopes)?;

    // TODO make this a global setting!
    let credentials_storage_path = "credentials.json";
    // we've got credentials - store them !
    credentials.store(&credentials_storage_path);
    return Ok(credentials);
}


fn refresh_credentials() -> Result<Credentials, drive_v3::Error> {
    // The OAuth scopes you need
    let scopes: [&'static str; 2] = [
        "https://www.googleapis.com/auth/drive.metadata.readonly",
        "https://www.googleapis.com/auth/drive.file",
    ];


    // TODO make this a global setting!
    let credentials_storage_path = "credentials.json";
    let mut stored_credentials = Credentials::from_file(&credentials_storage_path, &scopes)?;

    // Refresh the credentials if they have expired
    if !stored_credentials.are_valid() {
        stored_credentials.refresh()?;

        // Save them so we don't have to refresh them every time
        stored_credentials.store(&credentials_storage_path)?;
    }
    return Ok(stored_credentials);
}

fn main() {
    println!("Hello, world!");
    init_gdrive_v3();
}
