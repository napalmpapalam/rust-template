use std::{io, net::SocketAddr};

use thiserror::Error;

/// Why the server stopped.
#[derive(Debug, Error)]
pub enum ServerError {
    /// The address is taken, or not one this process may bind.
    #[error("binding {addr}")]
    Bind {
        /// The address that was refused.
        addr: SocketAddr,
        /// The OS error behind it.
        #[source]
        source: io::Error,
    },

    /// The accept loop failed.
    #[error("serving")]
    Serve(#[source] io::Error),
}
