use google_drive::Client;
use google_drive::Files;

use std::io::{BufRead, BufReader};
use std::fmt;
use std::fs::File;
use std::str;


// read whole file, and return it as a single string
pub fn read_string(filename: String) -> String {
    let reader = BufReader::new(File::open(filename).expect("Cannot open file"));
    let mut result = String::new();
    let mut val: String;
    for line in reader.lines() {
        val = line.unwrap();
        result += &(val + "\n");
    }

    return result;
}


async fn initialize_gdrive() {
    let mut google_drive = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    //let user_consent_url = google_drive.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    //let mut access_token = google_drive.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    //access_token = google_ drive.refresh_access_token().await.unwrap();
}

async fn list_gdrive() { 
    let client_id = String::from("927638576198-47dem4qc3q9009obg2m1ri5nj2trfk4c.apps.googleusercontent.com");
    let client_secret = read_string("client_secret.json".to_string());
    let redirect_uri = "";
    let token = "";
    let refresh_token = "";
    let _google_drive = Client::new(
        //String::from("client-id"),
        //String::from("client-secret"),
        //String::from("redirect-uri"),
        //String::from("token"),
        //String::from("refresh-token")
        client_id, 
        client_secret,
        redirect_uri,
        token,
        refresh_token
   );
}

fn main() {
    println!("Hello, world!");
    list_gdrive();
}
