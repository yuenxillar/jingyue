#[derive(Clone, Debug, serde::Deserialize)]
pub struct ApplicationConfig {
    pub server: ServerConfig,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub local: bool,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            server: Default::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 9999,
            local: true,
        }
    }
}
