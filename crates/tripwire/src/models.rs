use gpui_component::IconName;
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
    Stage,
    Forum,
    Media,
}

impl ChannelKind {
    pub fn prefix(&self) -> &'static str {
        match self {
            ChannelKind::Text => "#",
            ChannelKind::Voice => "🔊",
            ChannelKind::Announcement => "📢",
            ChannelKind::Stage => "🎙️",
            ChannelKind::Forum => "💬",
            ChannelKind::Media => "📁",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            ChannelKind::Text => IconName::File,
            ChannelKind::Voice => IconName::Inbox,
            ChannelKind::Announcement => IconName::Bell,
            ChannelKind::Stage => IconName::LayoutDashboard,
            ChannelKind::Forum => IconName::BookOpen,
            ChannelKind::Media => IconName::FolderOpen,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ChannelKind::Text => "Text channel for messaging",
            ChannelKind::Voice => "Voice channel for audio conversations",
            ChannelKind::Announcement => "Announcement channel for important updates",
            ChannelKind::Stage => "Stage channel for large audio events",
            ChannelKind::Forum => "Forum channel for topic-based discussions",
            ChannelKind::Media => "Media channel for sharing files and images",
        }
    }

    pub fn is_voice_based(&self) -> bool {
        matches!(self, ChannelKind::Voice | ChannelKind::Stage)
    }
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub kind: ChannelKind,
    pub unread: usize,
    pub topic: Option<String>,
    pub members_connected: usize,
}

impl Channel {
    pub fn is_voice_based(&self) -> bool {
        matches!(self.kind, ChannelKind::Voice | ChannelKind::Stage)
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub mime_type: String,
    pub base64_data: String,
    pub size: usize,
}

impl Attachment {
    pub fn is_image(&self) -> bool {
        self.mime_type.starts_with("image/")
    }

    pub fn size_mb(&self) -> f64 {
        self.size as f64 / (1024.0 * 1024.0)
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub author: User,
    pub content: String,
    pub timestamp: String,
    pub edited: bool,
    pub attachment: Option<Attachment>,
}

#[derive(Debug, Clone)]
pub struct DirectMessageChannel {
    pub id: String,
    pub recipient: User,
    pub last_message: Option<String>,
    pub last_message_time: Option<String>,
    pub unread: usize,
}

impl DirectMessageChannel {
    pub fn display_name(&self) -> String {
        self.recipient.username.clone()
    }
}
