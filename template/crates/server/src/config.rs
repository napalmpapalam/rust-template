//! Server configuration section, deserialized as part of the binary's config.

use std::net::{Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};

/// Port the server binds when the config says nothing.
const DEFAULT_PORT: u16 = 8080;

/// Where the HTTP server binds.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields, default)]
pub struct ServerConfig {
    /// Address and port, e.g. `0.0.0.0:8080`.
    pub bind_addr: SocketAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: SocketAddr::from((Ipv4Addr::UNSPECIFIED, DEFAULT_PORT)),
        }
    }
}
