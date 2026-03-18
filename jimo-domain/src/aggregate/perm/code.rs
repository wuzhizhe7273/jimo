#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PermCode(String);

const VALID_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ_";

fn is_valid_code(code: &str) -> bool {
    !code.is_empty() && code.bytes().all(|b| VALID_CHARS.contains(&b))
}

fn is_valid_dot_code(s: &str) -> bool {
    !s.is_empty() && s.split('.').all(is_valid_code)
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid perm code: {0}")]
pub struct PermCodeInvalidError(String);

impl PermCode {
    pub fn new(code: String) -> Result<PermCode, PermCodeInvalidError> {
        if !is_valid_dot_code(&code) {
            return Err(PermCodeInvalidError(code));
        }
        Ok(PermCode(code))
    }

    pub fn new_unchecked(code: String) -> PermCode {
        PermCode(code)
    }

    pub fn empty() -> PermCode {
        PermCode(String::new())
    }

    pub fn try_from_str(s: &str) -> Result<PermCode, PermCodeInvalidError> {
        if !is_valid_dot_code(s) {
            return Err(PermCodeInvalidError(s.to_string()));
        }
        Ok(PermCode(s.to_string()))
    }

    pub fn push(&mut self, code: impl Into<String>) -> Result<&mut Self, PermCodeInvalidError> {
        let code = code.into();
        if !is_valid_code(&code) {
            return Err(PermCodeInvalidError(code));
        }
        if self.0.is_empty() {
            self.0 = code;
        } else {
            self.0 = format!("{}.{}", self.0, code);
        }
        Ok(self)
    }

    pub fn push_unchecked(&mut self, code: impl Into<String>) -> &mut Self {
        let code = code.into();
        if self.0.is_empty() {
            self.0 = code;
        } else {
            self.0 = format!("{}.{}", self.0, code);
        }
        self
    }

    pub fn len(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0.split('.').count()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = String> {
        self.0.split('.').map(|s| s.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_valid(&self) -> bool {
        is_valid_dot_code(&self.0)
    }

    pub fn parent(&self) -> Option<PermCode> {
        self.0
            .rfind('.')
            .map(|pos| PermCode(self.0[..pos].to_string()))
    }

    pub fn last(&self) -> Option<String> {
        self.0.rsplit('.').next().map(|s| s.to_string())
    }

    pub fn is_child_of(&self, parent: &str) -> bool {
        if let Some(p) = self.parent() {
            p.0 == parent
        } else {
            false
        }
    }

    pub fn ancestors(&self) -> Vec<PermCode> {
        let mut ancestors = Vec::new();
        let parts: Vec<&str> = self.0.split('.').collect();
        for i in 1..parts.len() {
            ancestors.push(PermCode(parts[..i].join(".")));
        }
        ancestors
    }

    pub fn is_ancestor_of(&self, other: &PermCode) -> bool {
        if self.0.is_empty() || other.0.is_empty() {
            return false;
        }
        other.0.starts_with(&format!("{}.", self.0)) || other.0 == self.0
    }

    pub fn is_descendant_of(&self, other: &PermCode) -> bool {
        other.is_ancestor_of(self)
    }

    pub fn from_vec(codes: Vec<String>) -> Result<PermCode, PermCodeInvalidError> {
        let code = codes.join(".");
        if !is_valid_dot_code(&code) {
            return Err(PermCodeInvalidError(code));
        }
        Ok(PermCode(code))
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.0.split('.').map(|s| s.to_string()).collect()
    }
}

impl From<String> for PermCode {
    fn from(s: String) -> Self {
        PermCode::new(s).expect("invalid perm code")
    }
}

impl From<&str> for PermCode {
    fn from(s: &str) -> Self {
        PermCode::new(s.to_string()).expect("invalid perm code")
    }
}

impl From<PermCode> for String {
    fn from(code: PermCode) -> Self {
        code.0
    }
}

impl AsRef<str> for PermCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PermCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
