use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
pub enum VaultRequestsError {
    InvalidApiToken,
    InvalidPasswordId,
    InvalidPrivateKey,
    InvalidPublicKey,
}

impl Display for VaultRequestsError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl VaultRequestsError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidApiToken => "Invalid API token",
            Self::InvalidPasswordId => "Invalid PasswordId",
            Self::InvalidPrivateKey => "Invalid API key",
            Self::InvalidPublicKey => "Invalid API key",
        }
    }
}

impl Error for VaultRequestsError {}

// define possible errors

// connection error
// invalid api key
// missing pub/private key
// password id not in vault