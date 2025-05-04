use drive_v3::{Credentials, Drive};

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

fn list_files(credentials: Credentials) -> Result<(), drive_v3::Error> {
    let drive = Drive::new(&credentials);

    let file_list = drive.files.list()
        .fields("files(name, id, mimeType)") // Set what fields will be returned
        // .q("name = 'file_im_looking_for' and not trashed") // search for specific files
        .q("name != 'file_i_want_to_ignore' and not trashed") // search for specific files
        .execute()?;

    if let Some(files) = file_list.files {
        for file in &files {
            println!("{}", file);
        }
    }
    return Ok(());
}

fn main() {
    println!("Hello, world!");
    let credentials = init_gdrive_v3();
    list_files(credentials.unwrap());
}
