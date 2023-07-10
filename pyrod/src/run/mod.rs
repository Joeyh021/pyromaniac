mod python;
mod rust;

use std::{ffi::OsString, time::Duration};
use thiserror::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Language {
    Python,
    Rust,
    Java,
}

pub trait Runner: Send + Sync {
    fn compile(&self, code: String) -> Result<(), RunError>;
    fn run(&self, stdin: String) -> Result<(OsString, OsString), RunError>;
}

impl Language {
    pub fn get_runner(self) -> &'static dyn Runner {
        match self {
            Language::Python => Box::leak(Box::new(python::PythonRunner)),
            Language::Rust => Box::leak(Box::new(rust::RustRunner)),
            Language::Java => todo!(),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Python => "python",
                Language::Rust => "rust",
                Language::Java => "java",
            }
        )
    }
}

#[derive(Debug, Error, serde::Deserialize, serde::Serialize)]
pub enum RunError {
    #[error("Thread panicked during execution: {0}")]
    ThreadPanicked(String),
    #[error("I/O error while running code: {0}")]
    IOError(String),
    #[error(
        "File not found error while running code. Fuck you if you wanted to know what file though."
    )]
    FileNotFound,
    #[error("Output data from program was not valid UTF-8")]
    OutputUtf8Error,
    #[error("Code failed to compile: stdout: {0:?}, stderr: {1:?}")]
    CompileError(OsString, OsString),
    #[error("Code exceeded max runtime of {0:?}")]
    RunTimeout(Duration),
    #[error("Code exceeded max compilation time of {0:?}")]
    CompileTimeout(Duration),
}

impl From<std::io::Error> for RunError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => Self::FileNotFound,
            _ => Self::IOError(format!("{value:?}")),
        }
    }
}

impl From<tokio::task::JoinError> for RunError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::ThreadPanicked(format!("{value:?}"))
    }
}
