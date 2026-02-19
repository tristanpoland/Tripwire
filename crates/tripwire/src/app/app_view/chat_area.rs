//! Main chat area — channel header, scrollable message list, and message input.

use gpui::{
    AnyElement, Context, ElementId, IntoElement as _, SharedString, Window, div,
    prelude::FluentBuilder as _, px, StyledImage as _,
};
use gpui::InteractiveElement;
use gpui::StatefulInteractiveElement;
use gpui_component::button::ButtonVariants;
use gpui::ParentElement;
use gpui_component::StyledExt;
use gpui::Styled;
use gpui_component::{
    ActiveTheme as _, IconName, Sizable as _,
    avatar::Avatar,
    button::Button,
    h_flex, v_flex,
    input::Input,
    scroll::ScrollableElement as _,
    tooltip::Tooltip,
};

use crate::app::{AppView, TripwireApp};
use crate::models::Message;

impl TripwireApp {
    pub(crate) fn render_chat_area(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match self.current_view {
            AppView::Servers => {
                let channel_name = self.active_channel_name().unwrap_or("general").to_string();
                let channel_topic = self.active_channel_topic().map(|t| t.to_string());
                let messages: Vec<Message> = self.active_messages().to_vec();
                let channel_kind = self.active_channel_kind();
                let members_connected = self.active_channel().map(|c| c.members_connected).unwrap_or(0);

                v_flex()
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .overflow_hidden()
                    .bg(cx.theme().background)
                    .child(self.render_channel_header(
                        &channel_name,
                        channel_topic.as_deref(),
                        channel_kind,
                        members_connected,
                        cx,
                    ))
                    .child(self.render_message_list(&messages, cx))
                    .child(self.render_message_composer(&channel_name, window, cx))
                    .into_any_element()
            }
            AppView::DirectMessages => {
                let dm_name = self
                    .active_dm_id
                    .as_ref()
                    .and_then(|id| {
                        self.dm_channels
                            .iter()
                            .find(|dm| dm.id == *id)
                            .map(|dm| dm.recipient.username.clone())
                    })
                    .unwrap_or_else(|| "Select a DM".to_string());
                let messages: Vec<Message> = self.active_dm_messages().to_vec();

                v_flex()
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .overflow_hidden()
                    .bg(cx.theme().background)
                    .child(self.render_dm_header(&dm_name, cx))
                    .child(self.render_message_list(&messages, cx))
                    .child(self.render_message_composer(&dm_name, window, cx))
                    .into_any_element()
            }
        }
    }

