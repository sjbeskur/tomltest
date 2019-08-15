
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Creds{
    pub default: Profile
}

#[derive(Debug,  Serialize, Deserialize, Clone)]
pub struct Profile {
    pub key: String,
    pub secret: String
}