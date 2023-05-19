use std::{io::Read, println};

use anyhow::Result;
use inquire::InquireError;

const CLIENT_ID: &str = "Iv1.4de4d4a1d7ba2f81";

pub fn is_authenticated() -> bool {
    otter_auth_client::get_config().is_ok()
}

pub async fn authenticate() -> Result<()> {
    let auth = otter_auth_client::get_github_auth_code(CLIENT_ID)
        .await
        .map_err(|e| InquireError::Custom(Box::new(e)))?;

    println!("Please go to the following url: {}", &auth.verification_uri);
    println!("Enter the following code: {}", &auth.user_code);
    println!("Press enter when you have completed authentication");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
    otter_auth_client::save_config(&auth).map_err(|e| InquireError::Custom(Box::new(e)))?;
    Ok(())
}
