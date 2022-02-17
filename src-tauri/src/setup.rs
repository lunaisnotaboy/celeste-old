use crate::config::read_config;
use crate::structures::UserLogin;
use matrix_sdk::{
    config::{ClientConfig, SyncSettings},
    ruma::{
        api::client::r0::{
            account::register::{RegistrationKind, Request as RegistrationRequest},
            profile, uiaa,
        },
        events::room::message::SyncRoomMessageEvent,
        MxcUri, ServerName, UserId,
    },
    Client, Result as MatrixResult, reqwest::Url,
};
use rand::Rng;
use std::convert::TryFrom;

// User Profile Structure
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub struct UserProfile {
    pub avatar_url: Option<Box<MxcUri>>,
    pub displayname: Option<String>,
}

// Main setup function
pub async fn setup() {
    info!("Initializing...");
    if read_config()
        .getbool("app.networking", "offline")
        .unwrap()
        .unwrap()
    {
        info!("Starting up in offline mode");
    }

    std::thread::sleep(std::time::Duration::from_secs(2));
    info!("Done initializing.");
}

// This function calls the GET profile endpoint
async fn get_profile(client: Client, mxid: &UserId) -> MatrixResult<UserProfile> {
    // First construct the request you want to make
    // See https://docs.rs/ruma-client-api/0.9.0/ruma_client_api/index.html for all available Endpoints
    let request = profile::get_profile::Request::new(mxid);

    // Start the request using matrix_sdk::Client::send
    let resp = client.send(request, None).await?;

    // Use the response and construct a UserProfile struct.
    // See https://docs.rs/ruma-client-api/0.9.0/ruma_client_api/r0/profile/get_profile/struct.Response.html
    // for details on the Response for this Request
    let user_profile = UserProfile {
        avatar_url: resp.avatar_url,
        displayname: resp.displayname,
    };
    Ok(user_profile)
}

/*
async fn client_register(
    username: &str,
    homeserver: Url,
    password: &str,
    device_id: Option<&str>,
) -> Result<Client, matrix_sdk::Error> {
    // Setup client
    let client = Client::new(homeserver).await?;

    // Setup request
    let request = assign!(RegistrationRequest::new(), {
        username: Some(username),
        password: Some(password),
        auth: Some(uiaa::AuthData::FallbackAcknowledgement(
            uiaa::FallbackAcknowledgement::new("foobar"),
        )),
    });

    // Register
    client.register(request).await?;

    Ok(client)
}*/

async fn client_login(
    username: &str,
    homeserver: Url,
    password: &str,
    device_id: Option<&str>,
) -> Result<Client, matrix_sdk::Error> {
    // Client config
    let client_config = ClientConfig::new().disable_ssl_verification(); // Just for testing
    match read_config().get("app.networking", "proxy") {
        Some(proxy) => {
            client_config.proxy(&proxy);
        }
        None => {}
    }

    // Setup client
    let client = Client::new(homeserver).await?;

    // Log in
    client.login(username, password, device_id, None).await?;

    Ok(client)
}

// Login to matrix
#[tauri::command]
pub async fn login(login: UserLogin) -> Result<UserProfile, String> {
    // Get user info
    let username = login.username.as_str();
    let homeserver = login.homeserver;
    let password = login.password.as_str();
    // wtf is this
    let id = match login.device_id {
        Some(id) => id,
        None => "".to_string(),
    };
    let blank = "".to_string();
    let device_id = match id {
        blank => None,
        _ => Some(id.as_str()),
    };

    // Setup the client & login
    let client = match client_login(username, homeserver, password, device_id).await {
        Ok(client) => client,
        Err(e) => return Err(e.to_string()),
    };

    // Get a full MXID from the user_id
	let user_mxid = UserId::try_from(format!("@{}:{}", username, homeserver.host_str().unwrap()).to_str());
    // Get the profile info of the user
    let profile = get_profile(client, &user_mxid).await;
    debug!("{:#?}", profile);

    match profile {
        Ok(p) => return Ok(p),
        Err(e) => return Err(e.to_string()),
    }
}

/*
// Register a new account
#[tauri::command]
pub async fn register(registration: UserLogin) -> Result<(), String> {
    // Get user info
    let username = registration.username.as_str();
    let homeserver = registration.homeserver;
    let password = registration.password.as_str();
    // wtf is this
    let id = match registration.device_id {
        Some(id) => id,
        None => "".to_string(),
    };
    let blank = "".to_string();
    let device_id = match id {
        blank => None,
        _ => Some(id.as_str()),
    };

    // Setup the client & login
    let client = match client_register(username, homeserver, password, device_id).await {
        Ok(client) => client,
        Err(e) => return Err(e.to_string()),
    };
    Ok(())
}*/
