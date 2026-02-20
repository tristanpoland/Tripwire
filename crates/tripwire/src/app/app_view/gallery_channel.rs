//! Gallery/Media channel view - Grid layout for images and videos

use gpui::{
    AnyElement, Context, InteractiveElement, ParentElement, SharedString, Styled, Window, div, px,
    IntoElement,
};
use gpui_component::{
    h_flex, v_flex, ActiveTheme as _, Icon, IconName, Sizable as _,
    button::{Button, ButtonVariants as _},
    scroll::ScrollableElement,
};

use crate::app::TripwireApp;
use crate::models::Message;

impl TripwireApp {
    pub(crate) fn render_gallery_channel_ui(
        &mut self,
        _channel_name: &str,
        messages: &[Message],
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // Filter messages to only those with attachments (images/videos)
        let media_messages: Vec<&Message> = messages
            .iter()
            .filter(|m| m.attachment.is_some())
            .collect();

        v_flex()
            .flex_1()
            .overflow_y_scrollbar()
            .p_4()
            .gap_4()
            .child(
                // Upload area at top
                div()
                    .w_full()
                    .p_6()
                    .rounded(cx.theme().radius_lg)
                    .border_2()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().muted)
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .cursor_pointer()
                    .hover(|s| s.bg(cx.theme().accent).opacity(0.1))
                    .child(
                        Icon::new(IconName::Plus)
                            .size(px(48.0))
                            .text_color(cx.theme().muted_foreground)
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().foreground)
                            .child("Drop files to upload")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child("or click to browse")
                    )
            )
            .child(
                // Filter/Sort controls
                h_flex()
                    .w_full()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("{} items", media_messages.len()))
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("gallery-filter")
                                    .label("Filter")
                                    .icon(IconName::Settings)
                                    .ghost()
                                    .xsmall()
                            )
                            .child(
                                Button::new("gallery-view-grid")
                                    .icon(IconName::User)
                                    .ghost()
                                    .xsmall()
                            )
                    )
            )
            .child(self.render_media_grid(&media_messages, cx))
            .into_any_element()
    }

    fn render_media_grid(&self, media_messages: &[&Message], cx: &Context<Self>) -> AnyElement {
        let mut grid_items = Vec::new();

        for message in media_messages {
            if let Some(attachment) = &message.attachment {
                let item_id = format!("{}-media", message.id);
                
                grid_items.push(
                    div()
                        .id(SharedString::from(item_id.clone()))
                        .w(px(200.0))
                        .h(px(200.0))
                        .rounded(cx.theme().radius_lg)
                        .overflow_hidden()
                        .bg(cx.theme().muted)
                        .border_1()
                        .border_color(cx.theme().border)
                        .cursor_pointer()
                        .hover(|s| s.border_color(cx.theme().accent))
                        .relative()
                        .child(
                            div()
                                .size_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .bg(cx.theme().sidebar)
                                .child(
                                    Icon::new(if attachment.filename.ends_with(".mp4") 
                                        || attachment.filename.ends_with(".webm") {
                                        IconName::Settings
                                    } else {
                                        IconName::User
                                    })
                                    .size(px(48.0))
                                    .text_color(cx.theme().muted_foreground)
                                )
                        )
                        .child(
                            div()
                                .absolute()
                                .bottom_0()
                                .left_0()
                                .right_0()
                                .p_2()
                                .bg(gpui::rgba(0x000000CC))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(gpui::white())
                                        .overflow_hidden()
                                        .text_ellipsis()
                                        .child(attachment.filename.clone())
                                )
                        )
                );
            }
        }

        div()
            .w_full()
            .flex()
            .flex_wrap()
            .gap_4()
            .children(grid_items.into_iter().take(24))
            .into_any_element()
    }
}
