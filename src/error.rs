use thiserror::Error;

#[derive(Debug, Error)]
pub enum ForgeError {
    #[error("Build error: {0}")]
    Build(String),

    #[error("Patch error: {0}")]
    Patch(String),

    #[error("Validation error: {0}")]
    Validate(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Toolchain error: {0}")]
    Toolchain(String),

    #[error("Scaffold error: {0}")]
    Scaffold(String),

    #[error("Script error: {0}")]
    Script(String),

    #[error("Test error: {0}")]
    Test(String),

    #[error("Certificate error: {0}")]
    Cert(String),

    #[error("Package error: {0}")]
    Package(String),

    #[error("Clean error: {0}")]
    Clean(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ForgeError {
    pub fn exit_code(&self) -> i32 {
        match self {
            ForgeError::Build(_)
            | ForgeError::Patch(_)
            | ForgeError::Validate(_)
            | ForgeError::Script(_)
            | ForgeError::Test(_)
            | ForgeError::Clean(_) => 1,

            ForgeError::Config(_) | ForgeError::Scaffold(_) => 2,

            ForgeError::Toolchain(_) | ForgeError::Cert(_) | ForgeError::Package(_) => 3,

            ForgeError::Io(_) | ForgeError::Other(_) => 1,
        }
    }
}
