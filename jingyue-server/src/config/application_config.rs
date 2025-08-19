#[derive(Clone, Debug, serde::Deserialize)]
pub struct ApplicationConfig {
    pub server: ServerConfig,
    pub heartbeat: HeartbeatConfig,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub local: bool,

}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct HeartbeatConfig {
    window: Box<str>,
    threshold: usize,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            server: Default::default(),
            heartbeat: Default::default()
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

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self {
            window: "5s".into(),
            threshold: 1,
        }
    }
}
