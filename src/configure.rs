use std::path::Path;

use crate::{cli::Provider, error::Result, providers::ProviderUserInfo};

/// Configures the specified weather provider by saving user information to a configuration file.
///
/// # Arguments
///
/// * `provider` - A `Provider` enum representing the weather provider to be configured.
/// * `config_file` - A `Path` representing the path to the configuration file to be written.
///
/// # Returns
///
/// A `Result` indicating whether the operation was successful.
pub async fn configure(provider: &Provider, config_file: &Path) -> Result<()> {
    match provider {
        Provider::OpenWeather => {
            openweather(config_file)?;
        }
        Provider::WeatherApi => weatherapi(config_file)?,
    }
    println!("Key saved successfully.");

    Ok(())
}

fn openweather(config_file: &Path) -> Result<()> {
    let key = inquire::Password::new("OpenWeather api key:")
        .without_confirmation()
        .prompt()?;
    let serialized = serde_json::to_string(&ProviderUserInfo::OpenWeather { api_key: key })?;
    std::fs::write(config_file, serialized)?;
    Ok(())
}

fn weatherapi(config_file: &Path) -> Result<()> {
    let key = inquire::Password::new("Weather API api key:")
        .without_confirmation()
        .prompt()?;
    let serialized = serde_json::to_string(&ProviderUserInfo::WeatherApi { api_key: key })?;
    std::fs::write(config_file, serialized)?;
    Ok(())
}
