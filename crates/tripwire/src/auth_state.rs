use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::models::{User, UserStatus};

/// Persisted auth data stored to disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAuth {
    pub token: String,
    pub user: User,
}

/// In-memory auth state. Loads/saves from a local JSON file.
#[derive(Debug, Clone)]
pub struct AuthState {
    pub current_user: Option<User>,
    pub token: Option<String>,
    pub login_error: Option<String>,
}

impl AuthState {
    /// Create a new auth state, loading any persisted session from disk.
    pub fn new() -> Self {
        if let Some(stored) = Self::load_from_disk() {
            Self {
                current_user: Some(stored.user),
                token: Some(stored.token),
                login_error: None,
            }
        } else {
            Self {
                current_user: None,
                token: None,
                login_error: None,
            }
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    /// Mock login — in a real app this would call the Tripwire API.
    pub fn login(&mut self, email: &str, password: &str) -> bool {
        if email.trim().is_empty() {
            self.login_error = Some("Email is required.".into());
            return false;
        }
        if password.len() < 6 {
            self.login_error = Some("Password must be at least 6 characters.".into());
            return false;
        }

        let username = email
            .split('@')
            .next()
            .unwrap_or("user")
            .to_string();

        let user = User {
            id: format!("user_{}", username),
            username,
            discriminator: "0001".to_string(),
            status: UserStatus::Online,
        };
        let token = format!("token_mock_{}", email);

        self.current_user = Some(user.clone());
        self.token = Some(token.clone());
        self.login_error = None;

        self.persist(&StoredAuth { token, user });
        true
    }

    /// Instantly log in with a dev user — bypasses credential checks.
    pub fn bypass_login(&mut self) {
        let user = User {
            id: "dev_user".to_string(),
            username: "DevUser".to_string(),
            discriminator: "9999".to_string(),
            status: UserStatus::Online,
        };
        let token = "dev_bypass_token".to_string();

        self.current_user = Some(user.clone());
        self.token = Some(token.clone());
        self.login_error = None;

        self.persist(&StoredAuth { token, user });
    }

    /// Clear auth state and remove persisted session.
    pub fn logout(&mut self) {
        self.current_user = None;
        self.token = None;
        self.login_error = None;
        if let Some(path) = Self::auth_file_path() {
            let _ = std::fs::remove_file(path);
        }
    }

    // ── Disk persistence ──────────────────────────────────────────────────────

    fn data_dir() -> Option<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            std::env::var("APPDATA")
                .ok()
                .map(|p| PathBuf::from(p).join("Tripwire"))
        }
        #[cfg(target_os = "macos")]
        {
            std::env::var("HOME").ok().map(|p| {
                PathBuf::from(p)
                    .join("Library")
                    .join("Application Support")
                    .join("Tripwire")
            })
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            std::env::var("HOME")
                .ok()
                .map(|p| PathBuf::from(p).join(".config").join("tripwire"))
        }
    }

    fn auth_file_path() -> Option<PathBuf> {
        Self::data_dir().map(|d| d.join("auth.json"))
    }

    fn load_from_disk() -> Option<StoredAuth> {
        let path = Self::auth_file_path()?;
        let contents = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&contents).ok()
    }

    fn persist(&self, auth: &StoredAuth) {
        let Some(dir) = Self::data_dir() else { return };
        let _ = std::fs::create_dir_all(&dir);
        let Some(path) = Self::auth_file_path() else { return };
        if let Ok(json) = serde_json::to_string_pretty(auth) {
            let _ = std::fs::write(path, json);
        }
    }
}
