use std::collections::HashMap;

use gpui::{App, Context, Entity, FocusHandle, Focusable, Subscription, Window};
use gpui_component::input::{InputEvent, InputState};
use gpui::AppContext;
use crate::auth_state::AuthState;
use crate::mock_data;
use crate::models::{Channel, ChannelKind, DirectMessageChannel, Message, Server};

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

    pub(crate) _subscriptions: Vec<Subscription>,
}

impl TripwireApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let auth = AuthState::new();

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
        if content.is_empty() {
            return;
        }
        
        if let Some(user) = self.auth.current_user.clone() {
            let msg = Message {
                id: timestamp_id(),
                author: user,
                content,
                timestamp: "Just now".to_string(),
                edited: false,
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

impl Render for TripwireApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.auth.is_authenticated() {
            self.render_app(window, cx).into_any_element()
        } else {
            self.render_auth(window, cx).into_any_element()
        }
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
