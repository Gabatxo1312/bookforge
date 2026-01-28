use snafu::prelude::*;
use std::net::{AddrParseError, SocketAddr};

use serde::{Deserialize, Serialize};

/// Errors related to listener configuration and socket binding.
#[derive(Snafu, Debug)]
pub enum ListenerError {
    /// The provided bind address and/or port could not be parsed into a valid
    /// `SocketAddr`.
    #[snafu(display("Socket address is wrong"))]
    SocketAddrInvalid {
        socket_addr: String,
        source: AddrParseError,
    },
}

/// Listener configuration used to bind a TCP socket.
///
/// `bind_addr` represents the interface to bind to (e.g. `0.0.0.0`,
/// `127.0.0.1`, `::`), and `port` is the TCP port.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Listener {
    pub port: u32,
    pub bind_addr: String,
}

/// Default listener configuration:
/// - bind on all interfaces (`0.0.0.0`)
/// - listen on port 8000
impl Default for Listener {
    fn default() -> Self {
        Listener {
            port: Self::default_port(),
            bind_addr: Self::default_bind_addr(),
        }
    }
}

impl Listener {
    /// Default bind address used when none is specified.
    fn default_bind_addr() -> String {
        "0.0.0.0".to_string()
    }

    /// Default port used whend none is specified
    fn default_port() -> u32 {
        8000
    }

    /// Computes the socket address used for binding.
    ///
    /// # Errors
    /// Returns `ListenerError::SocketAddrInvalid` if the address or port
    /// cannot be parsed into a valid `SocketAddr`.
    pub fn socket_addr(&self) -> Result<SocketAddr, ListenerError> {
        let socket_addr = format!("{}:{}", self.bind_addr, self.port);
        socket_addr
            .parse()
            .context(SocketAddrInvalidSnafu { socket_addr })
    }
}
