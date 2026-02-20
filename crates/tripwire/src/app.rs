use std::collections::HashMap;

use gpui::{App, Context, Entity, FocusHandle, Focusable, Subscription, Window};
use gpui_component::input::{InputEvent, InputState};
use gpui::AppContext;
use crate::auth_state::AuthState;
use crate::mock_data;
use crate::models::{Attachment, Channel, ChannelKind, DirectMessageChannel, Message, MessageReply, Server, User, UserProfile, VoiceState};
use crate::titlebar::TripwireTitleBar;
use crate::app::app_view::settings::SettingsScreen;

#[derive(Debug, Clone, PartialEq)]
pub enum AppView {
    Servers,
    DirectMessages,
}

pub mod app_view;
pub mod auth_view;

// ── TripwireApp ───────────────────────────────────────────────────────────────

/// Root application entity. Manages the full lifecycle: auth → app.
/// Render is delegated to `auth_view` or `app_view` modules.
pub struct TripwireApp {
    pub(crate) focus_handle: FocusHandle,
    pub(crate) titlebar: Entity<TripwireTitleBar>,

    // ── Auth state ──────────────────────────────────────────────────────────
    pub(crate) auth: AuthState,
    pub(crate) email_input: Entity<InputState>,
    pub(crate) password_input: Entity<InputState>,

    // ── App state ──────────────────────────────────────────────────────────
    pub(crate) current_view: AppView,
    pub(crate) servers: Vec<Server>,
    pub(crate) active_server: usize,
    pub(crate) active_channel_id: Option<String>,
    /// Messages keyed by channel_id
    pub(crate) messages: HashMap<String, Vec<Message>>,
    pub(crate) dm_channels: Vec<DirectMessageChannel>,
    pub(crate) active_dm_id: Option<String>,
    /// DM messages keyed by dm_id
    pub(crate) dm_messages: HashMap<String, Vec<Message>>,
    pub(crate) message_input: Entity<InputState>,
    pub(crate) show_members: bool,
    pub(crate) pending_attachment: Option<Attachment>,
    pub(crate) emoji_search: String,
    pub(crate) active_emoji_picker_message: Option<String>,
    
    // ── Reply state ─────────────────────────────────────────────────────────
    pub(crate) replying_to: Option<MessageReply>,
    
    // ── Edit state ──────────────────────────────────────────────────────────
    pub(crate) editing_message_id: Option<String>,
    
    // ── Typing indicators ────────────────────────────────────────────────────
    pub(crate) typing_users: HashMap<String, Vec<String>>, // channel_id -> [user_ids]
    
    // ── Voice state ─────────────────────────────────────────────────────────
    pub(crate) voice_state: Option<VoiceState>,
    pub(crate) show_voice_switch_warning: Option<(Channel, Option<Server>)>, // (new_channel, new_server)
    
    // ── Profile state ───────────────────────────────────────────────────────
    pub(crate) show_profile: Option<UserProfile>,
    pub(crate) user_profiles: HashMap<String, UserProfile>,
    
    // ── Settings state ──────────────────────────────────────────────────────
    pub(crate) show_settings: bool,
    pub(crate) settings_screen: SettingsScreen,
    
    // ── Server Settings state ───────────────────────────────────────────────
    pub(crate) show_server_settings: bool,
    pub(crate) server_settings_screen: app_view::server_settings::ServerSettingsScreen,

    pub(crate) _subscriptions: Vec<Subscription>,
}

