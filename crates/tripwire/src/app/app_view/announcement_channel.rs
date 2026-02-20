//! Announcement channel view - Read-only with emphasized messages

use gpui::{
    AnyElement, Context, ElementId, InteractiveElement, ParentElement as _, SharedString, Styled,
    Window, div, px, IntoElement,
};
use gpui_component::{
    h_flex, v_flex, ActiveTheme as _, Icon, IconName, Sizable as _,
    avatar::Avatar,
    scroll::ScrollableElement,
};

use crate::app::TripwireApp;
use crate::models::Message;

impl TripwireApp {
    pub(crate) fn render_announcement_channel_ui(
        &mut self,
        _channel_name: &str,
        messages: &[Message],
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        v_flex()
            .flex_1()
            .overflow_hidden()
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_3()
                    .bg(cx.theme().accent)
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .flex_shrink_0()
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .child(
                                Icon::new(IconName::Bell)
                                    .text_color(cx.theme().accent_foreground)
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().accent_foreground)
                                    .child("This is an announcement channel. Only admins can post.")
                            )
                    )
            )
            .child(
                div()
                    .id("announcement-scroll")
                    .flex_1()
                    .overflow_y_scrollbar()
                    .p_4()
                    .children(messages.iter().map(|msg| {
                        self.render_announcement_message(msg, cx)
                    }))
            )
            .into_any_element()
    }

    fn render_announcement_message(&self, message: &Message, cx: &Context<Self>) -> AnyElement {
        div()
            .id(ElementId::Name(SharedString::from(message.id.clone())))
            .w_full()
            .mb_4()
            .child(
                div()
                    .w_full()
                    .p_4()
                    .rounded(cx.theme().radius_lg)
                    .bg(cx.theme().muted)
                    .border_2()
                    .border_color(cx.theme().accent)
                    .child(
                        v_flex()
                            .w_full()
                            .gap_3()
                            .child(
                                h_flex()
                                    .gap_3()
                                    .items_center()
                                    .child(
                                        Avatar::new()
                                            .name(message.author.username.clone())
                                            .size(px(48.0))
                                    )
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .text_base()
                                                    .font_weight(gpui::FontWeight::BOLD)
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
                            )
                            .child(
                                div()
                                    .pl(px(60.0))
                                    .text_sm()
                                    .line_height(px(22.0))
                                    .text_color(cx.theme().foreground)
                                    .child(message.content.clone())
                            )
                            .child({
                                let mut content = v_flex();
                                
                                if let Some(attachment) = &message.attachment {
                                    content = content.child(
                                        div()
                                            .pl(px(60.0))
                                            .child(
                                                div()
                                                    .w_full()
                                                    .max_w(px(400.0))
                                                    .rounded(cx.theme().radius)
                                                    .overflow_hidden()
                                                    .border_1()
                                                    .border_color(cx.theme().border)
                                                    .child(
                                                        h_flex()
                                                            .p_3()
                                                            .gap_2()
                                                            .items_center()
                                                            .bg(cx.theme().sidebar)
                                                            .child(
                                                                Icon::new(IconName::Settings)
                                                                    .xsmall()
                                                                    .text_color(cx.theme().muted_foreground)
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(cx.theme().foreground)
                                                                    .child(attachment.filename.clone())
                                                            )
                                                    )
                                            )
                                    );
                                }
                                
                                if !message.reactions.is_empty() {
                                    content = content.child(
                                        h_flex()
                                            .pl(px(60.0))
                                            .gap_2()
                                            .flex_wrap()
                                            .children(message.reactions.iter().map(|(emoji, users)| {
                                                div()
                                                    .px_3()
                                                    .py_1()
                                                    .rounded_full()
                                                    .bg(cx.theme().muted)
                                                    .border_1()
                                                    .border_color(cx.theme().border)
                                                    .cursor_pointer()
                                                    .hover(|s| s.bg(cx.theme().accent))
                                                    .child(
                                                        h_flex()
                                                            .gap_1()
                                                            .items_center()
                                                            .child(
                                                                div()
                                                                    .text_sm()
                                                                    .child(emoji.clone())
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                                                    .text_color(cx.theme().foreground)
                                                                    .child(users.len().to_string())
                                                            )
                                                    )
                                                    .into_any_element()
                                            }))
                                    );
                                }
                                
                                content
                            })
                    )
            )
            .into_any_element()
    }
}