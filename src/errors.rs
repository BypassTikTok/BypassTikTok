use std::fmt;

#[derive(Debug)]
pub enum ProxyError {
    ConfigError(String),
    RequestError(String),
    AuthenticationError(String),
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyError::ConfigError(msg) => write!(f, "Config Error: {}", msg),
            ProxyError::RequestError(msg) => write!(f, "Request Error: {}", msg),
            ProxyError::AuthenticationError(msg) => write!(f, "Authentication Error: {}", msg),
        }
    }
}

impl std::error::Error for ProxyError {}
