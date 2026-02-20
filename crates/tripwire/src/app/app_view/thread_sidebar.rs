//! Thread sidebar - shows a message thread with original message and replies

use gpui::{
    div, px, AnyElement, Context, IntoElement,
    InteractiveElement, MouseButton, ParentElement, Styled, Window,
};
use gpui_component::{
    h_flex, v_flex, ActiveTheme as _, IconName, Sizable as _,
    avatar::Avatar,
    button::{Button, ButtonVariants},
    input::Input,
    scroll::ScrollableElement as _,
    StyledExt,
};

use crate::app::TripwireApp;

impl TripwireApp {
    pub(crate) fn render_thread_sidebar(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // Voice chat sidebar mode takes priority
        if self.show_voice_chat_sidebar && self.is_in_voice() {
            return self.render_voice_chat_sidebar(window, cx);
        }
        
        // Regular thread mode
        let thread_id = self.open_thread_id.as_ref()?;
        
        // Get the parent message
        let parent_message = self.get_message_by_id(thread_id)?;
        
        // Get thread messages
        let thread_messages = self.thread_messages.get(thread_id)
            .map(|msgs| msgs.clone())
            .unwrap_or_default();
        
        Some(
            v_flex()
                .w(px(420.0))
                .h_full()
                .bg(cx.theme().background)
                .border_l_1()
                .border_color(cx.theme().border)
                .child(
                    // Header
                    h_flex()
                        .flex_shrink_0()
                        .h(px(48.0))
                        .px_4()
                        .items_center()
                        .justify_between()
                        .border_b_1()
                        .border_color(cx.theme().border)
                        .child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .text_color(cx.theme().foreground)
                                        .child("Thread")
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(cx.theme().muted_foreground)
                                        .child(format!("{} {}", 
                                            thread_messages.len(),
                                            if thread_messages.len() == 1 { "reply" } else { "replies" }
                                        ))
                                )
                        )
                        .child(
                            Button::new("btn-close-thread")
                                .icon(IconName::Close)
                                .ghost()
                                .xsmall()
                                .tooltip("Close thread")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.close_thread(cx);
                                }))
                        )
                )
                .child(
                    // Thread content
                    v_flex()
                        .flex_1()
                        .min_h_0()
                        .overflow_hidden()
                        .child(
                            div()
                                .flex_1()
                                .overflow_y_scrollbar()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .p_4()
                                        // Original message
                                        .child(
                                            v_flex()
                                                .gap_1()
                                                .p_4()
                                                .rounded(cx.theme().radius)
                                                .bg(cx.theme().sidebar)
                                                .border_1()
                                                .border_color(cx.theme().border)
                                                .child(
                                                    h_flex()
                                                        .gap_3()
                                                        .items_start()
                                                        .child(
                                                            div()
                                                                .flex_shrink_0()
                                                                .child(
                                                                    Avatar::new()
                                                                        .name(parent_message.author.username.clone())
                                                                        .with_size(gpui_component::Size::Medium)
                                                                )
                                                        )
                                                        .child(
                                                            v_flex()
                                                                .flex_1()
                                                                .gap_1()
                                                                .child(
                                                                    h_flex()
                                                                        .gap_2()
                                                                        .items_baseline()
                                                                        .child(
                                                                            div()
                                                                                .text_sm()
                                                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                                                .text_color(cx.theme().foreground)
                                                                                .child(parent_message.author.username.clone())
                                                                        )
                                                                        .child(
                                                                            div()
                                                                                .text_xs()
                                                                                .text_color(cx.theme().muted_foreground)
                                                                                .child(parent_message.timestamp.clone())
                                                                        )
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_sm()
                                                                        .text_color(cx.theme().foreground)
                                                                        .child(parent_message.content.clone())
                                                                )
                                                        )
                                                )
                                        )
                                        // Thread replies divider
                                        .child(
                                            h_flex()
                                                .items_center()
                                                .gap_2()
                                                .py_2()
                                                .child(
                                                    div()
                                                        .h(px(1.0))
                                                        .flex_1()
                                                        .bg(cx.theme().border)
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                                        .text_color(cx.theme().muted_foreground)
                                                        .child(format!("{} {}", 
                                                            thread_messages.len(),
                                                            if thread_messages.len() == 1 { "Reply" } else { "Replies" }
                                                        ))
                                                )
                                                .child(
                                                    div()
                                                        .h(px(1.0))
                                                        .flex_1()
                                                        .bg(cx.theme().border)
                                                )
                                        )
                                        // Thread messages
                                        .children(thread_messages.iter().enumerate().map(|(idx, msg)| {
                                            self.render_thread_message(msg, idx, window, cx)
                                        }))
                                )
                        )
                )
                .child(
                    // Thread reply input
                    self.render_thread_composer(window, cx)
                )
                .into_any_element()
        )
    }
    
    fn render_voice_chat_sidebar(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // Get messages for the active voice channel
        let messages = self.active_channel_id.as_ref()
            .and_then(|id| self.messages.get(id))
            .map(|msgs| msgs.clone())
            .unwrap_or_default();
        
        Some(
            v_flex()
                .w(px(420.0))
                .h_full()
                .bg(cx.theme().background)
                .border_l_1()
                .border_color(cx.theme().border)
                .child(
                    // Header
                    h_flex()
                        .flex_shrink_0()
                        .h(px(48.0))
                        .px_4()
                        .items_center()
                        .justify_between()
                        .border_b_1()
                        .border_color(cx.theme().border)
                        .child(
                            div()
                                .text_lg()
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_color(cx.theme().foreground)
                                .child("Voice Chat")
                        )
                        .child(
                            Button::new("btn-close-voice-chat")
                                .icon(IconName::Close)
                                .ghost()
                                .xsmall()
                                .tooltip("Close voice chat")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.close_voice_chat_sidebar(cx);
                                }))
                        )
                )
                .child(
                    // Messages
                    div()
                        .flex_1()
                        .min_h_0()
                        .overflow_hidden()
                        .child(
                            div()
                                .flex_1()
                                .overflow_y_scrollbar()
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .p_3()
                                        .children(messages.iter().enumerate().map(|(idx, msg)| {
                                            self.render_thread_message(msg, idx, window, cx)
                                        }))
                                )
                        )
                )
                .child(
                    // Voice chat input
                    self.render_voice_chat_composer(window, cx)
                )
                .into_any_element()
        )
    }
    
    fn render_voice_chat_composer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        v_flex()
            .flex_shrink_0()
            .p_4()
            .border_t_1()
            .border_color(cx.theme().border)
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        div()
                            .px_3()
                            .py_3()
                            .rounded(cx.theme().radius)
                            .bg(cx.theme().muted)
                            .border_1()
                            .border_color(cx.theme().border)
                            .child(
                                Input::new(&self.voice_chat_input).appearance(false)
                            )
                    )
            )
            .into_any_element()
    }
    
    fn render_thread_message(
        &self,
        message: &crate::models::Message,
        index: usize,
        _window: &mut Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        h_flex()
            .gap_3()
            .py_3()
            .px_3()
            .rounded(cx.theme().radius)
            .hover(|s| s.bg(cx.theme().accent))
            .child(
                div()
                    .flex_shrink_0()
                    .child(
                        Avatar::new()
                            .name(message.author.username.clone())
                            .with_size(gpui_component::Size::Small)
                    )
            )
            .child(
                v_flex()
                    .flex_1()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_baseline()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(cx.theme().foreground)
                                    .child(message.author.username.clone())
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(message.timestamp.clone())
                            )
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().foreground)
                            .child(message.content.clone())
                    )
            )
            .into_any_element()
    }
    
    fn render_thread_composer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        v_flex()
            .flex_shrink_0()
            .p_4()
            .border_t_1()
            .border_color(cx.theme().border)
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        div()
                            .px_3()
                            .py_3()
                            .rounded(cx.theme().radius)
                            .bg(cx.theme().muted)
                            .border_1()
                            .border_color(cx.theme().border)
                            .child(
                                Input::new(&self.thread_input).appearance(false)
                            )
                    )
                    .child(
                        h_flex()
                            .justify_end()
                            .child(
                                Button::new("btn-send-thread-reply")
                                    .label("Reply")
                                    .icon(IconName::ArrowRight)
                                    .primary()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.send_thread_reply(window, cx);
                                    }))
                            )
                    )
            )
            .into_any_element()
    }
    
    // ── Thread actions ────────────────────────────────────────────────────
    
    pub(crate) fn open_thread(&mut self, message_id: String, cx: &mut Context<Self>) {
        self.open_thread_id = Some(message_id.clone());
        
        // Load thread messages from mock data if not already loaded
        if !self.thread_messages.contains_key(&message_id) {
            self.thread_messages.insert(message_id, vec![]);
        }
        
        cx.notify();
    }
    
    pub(crate) fn close_thread(&mut self, cx: &mut Context<Self>) {
        self.open_thread_id = None;
        cx.notify();
    }
    
    pub(crate) fn close_voice_chat_sidebar(&mut self, cx: &mut Context<Self>) {
        self.show_voice_chat_sidebar = false;
        cx.notify();
    }
    
    pub(crate) fn open_voice_chat_sidebar(&mut self, cx: &mut Context<Self>) {
        // Close any open thread first
        self.open_thread_id = None;
        self.show_voice_chat_sidebar = true;
        cx.notify();
    }
    
    pub(crate) fn is_in_voice(&self) -> bool {
        self.voice_state.is_some()
    }
    
    pub(crate) fn send_thread_reply(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let thread_id = match &self.open_thread_id {
            Some(id) => id.clone(),
            None => return,
        };
        
        let content = self.thread_input.read(cx).text().to_string();
        if content.trim().is_empty() {
            return;
        }
        
        // Create new message in thread
        let new_message = crate::models::Message {
            id: format!("thread-msg-{}", uuid::Uuid::new_v4()),
            author: crate::models::User {
                id: "current-user".to_string(),
                username: "You".to_string(),
                discriminator: "0001".to_string(),
                status: crate::models::UserStatus::Online,
            },
            content,
            timestamp: "Just now".to_string(),
            edited: false,
            edited_timestamp: None,
            attachment: None,
            reactions: std::collections::HashMap::new(),
            reply_to: None,
            mentioned_users: vec![],
            pinned: false,
            thread_id: Some(thread_id.clone()),
            thread_count: 0,
            created_at: std::time::SystemTime::now(),
        };
        
        self.thread_messages
            .entry(thread_id.clone())
            .or_insert_with(Vec::new)
            .push(new_message);
        
        // Update thread count on parent message
        if let Some(parent) = self.get_message_by_id_mut(&thread_id) {
            parent.thread_count += 1;
        }
        
        // Clear input
        self.thread_input.update(cx, |input, cx| {
            input.set_value("", window, cx);
        });
        
        cx.notify();
    }
    
    fn get_message_by_id(&self, message_id: &str) -> Option<&crate::models::Message> {
        // Search in channel messages
        for msgs in self.messages.values() {
            if let Some(msg) = msgs.iter().find(|m| m.id == message_id) {
                return Some(msg);
            }
        }
        
        // Search in DM messages
        for msgs in self.dm_messages.values() {
            if let Some(msg) = msgs.iter().find(|m| m.id == message_id) {
                return Some(msg);
            }
        }
        
        None
    }
    
    fn get_message_by_id_mut(&mut self, message_id: &str) -> Option<&mut crate::models::Message> {
        // Search in channel messages
        for msgs in self.messages.values_mut() {
            if let Some(msg) = msgs.iter_mut().find(|m| m.id == message_id) {
                return Some(msg);
            }
        }
        
        // Search in DM messages
        for msgs in self.dm_messages.values_mut() {
            if let Some(msg) = msgs.iter_mut().find(|m| m.id == message_id) {
                return Some(msg);
            }
        }
        
        None
    }
}
