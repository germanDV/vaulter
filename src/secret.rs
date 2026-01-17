pub struct Secret {
    key: String,
    val: String,
}

#[derive(Debug)]
pub enum SecretError {
    InvalidKey(String),
    InvalidVal(String),
    ClipboardErr(String),
}

impl std::fmt::Display for SecretError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SecretError::InvalidKey(msg) => write!(f, "{}", msg),
            SecretError::InvalidVal(msg) => write!(f, "{}", msg),
            SecretError::ClipboardErr(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for SecretError {}

impl Secret {
    const MIN_LEN: usize = 2;
    const MAX_LEN: usize = 128;

    pub fn new(key: String, val: String) -> Result<Self, SecretError> {
        if !Self::is_valid_string(&key) {
            return Err(SecretError::InvalidKey(format!(
                "key must be between {} and {} characters",
                Self::MIN_LEN,
                Self::MAX_LEN
            )));
        }

        if !Self::is_valid_string(&val) {
            return Err(SecretError::InvalidVal(format!(
                "value must be between {} and {} characters",
                Self::MIN_LEN,
                Self::MAX_LEN
            )));
        }

        Ok(Secret { key, val })
    }

    pub fn val(&self) -> &String {
        &self.val
    }

    fn is_valid_string(s: &String) -> bool {
        s.len() >= Self::MIN_LEN && s.len() <= Self::MAX_LEN
    }
}

impl std::fmt::Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}=[REDACTED] (but really: {})", self.key, self.val)
    }
}
