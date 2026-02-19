//! Main chat area — channel header, scrollable message list, and message input.

use gpui::{
    AnyElement, Context, ElementId, IntoElement as _, SharedString, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable as _,
    avatar::Avatar,
    button::Button,
    h_flex, v_flex,
    input::Input,
    tooltip::Tooltip,
};

use crate::app::TripwireApp;
use crate::models::Message;

impl TripwireApp {
    pub(crate) fn render_chat_area(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let channel_name = self.active_channel_name().unwrap_or("general").to_string();
        let channel_topic = self
            .active_channel_topic()
            .map(|t| t.to_string());
        let messages: Vec<Message> = self.active_messages().to_vec();

        v_flex()
            .flex_1()
            .h_full()
            .min_w_0()
            .overflow_hidden()
            .bg(cx.theme().background)
            // ── Channel header ───────────────────────────────────────────────
            .child(self.render_channel_header(&channel_name, channel_topic.as_deref(), cx))
            // ── Message list ─────────────────────────────────────────────────
            .child(self.render_message_list(&messages, cx))
            // ── Message composer ─────────────────────────────────────────────
            .child(self.render_message_composer(&channel_name, window, cx))
            .into_any_element()
    }

    fn render_channel_header(
        &self,
        channel_name: &str,
        topic: Option<&str>,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        h_flex()
            .h(px(48.))
            .flex_shrink_0()
            .px_4()
            .gap_3()
            .items_center()
            .border_b_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            // # icon + channel name
            .child(
                div()
                    .text_lg()
                    .text_color(cx.theme().muted_foreground)
                    .child("#"),
            )
            .child(
                div()
                    .text_sm()
                    .font_semibold()
                    .text_color(cx.theme().foreground)
                    .child(channel_name.to_string()),
            )
            // Divider
            .when(topic.is_some(), |this| {
                this.child(
                    div()
                        .w(px(1.))
                        .h(px(16.))
                        .bg(cx.theme().border),
                )
            })
            // Topic
            .when_some(topic, |this, t| {
                this.child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .overflow_hidden()
                        .text_ellipsis()
                        .child(t.to_string()),
                )
            })
            // Spacer
            .child(div().flex_1())
            // Toolbar buttons
            .child(
                Button::new("btn-search-msgs")
                    .icon(IconName::Search)
                    .ghost()
                    .xsmall()
                    .tooltip(|window, cx| Tooltip::text("Search", window, cx))
                    .on_click(|_, _, _| {}),
            )
            .child(
                Button::new("btn-toggle-members")
                    .icon(IconName::PanelRight)
                    .ghost()
                    .xsmall()
                    .tooltip(|window, cx| Tooltip::text("Toggle Member List", window, cx))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.show_members = !this.show_members;
                        cx.notify();
                    })),
            )
    }

    fn render_message_list(
        &self,
        messages: &[Message],
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex_1()
            .overflow_y_scroll()
            .px_4()
            .py_4()
            .children(
                messages
                    .iter()
                    .enumerate()
                    .map(|(ix, msg)| self.render_message(ix, msg, cx)),
            )
    }

    fn render_message(
        &self,
        index: usize,
        msg: &Message,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let author_name = msg.author.username.clone();
        let content = msg.content.clone();
        let timestamp = msg.timestamp.clone();
        let is_edited = msg.edited;

        h_flex()
            .id(ElementId::Name(SharedString::from(format!("msg-{index}"))))
            .gap_3()
            .py_1()
            .px_2()
            .items_start()
            .rounded(cx.theme().radius)
            .hover(|s| s.bg(cx.theme().accent))
            // Avatar
            .child(
                Avatar::new()
                    .name(author_name.as_str())
                    .with_size(gpui_component::Size::Medium),
            )
            // Content block
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_0()
                    // Author + timestamp
                    .child(
                        h_flex()
                            .gap_2()
                            .items_baseline()
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .text_color(cx.theme().foreground)
                                    .child(author_name),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(timestamp),
                            )
                            .when(is_edited, |this| {
                                this.child(
                                    div()
                                        .text_xs()
                                        .text_color(cx.theme().muted_foreground)
                                        .child("(edited)"),
                                )
                            }),
                    )
                    // Message body
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().foreground)
                            .child(content),
                    ),
            )
    }

    fn render_message_composer(
        &self,
        channel_name: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        h_flex()
            .flex_shrink_0()
            .mx_4()
            .mb_4()
            .px_4()
            .py_2()
            .gap_2()
            .items_center()
            .rounded(cx.theme().radius_lg)
            .bg(cx.theme().card)
            .border_1()
            .border_color(cx.theme().border)
            // Attachment button (placeholder)
            .child(
                Button::new("btn-attach")
                    .icon(IconName::Plus)
                    .ghost()
                    .xsmall()
                    .tooltip(|window, cx| Tooltip::text("Attach File", window, cx))
                    .on_click(|_, _, _| {}),
            )
            // Text input — fills remaining space
            .child(
                Input::new(&self.message_input)
                    .appearance(false)
                    .flex_1(),
            )
            // Emoji placeholder
            .child(
                Button::new("btn-emoji")
                    .icon(IconName::Star)
                    .ghost()
                    .xsmall()
                    .tooltip(|window, cx| Tooltip::text("Emoji", window, cx))
                    .on_click(|_, _, _| {}),
            )
            // Send button
            .child(
                Button::new("btn-send")
                    .icon(IconName::ArrowRight)
                    .primary()
                    .xsmall()
                    .tooltip(|window, cx| Tooltip::text("Send Message", window, cx))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.send_message(window, cx);
                    })),
            )
    }
}