    fn render_channel_header(
        &self,
        channel_name: &str,
        topic: Option<&str>,
        channel_kind: Option<crate::models::ChannelKind>,
        members_connected: usize,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        use crate::models::ChannelKind;
        
        let show_voice_info = matches!(
            channel_kind,
            Some(ChannelKind::Voice) | Some(ChannelKind::Stage)
        ) && members_connected > 0;

        h_flex()
            .h(px(48.))
            .flex_shrink_0()
            .px_4()
            .gap_3()
            .items_center()
            .border_b_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            // Channel icon
            .when_some(channel_kind.clone(), |this, kind| {
                this.child(
                    gpui_component::Icon::new(kind.icon())
                        .small()
                        .text_color(cx.theme().muted_foreground),
                )
            })
            // Channel name
            .child(
                div()
                    .text_base()
                    .font_semibold()
                    .text_color(cx.theme().foreground)
                    .child(channel_name.to_string()),
            )
            // Voice channel member count
            .when(show_voice_info, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_1()
                        .rounded(cx.theme().radius)
                        .bg(cx.theme().accent)
                        .child(
                            gpui_component::Icon::new(IconName::User)
                                .xsmall()
                                .text_color(cx.theme().foreground),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_semibold()
                                .text_color(cx.theme().foreground)
                                .child(members_connected.to_string()),
                        ),
                )
            })
            // Divider
            .when(topic.is_some(), |this| {
                this.child(div().w(px(1.)).h(px(20.)).bg(cx.theme().border))
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
                    .tooltip("Search")
                    .on_click(|_, _, _| {}),
            )
            .child(
                Button::new("btn-toggle-members")
                    .icon(IconName::PanelRight)
                    .ghost()
                    .xsmall()
                    .tooltip("Toggle Member List")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.show_members = !this.show_members;
                        cx.notify();
                    })),
            )
    }

    fn render_dm_header(
        &self,
        recipient_name: &str,
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
            // @ symbol for DMs
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .text_color(cx.theme().muted_foreground)
                    .child("@"),
            )
            // Recipient name
            .child(
                div()
                    .text_base()
                    .font_semibold()
                    .text_color(cx.theme().foreground)
                    .child(recipient_name.to_string()),
            )
            // Spacer
            .child(div().flex_1())
            // Toolbar buttons
            .child(
                Button::new("btn-search-dms")
                    .icon(IconName::Search)
                    .ghost()
                    .xsmall()
                    .tooltip("Search")
                    .on_click(|_, _, _| {}),
            )
    }

    fn render_message_list(
        &self,
        messages: &[Message],
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        // Pre-compute elements to avoid FnMut borrow-checker issues with cx.
        let mut message_elements: Vec<gpui::AnyElement> = Vec::new();
        for (ix, msg) in messages.iter().enumerate() {
            message_elements.push(self.render_message(ix, msg, cx).into_any_element());
        }

        div()
            .flex_1()
            .overflow_y_scrollbar()
            .px_4()
            .py_4()
            .children(message_elements)
    }

    fn render_message(
        &self,
        index: usize,
        msg: &Message,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let author_name = msg.author.username.clone();
        let avatar_name = author_name.clone();
        let content = msg.content.clone();
        let timestamp = msg.timestamp.clone();
        let is_edited = msg.edited;
        let has_attachment = msg.attachment.is_some();

        h_flex()
            .id(ElementId::Name(SharedString::from(format!("msg-{index}"))))
            .gap_3()
            .py_2()
            .px_3()
            .items_start()
            .rounded(cx.theme().radius)
            .hover(|s| s.bg(cx.theme().accent))
            // Avatar
            .child(
                div()
                    .flex_shrink_0()
                    .child(
                        Avatar::new()
                            .name(avatar_name)
                            .with_size(gpui_component::Size::Medium),
                    ),
            )
            // Content block
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_1()
                    // Author + timestamp row
                    .child(
                        h_flex()
                            .gap_2()
                            .items_baseline()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
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
                                        .italic()
                                        .child("(edited)"),
                                )
                            }),
                    )
                    // Message body
                    .when(!content.is_empty(), |this| {
                        this.child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().foreground)
                                .child(content),
                        )
                    })
                    // Attachment (if present)
                    .when(has_attachment, |this| {
                        if let Some(ref attachment) = msg.attachment {
                            this.child(self.render_attachment(attachment, cx))
                        } else {
                            this
                        }
                    }),
            )
    }

    fn render_attachment(
        &self,
        attachment: &crate::models::Attachment,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let data_url = format!("data:{};base64,{}", attachment.mime_type, attachment.base64_data);
        
        div()
            .mt_2()
            .max_w(px(400.0))
            .rounded(cx.theme().radius)
            .border_1()
            .border_color(cx.theme().border)
            .overflow_hidden()
            .cursor_pointer()
            .child(
                gpui::img(data_url)
                    .w_full()
                    .object_fit(gpui::ObjectFit::Cover)
            )
    }

    fn render_message_composer(
        &self,
        channel_name: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let _ = channel_name;
        let has_attachment = self.pending_attachment.is_some();
        
        v_flex()
            .flex_shrink_0()
            .gap_2()
            .child(
                // Attachment preview
                if let Some(ref attachment) = self.pending_attachment {
                    div()
                        .px_4()
                        .child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .px_3()
                                .py_2()
                                .rounded(cx.theme().radius)
                                .bg(cx.theme().muted)
                                .border_1()
                                .border_color(cx.theme().border)
                                .child(
                                    div()
                                        .flex_1()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(cx.theme().foreground)
                                                .child(format!("📎 {}", attachment.filename))
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(cx.theme().muted_foreground)
                                                .child(format!("{:.2} MB", attachment.size_mb()))
                                        )
                                )
                                .child(
                                    Button::new("btn-remove-attachment")
                                        .icon(IconName::Close)
                                        .ghost()
                                        .xsmall()
                                        .tooltip("Remove attachment")
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.clear_attachment(cx);
                                        })),
                                )
                        )
                        .into_any_element()
                } else {
                    div().into_any_element()
                }
            )
            .child(
                h_flex()
                    .px_4()
                    .pb_4()
                    .when(!has_attachment, |this| this.pt_2())
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .px_3()
                            .py_3()
                            .gap_2()
                            .rounded(cx.theme().radius_lg)
                            .bg(cx.theme().popover)
                            .border_1()
                            .border_color(cx.theme().border)
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    // Attachment button
                                    .child(
                                        Button::new("btn-attach")
                                            .icon(IconName::Plus)
                                            .ghost()
                                            .xsmall()
                                            .tooltip("Attach File")
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.attach_file(window, cx);
                                            })),
                                    )
                                    // Text input
                                    .child(div().flex_1().child(Input::new(&self.message_input).appearance(false)))
                                    // Emoji button
                                    .child(
                                        Button::new("btn-emoji")
                                            .icon(IconName::Star)
                                            .ghost()
                                            .xsmall()
                                            .tooltip("Emoji")
                                            .on_click(|_, _, _| {}),
                                    ),
                            ),
                    )
                    // Send button (outside input box)
                    .child(
                        Button::new("btn-send")
                            .icon(IconName::ArrowRight)
                            .primary()
                            .small()
                            .tooltip("Send Message")
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.send_message(window, cx);
                            })),
                    )
            )
    }
}
