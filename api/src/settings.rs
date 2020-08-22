use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Settings {
    pub authentication: Authentication,
    pub storage: Storage
}

#[derive(Default, Deserialize)]
pub struct Authentication {
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
    pub api_url: String,

    pub signin_policy: String,
    pub edit_profile_policy: String,
    pub reset_password_policy: String
}

#[derive(Default, Deserialize)]
pub struct Storage {
    pub blob_account: String,
    pub blob_key: String
}
