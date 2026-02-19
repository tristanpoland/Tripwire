use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Online,
    Idle,
    DoNotDisturb,
    Offline,
}

impl UserStatus {
    pub fn color_hex(&self) -> &'static str {
        match self {
            UserStatus::Online => "#23a55a",
            UserStatus::Idle => "#f0b232",
            UserStatus::DoNotDisturb => "#f23f43",
            UserStatus::Offline => "#80848e",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            UserStatus::Online => "Online",
            UserStatus::Idle => "Idle",
            UserStatus::DoNotDisturb => "Do Not Disturb",
            UserStatus::Offline => "Offline",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub status: UserStatus,
}

impl User {
    pub fn display_name(&self) -> String {
        self.username.clone()
    }

    pub fn tag(&self) -> String {
        format!("{}#{}", self.username, self.discriminator)
    }

    pub fn is_online(&self) -> bool {
        matches!(self.status, UserStatus::Online | UserStatus::Idle | UserStatus::DoNotDisturb)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelKind {
    Text,
    Voice,
    Announcement,
}

impl ChannelKind {
    pub fn prefix(&self) -> &'static str {
        match self {
            ChannelKind::Text => "#",
            ChannelKind::Voice => "♪",
            ChannelKind::Announcement => "!",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub kind: ChannelKind,
    pub unread: usize,
    pub topic: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChannelCategory {
    pub name: String,
    pub channels: Vec<Channel>,
    pub collapsed: bool,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub categories: Vec<ChannelCategory>,
    pub members: Vec<User>,
    pub notification_count: usize,
}

impl Server {
    pub fn initials(&self) -> String {
        self.name
            .split_whitespace()
            .filter_map(|w| w.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase()
    }

    pub fn all_channels(&self) -> Vec<&Channel> {
        self.categories.iter().flat_map(|c| c.channels.iter()).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub author: User,
    pub content: String,
    pub timestamp: String,
    pub edited: bool,
}
