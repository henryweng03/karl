//! Custom karl-related errors.
use std::io;

#[derive(Debug)]
pub enum Error {
    /// I/O error.
    IoError(io::Error),
    /// Error serializing or deserializing a request or result.
    SerializationError(String),
    /// The number of bytes received before EOF does not correspond to
    /// the number of bytes indicated by the 4-byte header.
    IncorrectPacketLength {
        actual: usize,
        expected: usize,
    },
    /// The number of packets received does not correspond to the number
    /// of packets actually received.
    IncorrectNumPackets {
        actual: usize,
        expected: usize,
    },
    /// Expected to read a packet but received the connection closed
    /// and no bytes were received.
    NoReply,
    /// The packet does not have enough bytes to constitute a header.
    /// The header should include 4 bytes.
    MissingHeader,
    /// No available hosts.
    NoAvailableHosts,
    /// Unexpected packet type.
    InvalidPacketType(u32),
    /// Invalid input root. Either the input root is uninitialized, or
    /// you initialized the root as an existing directory rather than a
    /// custom-built one.
    InvalidInputRoot,
    /// Reinitialized the input root. Should only initialize it once.
    DoubleInputInitialization,
    /// Received a ping result for a compute request or vice versa.
    InvalidResponseType,
    /// Package does not contain a valid binary in its root or imports.
    BinaryNotFound(String),
    /// Failure to install an imported package.
    InstallImportError(String),
    /// Unknown.
    UnknownError(String),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::UnknownError(error)
    }
}
