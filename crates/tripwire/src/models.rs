use gpui_component::IconName;
use serde::{Deserialize, Serialize};

// ── Voice State ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum VoiceConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Debug, Clone)]
pub struct VoiceState {
    pub channel_id: String,
    pub channel_name: String,
    pub server_id: Option<String>,
    pub server_name: Option<String>,
    pub status: VoiceConnectionStatus,
    pub is_muted: bool,
    pub is_deafened: bool,
    pub is_video_enabled: bool,
    pub is_screen_sharing: bool,
}

impl VoiceState {
    pub fn is_connected(&self) -> bool {
        self.status == VoiceConnectionStatus::Connected
    }
}

// ── User Status ────────────────────────────────────────────────────────────

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

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub user: User,
    pub bio: Option<String>,
    pub custom_status: Option<String>,
    pub custom_status_emoji: Option<String>,
    pub accent_color: Option<String>,
    pub banner_url: Option<String>,
    pub member_since: String,
    pub roles: Vec<Role>,
    pub badges: Vec<Badge>,
    pub note: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Badge {
    Staff,
    Partner,
    HypeSquad,
    BugHunterLevel1,
    BugHunterLevel2,
    EarlySupporter,
    TeamUser,
    VerifiedBot,
    EarlyVerifiedBotDeveloper,
    DiscordCertifiedModerator,
    ActiveDeveloper,
}

impl Badge {
    pub fn icon(&self) -> IconName {
        match self {
            Badge::Staff => IconName::Star,
            Badge::Partner => IconName::Star,
            Badge::HypeSquad => IconName::Star,
            Badge::BugHunterLevel1 => IconName::Search,
            Badge::BugHunterLevel2 => IconName::Search,
            Badge::EarlySupporter => IconName::Heart,
            Badge::TeamUser => IconName::User,
            Badge::VerifiedBot => IconName::Check,
            Badge::EarlyVerifiedBotDeveloper => IconName::Settings,
            Badge::DiscordCertifiedModerator => IconName::Star,
            Badge::ActiveDeveloper => IconName::Settings2,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Badge::Staff => "Staff",
            Badge::Partner => "Partnered Server Owner",
            Badge::HypeSquad => "HypeSquad Events",
            Badge::BugHunterLevel1 => "Bug Hunter Level 1",
            Badge::BugHunterLevel2 => "Bug Hunter Level 2",
            Badge::EarlySupporter => "Early Supporter",
            Badge::TeamUser => "Team User",
            Badge::VerifiedBot => "Verified Bot",
            Badge::EarlyVerifiedBotDeveloper => "Early Verified Bot Developer",
            Badge::DiscordCertifiedModerator => "Discord Certified Moderator",
            Badge::ActiveDeveloper => "Active Developer",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Badge::Staff => "#5865F2",
            Badge::Partner => "#4E96D4",
            Badge::HypeSquad => "#F47B68",
            Badge::BugHunterLevel1 => "#3BA55D",
            Badge::BugHunterLevel2 => "#3BA55D",
            Badge::EarlySupporter => "#EB459E",
            Badge::TeamUser => "#5865F2",
            Badge::VerifiedBot => "#3BA55D",
            Badge::EarlyVerifiedBotDeveloper => "#5865F2",
            Badge::DiscordCertifiedModerator => "#3BA55D",
            Badge::ActiveDeveloper => "#5865F2",
        }
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
pub struct VoiceParticipant {
    pub user_id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub is_speaking: bool,
    pub is_muted: bool,
    pub is_deafened: bool,
    pub is_video: bool,
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub kind: ChannelKind,
    pub unread: usize,
    pub topic: Option<String>,
    pub members_connected: usize,
    pub voice_participants: Vec<VoiceParticipant>,
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
    pub edited_timestamp: Option<String>,
    pub attachment: Option<Attachment>,
    pub reactions: std::collections::HashMap<String, Vec<String>>,
    pub reply_to: Option<Box<MessageReply>>,
    pub mentioned_users: Vec<String>,
    pub pinned: bool,
    pub thread_id: Option<String>,
    pub thread_count: usize,
    pub created_at: std::time::SystemTime, // For grouping logic
}

#[derive(Debug, Clone)]
pub struct MessageReply {
    pub message_id: String,
    pub author: User,
    pub content_preview: String,
}

impl Message {
    pub fn add_reaction(&mut self, emoji: String, user_id: String) {
        self.reactions.entry(emoji).or_default().push(user_id);
    }

    pub fn remove_reaction(&mut self, emoji: &str, user_id: &str) {
        if let Some(users) = self.reactions.get_mut(emoji) {
            users.retain(|id| id != user_id);
            if users.is_empty() {
                self.reactions.remove(emoji);
            }
        }
    }

    pub fn toggle_reaction(&mut self, emoji: String, user_id: String) {
        let has_reacted = self
            .reactions
            .get(&emoji)
            .map(|users| users.contains(&user_id))
            .unwrap_or(false);

        if has_reacted {
            self.remove_reaction(&emoji, &user_id);
        } else {
            self.add_reaction(emoji, user_id);
        }
    }

    pub fn reaction_count(&self, emoji: &str) -> usize {
        self.reactions.get(emoji).map(|users| users.len()).unwrap_or(0)
    }

    pub fn user_reacted(&self, emoji: &str, user_id: &str) -> bool {
        self.reactions
            .get(emoji)
            .map(|users| users.contains(&user_id.to_string()))
            .unwrap_or(false)
    }

    pub fn is_reply(&self) -> bool {
        self.reply_to.is_some()
    }

    pub fn is_mentioned(&self, user_id: &str) -> bool {
        self.mentioned_users.contains(&user_id.to_string())
    }

    pub fn is_thread_parent(&self) -> bool {
        self.thread_count > 0
    }

    pub fn content_preview(&self, max_len: usize) -> String {
        if self.content.len() <= max_len {
            self.content.clone()
        } else {
            format!("{}...", &self.content[..max_len])
        }
    }

    /// Check if this message should be grouped with the previous one
    /// (same author within 5 minutes)
    pub fn should_group_with(&self, other: &Message) -> bool {
        if self.author.id != other.author.id {
            return false;
        }
        
        // If either is a reply, don't group
        if self.is_reply() || other.is_reply() {
            return false;
        }
        
        // Check time difference (5 minutes = 300 seconds)
        if let Ok(duration) = self.created_at.duration_since(other.created_at) {
            duration.as_secs() < 300
        } else {
            false
        }
    }
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