impl TripwireApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let auth = AuthState::new();
        let titlebar = cx.new(|cx| TripwireTitleBar::new(window, cx));

        // Auth inputs
        let email_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Email or username"));
        let password_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Password").masked(true));

        // Pre-load first server / channel messages
        let servers = mock_data::make_servers();
        let active_channel_id = servers
            .first()
            .and_then(|s| s.all_channels().first().map(|c| c.id.clone()));

        let mut messages: HashMap<String, Vec<Message>> = HashMap::new();
        if let Some(ref ch_id) = active_channel_id {
            messages.insert(ch_id.clone(), mock_data::make_messages_for(ch_id));
        }

        // Pre-load DM channels
        let dm_channels = mock_data::make_dm_channels();
        let dm_messages: HashMap<String, Vec<Message>> = HashMap::new();

        let message_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Send a message..."));

        // Subscribe message input to catch Enter key to send
        let msg_sub = cx.subscribe(
            &message_input,
            |this: &mut TripwireApp, _, event: &InputEvent, cx| {
                if let InputEvent::PressEnter { .. } = event {
                    // Pressing enter outside multi-line triggers send
                    // We handle this via the send button; here just notify
                    cx.notify();
                }
            },
        );

        Self {
            focus_handle: cx.focus_handle(),
            titlebar,
            auth,
            email_input,
            password_input,
            current_view: AppView::Servers,
            servers,
            active_server: 0,
            active_channel_id,
            messages,
            dm_channels,
            active_dm_id: None,
            dm_messages,
            message_input,
            show_members: true,
            pending_attachment: None,
            emoji_search: String::new(),
            active_emoji_picker_message: None,
            replying_to: None,
            editing_message_id: None,
            typing_users: HashMap::new(),
            voice_state: None,
            show_voice_switch_warning: None,
            show_profile: None,
            user_profiles: HashMap::new(),
            show_settings: false,
            settings_screen: SettingsScreen::Account,
            show_server_settings: false,
            server_settings_screen: app_view::server_settings::ServerSettingsScreen::Overview,
            _subscriptions: vec![msg_sub],
        }
    }

    // ── Queries ────────────────────────────────────────────────────────────

    pub(crate) fn active_server(&self) -> Option<&Server> {
        self.servers.get(self.active_server)
    }

    pub(crate) fn active_channel(&self) -> Option<&Channel> {
        self.active_server().and_then(|s| {
            let id = self.active_channel_id.as_deref()?;
            s.all_channels().into_iter().find(|c| c.id == id)
        })
    }

    pub(crate) fn active_channel_name(&self) -> Option<&str> {
        self.active_channel().map(|c| c.name.as_str())
    }

    pub(crate) fn active_channel_topic(&self) -> Option<&str> {
        self.active_channel().and_then(|c| c.topic.as_deref())
    }

    pub(crate) fn active_channel_kind(&self) -> Option<ChannelKind> {
        self.active_channel().map(|c| c.kind.clone())
    }

    pub(crate) fn active_messages(&self) -> &[Message] {
        self.active_channel_id
            .as_deref()
            .and_then(|id| self.messages.get(id))
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub(crate) fn active_dm_messages(&self) -> &[Message] {
        self.active_dm_id
            .as_deref()
            .and_then(|id| self.dm_messages.get(id))
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    // ── Mutations ─────────────────────────────────────────────────────────

    pub(crate) fn switch_to_servers(&mut self, cx: &mut Context<Self>) {
        self.current_view = AppView::Servers;
        self.active_dm_id = None;
        cx.notify();
    }

    pub(crate) fn switch_to_dms(&mut self, cx: &mut Context<Self>) {
        self.current_view = AppView::DirectMessages;
        cx.notify();
    }

    pub(crate) fn switch_server(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        self.current_view = AppView::Servers;
        self.active_server = index;
        self.active_dm_id = None;
        if let Some(server) = self.servers.get(index) {
            let channel_id = server.all_channels().first().map(|c| c.id.clone());
            if let Some(ref ch_id) = channel_id {
                if !self.messages.contains_key(ch_id.as_str()) {
                    self.messages
                        .insert(ch_id.clone(), mock_data::make_messages_for(ch_id));
                }
            }
            self.active_channel_id = channel_id;
        }
        cx.notify();
    }

    pub(crate) fn switch_channel(
        &mut self,
        channel_id: String,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.current_view = AppView::Servers;
        if !self.messages.contains_key(&channel_id) {
            self.messages
                .insert(channel_id.clone(), mock_data::make_messages_for(&channel_id));
        }
        self.active_channel_id = Some(channel_id);
        self.active_dm_id = None;
        cx.notify();
    }

    pub(crate) fn switch_dm(
        &mut self,
        dm_id: String,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.current_view = AppView::DirectMessages;
        if !self.dm_messages.contains_key(&dm_id) {
            self.dm_messages
                .insert(dm_id.clone(), mock_data::make_dm_messages_for(&dm_id));
        }
        self.active_dm_id = Some(dm_id);
        self.active_channel_id = None;
        cx.notify();
    }

    pub(crate) fn toggle_category(&mut self, category_name: &str, cx: &mut Context<Self>) {
        if let Some(server) = self.servers.get_mut(self.active_server) {
            if let Some(cat) = server
                .categories
                .iter_mut()
                .find(|c| c.name == category_name)
            {
                cat.collapsed = !cat.collapsed;
                cx.notify();
            }
        }
    }

    pub(crate) fn send_message(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let content = self.message_input.read(cx).value().trim().to_string();
        if content.is_empty() && self.pending_attachment.is_none() {
            return;
        }
        
        if let Some(user) = self.auth.current_user.clone() {
            let msg = Message {
                id: timestamp_id(),
                author: user,
                content,
                timestamp: "Just now".to_string(),
                edited: false,
                edited_timestamp: None,
                attachment: self.pending_attachment.take(),
                reactions: std::collections::HashMap::new(),
                reply_to: self.replying_to.take().map(Box::new),
                mentioned_users: vec![],
                pinned: false,
                thread_id: None,
                thread_count: 0,
                created_at: std::time::SystemTime::now(),
            };
            
            match self.current_view {
                AppView::Servers => {
                    if let Some(channel_id) = self.active_channel_id.clone() {
                        self.messages.entry(channel_id).or_default().push(msg);
                    }
                }
                AppView::DirectMessages => {
                    if let Some(dm_id) = self.active_dm_id.clone() {
                        self.dm_messages.entry(dm_id).or_default().push(msg);
                    }
                }
            }
        }
        
        self.message_input.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        cx.notify();
    }

    pub(crate) fn attach_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let paths_future = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Select an image or GIF".into()),
        });

        let entity = cx.entity();
        cx.spawn_in(window, async move |_, window| {
            if let Ok(Ok(Some(paths))) = paths_future.await {
                if let Some(path) = paths.first() {
                    window.update(|window, cx| {
                        entity.update(cx, |this, cx| {
                            this.process_attachment(path.clone(), window, cx);
                        })
                    }).ok();
                }
            }
        }).detach();
    }

    fn process_attachment(
        &mut self,
        path: std::path::PathBuf,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Read file
        if let Ok(data) = std::fs::read(&path) {
            // Check file size (5MB limit)
            let size = data.len();
            if size > 5 * 1024 * 1024 {
                // TODO: Show error notification
                eprintln!("File too large: {} bytes (max 5MB)", size);
                return;
            }

            // Determine MIME type from extension
            let mime_type = match path.extension().and_then(|s| s.to_str()) {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                Some("webp") => "image/webp",
                _ => {
                    eprintln!("Unsupported file type");
                    return;
                }
            };

            // Convert to base64
            let base64_data = base64_encode(&data);

            // Get filename
            let filename = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            self.pending_attachment = Some(Attachment {
                filename,
                mime_type: mime_type.to_string(),
                base64_data,
                size,
            });

            cx.notify();
        } else {
            eprintln!("Failed to read file: {:?}", path);
        }
    }

    pub(crate) fn clear_attachment(&mut self, cx: &mut Context<Self>) {
        self.pending_attachment = None;
        cx.notify();
    }

    pub(crate) fn toggle_reaction(
        &mut self,
        message_id: String,
        emoji: String,
        cx: &mut Context<Self>,
    ) {
        if let Some(user_id) = self.auth.current_user.as_ref().map(|u| u.id.clone()) {
            let messages = match self.current_view {
                AppView::Servers => {
                    if let Some(channel_id) = &self.active_channel_id {
                        self.messages.get_mut(channel_id)
                    } else {
                        None
                    }
                }
                AppView::DirectMessages => {
                    if let Some(dm_id) = &self.active_dm_id {
                        self.dm_messages.get_mut(dm_id)
                    } else {
                        None
                    }
                }
            };

            if let Some(messages) = messages {
                if let Some(msg) = messages.iter_mut().find(|m| m.id == message_id) {
                    msg.toggle_reaction(emoji, user_id);
                }
            }

            cx.notify();
        }
    }

    pub(crate) fn start_reply(&mut self, message: &Message, cx: &mut Context<Self>) {
        self.replying_to = Some(MessageReply {
            message_id: message.id.clone(),
            author: message.author.clone(),
            content_preview: message.content_preview(50),
        });
        cx.notify();
    }

    pub(crate) fn cancel_reply(&mut self, cx: &mut Context<Self>) {
        self.replying_to = None;
        cx.notify();
    }

    pub(crate) fn show_user_profile(&mut self, user: User, cx: &mut Context<Self>) {
        // Get or create profile
        let profile = self
            .user_profiles
            .entry(user.id.clone())
            .or_insert_with(|| mock_data::make_user_profile(user.clone()))
            .clone();
        
        self.show_profile = Some(profile);
        cx.notify();
    }

    pub(crate) fn close_profile(&mut self, cx: &mut Context<Self>) {
        self.show_profile = None;
        cx.notify();
    }

    pub(crate) fn open_settings(&mut self, cx: &mut Context<Self>) {
        self.show_settings = true;
        cx.notify();
    }

    pub(crate) fn close_settings(&mut self, cx: &mut Context<Self>) {
        self.show_settings = false;
        cx.notify();
    }

    pub(crate) fn switch_settings_screen(&mut self, screen: SettingsScreen, cx: &mut Context<Self>) {
        self.settings_screen = screen;
        cx.notify();
    }
    
    // ── Server Settings helpers ─────────────────────────────────────────────
    
    pub(crate) fn open_server_settings(&mut self, cx: &mut Context<Self>) {
        self.show_server_settings = true;
        self.server_settings_screen = app_view::server_settings::ServerSettingsScreen::Overview;
        cx.notify();
    }
    
    pub(crate) fn close_server_settings(&mut self, cx: &mut Context<Self>) {
        self.show_server_settings = false;
        cx.notify();
    }
    
    pub(crate) fn switch_server_settings_screen(&mut self, screen: app_view::server_settings::ServerSettingsScreen, cx: &mut Context<Self>) {
        self.server_settings_screen = screen;
        cx.notify();
    }
    
    // ── Message editing helpers ─────────────────────────────────────────────
    
    pub(crate) fn start_edit_message(&mut self, message_id: String, window: &mut Window, cx: &mut Context<Self>) {
        // Find the message and populate input with its content
        let messages = match self.current_view {
            AppView::Servers => {
                if let Some(channel_id) = &self.active_channel_id {
                    self.messages.get(channel_id)
                } else {
                    None
                }
            }
            AppView::DirectMessages => {
                if let Some(dm_id) = &self.active_dm_id {
                    self.dm_messages.get(dm_id)
                } else {
                    None
                }
            }
        };
        
        if let Some(messages) = messages {
            if let Some(msg) = messages.iter().find(|m| m.id == message_id) {
                self.editing_message_id = Some(message_id);
                self.message_input.update(cx, |state, cx| {
                    state.set_value(&msg.content, window, cx);
                });
                cx.notify();
            }
        }
    }
    
    pub(crate) fn cancel_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editing_message_id = None;
        self.message_input.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        cx.notify();
    }
    
    pub(crate) fn save_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(message_id) = self.editing_message_id.clone() {
            let new_content = self.message_input.read(cx).value().trim().to_string();
            if new_content.is_empty() {
                return;
            }
            
            let messages = match self.current_view {
                AppView::Servers => {
                    if let Some(channel_id) = &self.active_channel_id {
                        self.messages.get_mut(channel_id)
                    } else {
                        None
                    }
                }
                AppView::DirectMessages => {
                    if let Some(dm_id) = &self.active_dm_id {
                        self.dm_messages.get_mut(dm_id)
                    } else {
                        None
                    }
                }
            };
            
            if let Some(messages) = messages {
                if let Some(msg) = messages.iter_mut().find(|m| m.id == message_id) {
                    msg.content = new_content;
                    msg.edited = true;
                    msg.edited_timestamp = Some("Just now".to_string());
                }
            }
            
            self.cancel_edit(window, cx);
        }
    }
    
    pub(crate) fn delete_message(&mut self, message_id: String, cx: &mut Context<Self>) {
        let messages = match self.current_view {
            AppView::Servers => {
                if let Some(channel_id) = &self.active_channel_id {
                    self.messages.get_mut(channel_id)
                } else {
                    None
                }
            }
            AppView::DirectMessages => {
                if let Some(dm_id) = &self.active_dm_id {
                    self.dm_messages.get_mut(dm_id)
                } else {
                    None
                }
            }
        };
        
        if let Some(messages) = messages {
            messages.retain(|m| m.id != message_id);
        }
        
        cx.notify();
    }
    
    // ── Typing indicator helpers ────────────────────────────────────────────
    
    pub(crate) fn get_typing_users(&self) -> Vec<String> {
        let channel_or_dm_id = match self.current_view {
            AppView::Servers => self.active_channel_id.as_ref(),
            AppView::DirectMessages => self.active_dm_id.as_ref(),
        };
        
        channel_or_dm_id
            .and_then(|id| self.typing_users.get(id))
            .map(|users| {
                users
                    .iter()
                    .filter_map(|user_id| {
                        // Look up username from servers or DMs
                        if let Some(server) = self.active_server() {
                            server.members.iter()
                                .find(|m| &m.id == user_id)
                                .map(|m| m.username.clone())
                        } else if let Some(dm_id) = &self.active_dm_id {
                            self.dm_channels.iter()
                                .find(|dm| &dm.id == dm_id)
                                .map(|dm| dm.recipient.username.clone())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub(crate) fn logout(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.auth.logout();
        // Clear message input value happens implicitly since we reset auth
        cx.notify();
    }
    
    // ── Voice management helpers ────────────────────────────────────────────
    
    pub(crate) fn join_voice_channel(
        &mut self,
        channel: &Channel,
        server: Option<&Server>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // If already in a voice channel, show warning
        if let Some(ref current_voice) = self.voice_state {
            if current_voice.channel_id != channel.id {
                self.show_voice_switch_warning = Some((channel.clone(), server.cloned()));
                cx.notify();
                return;
            }
        }
        
        self.voice_state = Some(VoiceState {
            channel_id: channel.id.clone(),
            channel_name: channel.name.clone(),
            server_id: server.map(|s| s.id.clone()),
            server_name: server.map(|s| s.name.clone()),
            status: crate::models::VoiceConnectionStatus::Connecting,
            is_muted: false,
            is_deafened: false,
            is_video_enabled: false,
            is_screen_sharing: false,
        });
        
        // Simulate connection (in real app, this would be async)
        let entity = cx.entity();
        window.defer(cx, move |window, cx| {
            _ = entity.update(cx, |this, cx| {
                if let Some(ref mut voice) = this.voice_state {
                    voice.status = crate::models::VoiceConnectionStatus::Connected;
                    cx.notify();
                }
            });
        });
        
        cx.notify();
    }
    
    pub(crate) fn confirm_voice_switch(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((channel, server)) = self.show_voice_switch_warning.take() {
            // Leave current voice channel
            self.voice_state = None;
            // Join new voice channel
            self.join_voice_channel(&channel, server.as_ref(), window, cx);
        }
        cx.notify();
    }
    
    pub(crate) fn cancel_voice_switch(&mut self, cx: &mut Context<Self>) {
        self.show_voice_switch_warning = None;
        cx.notify();
    }
    
    pub(crate) fn leave_voice_channel(&mut self, cx: &mut Context<Self>) {
        self.voice_state = None;
        cx.notify();
    }
    
    pub(crate) fn toggle_mute(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut voice) = self.voice_state {
            voice.is_muted = !voice.is_muted;
            // If deafened, unmute  doesn't work
            if voice.is_deafened {
                return;
            }
            cx.notify();
        }
    }
    
    pub(crate) fn toggle_deafen(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut voice) = self.voice_state {
            voice.is_deafened = !voice.is_deafened;
            // Deafening also mutes
            if voice.is_deafened {
                voice.is_muted = true;
            }
            cx.notify();
        }
    }
    
    pub(crate) fn toggle_video(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut voice) = self.voice_state {
            voice.is_video_enabled = !voice.is_video_enabled;
            cx.notify();
        }
    }
    
    pub(crate) fn toggle_screen_share(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut voice) = self.voice_state {
            voice.is_screen_sharing = !voice.is_screen_sharing;
            cx.notify();
        }
    }
    
    pub(crate) fn is_in_voice_channel(&self, channel_id: &str) -> bool {
        self.voice_state
            .as_ref()
            .map(|v| v.channel_id == channel_id && v.is_connected())
            .unwrap_or(false)
    }
}

impl Focusable for TripwireApp {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// ── Render ────────────────────────────────────────────────────────────────────

use gpui::{IntoElement, Render};
use gpui::Styled;
use gpui::ParentElement;
use gpui_component::v_flex;

impl Render for TripwireApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(self.titlebar.clone())
            .child(if self.auth.is_authenticated() {
                self.render_app(window, cx).into_any_element()
            } else {
                self.render_auth(window, cx).into_any_element()
            })
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn timestamp_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("msg_{nanos}")
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        result.push(CHARS[(buf[0] >> 2) as usize] as char);
        result.push(CHARS[(((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(CHARS[(((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize] as char);
            if chunk.len() > 2 {
                result.push(CHARS[(buf[2] & 0x3f) as usize] as char);
            } else {
                result.push('=');
            }
        } else {
            result.push('=');
            result.push('=');
        }
    }
    
    result
}
