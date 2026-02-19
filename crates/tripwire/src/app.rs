use std::collections::HashMap;

use gpui::{App, Context, Entity, FocusHandle, Focusable, Subscription, Window};
use gpui_component::input::{InputEvent, InputState};
use gpui::AppContext;
use crate::auth_state::AuthState;
use crate::mock_data;
use crate::models::{Attachment, Channel, ChannelKind, DirectMessageChannel, Message, MessageReply, Server, User, UserProfile};
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

    pub(crate) fn logout(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.auth.logout();
        // Clear message input value happens implicitly since we reset auth
        cx.notify();
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
