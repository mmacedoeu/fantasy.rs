//! * Config
//! This is configuration library.
//! It's responsability is to load initialization params
//! from file system, system variables
//! The roadmap to this module will allow it to load from 
//! distributed key stores like etcd, zookeeper, tidb, consul
//! and others providing cloud aware configuration loading

extern crate config;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use failure::Error;

#[derive(Debug, Fail)]
/// Set of errors that can occurr during config process
pub enum ConfigError {
    #[fail(display = "{}", _0)]
    Inner(#[cause] config::ConfigError),
    #[fail(display = "{}", _0)]
    Msg(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub players: Option<usize>,
    pub range: Vec<u8>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            players: Some(2),
            range: vec![15, 70, 96],
        }
    }
}

pub fn load_config(file: &str) -> Result<AppConfig, Error> {
    let mut settings = config::Config::default();
    settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name(file))
        .map_err(|e| Error::from(ConfigError::Inner(e)))?
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `FANTASY_DEBUG=1 ./target/fantasy` would set the `debug` key
        .merge(config::Environment::with_prefix("FANTASY"))
        .map_err(|e| Error::from(ConfigError::Inner(e)))?;

    let app_config: AppConfig = settings
        .try_into()
        .map_err(|e| Error::from(ConfigError::Inner(e)))?;
    Ok(app_config)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
