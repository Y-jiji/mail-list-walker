use std::path::PathBuf;

pub struct Config {
    inner: serde_json::Value,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ConfigError {
    ConfigNoSuchKey(String),
    CannotParseJson(String),
    CannotOpenFile(PathBuf),
    CannotReadFile(PathBuf),
}

impl Into<String> for ConfigError {
    fn into(self) -> String {
        use ConfigError::*;
        match self {
            ConfigNoSuchKey(x) => format!("We don't find key {x} in your configuration file, {}/{}", file!(), line!()),
            CannotOpenFile(x) => format!("We cannot open configuration file {x:?}, {}/{}", file!(), line!()),
            CannotReadFile(x) => format!("We cannot read configuration file {x:?}, {}/{}", file!(), line!()),
            _ => unreachable!(),
        }
    }
}

impl Config {
    pub fn load(path: impl Into<PathBuf>) -> Result<Config, ConfigError> {
        use ConfigError::*;
        let path = path.into().clone();
        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(path.clone())
            .map_err(|_| CannotOpenFile(path.clone()))?;
        let inner = std::io::read_to_string(file).map_err(|_| CannotReadFile(path.clone()))?;
        let inner = serde_json::from_str(&inner).map_err(|e| CannotParseJson(e.to_string()))?;
        Ok(Self { inner })
    }
    pub fn get(&self, key: &str) -> Result<String, ConfigError> {
        let mut current = &self.inner;
        let whole = key.clone();
        for key in key.split(".") {
            current = match current.as_object() {
                None => Err(ConfigError::ConfigNoSuchKey(whole.to_string()))?,
                Some(map) => match map.get(key) {
                    None => Err(ConfigError::ConfigNoSuchKey(whole.to_string()))?,
                    Some(x) => x,
                },
            };
        }
        let val = match current {
            serde_json::Value::String(val) => val.to_string(),
            serde_json::Value::Array(arr) => arr.iter().map(|x| match x.as_str() {
                Some(s) => Ok(s),
                None => Err(()),
            }).try_collect::<Vec<_>>().map_err(|_| ConfigError::ConfigNoSuchKey(whole.to_string()))?.concat(),
            _ => Err(ConfigError::ConfigNoSuchKey(whole.to_string()))?,
        };
        Ok(val)
    }
}