use error::Error;
use key::{Keys, Wallet};
use toml;
use serde_json;
use std::fs;

/// include path to put keys
pub fn store_keys(keys: Keys) -> Result<(), Error> {
    let toml = toml::to_string(&keys)?;
    fs::write("keys.toml", toml)?;
    Ok(())
}

pub fn retrieve_keys_toml(path: &str) -> Result<Keys, Error> {
    let read = fs::read_to_string(path)?;
    let toml: Keys = toml::from_str(&read)?;
    Ok(toml)
}

pub fn retrieve_keys_json(path: &str) -> Result<Wallet, Error> {
    let read = fs::read_to_string(path)?;
    let wallet: Wallet = serde_json::from_str(&read)?;
    Ok(wallet)
}
