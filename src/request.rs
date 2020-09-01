use std::fmt;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

/// Requests.
#[derive(Debug, Serialize, Deserialize)]
pub enum KarlRequest {
    Ping(PingRequest),
    Compute(ComputeRequest),
}

/// Ping request.
#[derive(Debug, Serialize, Deserialize)]
pub struct PingRequest {}

/// Compute request.
#[derive(Serialize, Deserialize)]
pub struct ComputeRequest {
    /// Formatted zipped directory.
    ///
    /// config
    /// package/
    /// -- binary.wasm
    /// -- files
    pub zip: Vec<u8>,
    /// Whether to include stdout in the results.
    pub stdout: bool,
    /// Whether to include stderr in the results.
    pub stderr: bool,
    /// Files to include in the results, if they exist.
    pub files: HashSet<String>,
}

impl PingRequest {
    /// Create a new ping request.
    pub fn new() -> Self {
        PingRequest {}
    }
}

impl fmt::Debug for ComputeRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComputeRequest")
            .field("zip (nbytes)", &self.zip.len())
            .field("stdout", &self.stdout)
            .field("stderr", &self.stderr)
            .field("files", &self.files)
            .finish()
    }
}

impl ComputeRequest {
    /// Create a new compute request based on a a serialized version of a
    /// formatted zipped directory.
    ///
    /// config
    /// package/
    /// -- binary.wasm
    /// -- files
    pub fn new(zip: Vec<u8>) -> Self {
        ComputeRequest {
            zip,
            stdout: false,
            stderr: false,
            files: HashSet::new(),
        }
    }

    /// Include stdout in the results.
    pub fn stdout(mut self) -> Self {
        self.stdout = true;
        self
    }

    /// Include stderr in the results.
    pub fn stderr(mut self) -> Self {
        self.stderr = true;
        self
    }

    /// Include a specific output file in the results, if it exists.
    pub fn file(mut self, filename: &str) -> Self {
        self.files.insert(filename.to_string());
        self
    }
}
