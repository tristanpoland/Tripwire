//! Main chat area — channel header, scrollable message list, and message input.

use gpui::{
    AnyElement, Context, ElementId, IntoElement as _, SharedString, Window, div,
    prelude::FluentBuilder as _, px, StyledImage as _, IntoElement,
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
        let author_user = msg.author.clone();
        let avatar_name = author_name.clone();
        let avatar_user = author_user.clone();
        let content = msg.content.clone();
        let timestamp = msg.timestamp.clone();
        let is_edited = msg.edited;
        let has_attachment = msg.attachment.is_some();
        let message_id = msg.id.clone();
        let reactions = msg.reactions.clone();
        let user_id = self.auth.current_user.as_ref().map(|u| u.id.clone()).unwrap_or_default();
        let reply_to = msg.reply_to.clone();
        let is_reply = msg.is_reply();

        div()
            .relative()
            .group("message-hover")
            .child(
                v_flex()
                    .gap_1()
                    // Reply preview (if this is a reply)
                    .when(is_reply, |this| {
                        if let Some(reply) = reply_to.as_ref() {
                            let reply_author = reply.author.username.clone();
                            let reply_content = reply.content_preview.clone();
                            this.child(
                                h_flex()
                                    .ml(px(56.0))
                                    .gap_2()
                                    .items_center()
                                    .child(
                                        div()
                                            .w(px(2.0))
                                            .h(px(12.0))
                                            .rounded(px(1.0))
                                            .bg(cx.theme().muted_foreground)
                                    )
                                    .child(
                                        gpui_component::Icon::new(IconName::ArrowRight)
                                            .xsmall()
                                            .text_color(cx.theme().muted_foreground)
                                    )
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .items_baseline()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child(reply_author)
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child(reply_content)
                                            )
                                    )
                            )
                        } else {
                            this
                        }
                    })
                    // Main message
                    .child(
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
                                    .cursor_pointer()
                                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                        this.show_user_profile(avatar_user.clone(), cx);
                                    }))
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
                                            .cursor_pointer()
                                            .hover(|s| s.underline())
                                            .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                                this.show_user_profile(author_user.clone(), cx);
                                            }))
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
                            })
                            // Reactions (if any)
                            .when(!reactions.is_empty(), |this| {
                                this.child(
                                    h_flex()
                                        .mt_1()
                                        .gap_1()
                                        .flex_wrap()
                                        .children(
                                            reactions.iter().map(|(emoji, users)| {
                                                let count = users.len();
                                                let user_reacted = users.contains(&user_id);
                                                let emoji_clone = emoji.clone();
                                                let msg_id = message_id.clone();
                                                
                                                div()
                                                    .px_2()
                                                    .py_1()
                                                    .rounded(cx.theme().radius)
                                                    .border_1()
                                                    .when(user_reacted, |this| {
                                                        this.bg(cx.theme().accent)
                                                            .border_color(cx.theme().accent_foreground)
                                                    })
                                                    .when(!user_reacted, |this| {
                                                        this.bg(cx.theme().secondary)
                                                            .border_color(cx.theme().border)
                                                    })
                                                    .hover(|s| s.bg(cx.theme().accent).cursor_pointer())
                                                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                                        this.toggle_reaction(msg_id.clone(), emoji_clone.clone(), cx);
                                                    }))
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
                                                                    .font_weight(gpui::FontWeight::MEDIUM)
                                                                    .text_color(if user_reacted {
                                                                        cx.theme().accent_foreground
                                                                    } else {
                                                                        cx.theme().muted_foreground
                                                                    })
                                                                    .child(count.to_string())
                                                            )
                                                    )
                                            })
                                        )
                                )
                            })
                    )
                    )
            )
            // Hover toolbar (Discord-style) - positioned at top-right of message
            .child(
                div()
                    .absolute()
                    .top(px(-8.0))
                    .right(px(16.0))
                    .occlude()
                    .child(
                        div()
                            .flex()
                            .gap_px()
                            .px_1()
                            .py_px()
                            .rounded(cx.theme().radius)
                            .bg(cx.theme().background)
                            .border_1()
                            .border_color(cx.theme().border)
                            .shadow_md()
                            // Reply button
                            .child({
                                let message_clone = msg.clone();
                                Button::new(format!("reply-{}", message_id))
                                    .icon(IconName::ArrowLeft)
                                    .ghost()
                                    .xsmall()
                                    .tooltip("Reply")
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.start_reply(&message_clone, cx);
                                    }))
                            })
                            // Emoji picker
                            .child(
                                gpui_component::popover::Popover::new(format!("emoji-picker-{}", message_id))
                                    .trigger(
                                        Button::new(format!("add-reaction-{}", message_id))
                                            .icon(IconName::Plus)
                                            .ghost()
                                            .xsmall()
                                            .tooltip("Add Reaction")
                                    )
                                    .content({
                                        let msg_id = message_id.clone();
                                        let app_entity = cx.entity();
                                        move |_state, _window, cx| {
                                            Self::render_simple_emoji_picker(msg_id.clone(), app_entity.clone(), cx)
                                        }
                                    })
                            )
                    )
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
        let has_reply = self.replying_to.is_some();
        
        v_flex()
            .flex_shrink_0()
            .gap_2()
            // Reply preview (if replying)
            .when(has_reply, |this| {
                if let Some(reply) = self.replying_to.as_ref() {
                    let reply_author = reply.author.username.clone();
                    let reply_content = reply.content_preview.clone();
                    this.child(
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
                                        gpui_component::Icon::new(IconName::ArrowLeft)
                                            .small()
                                            .text_color(cx.theme().muted_foreground)
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child(format!("Replying to {}", reply_author))
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(cx.theme().foreground)
                                                    .child(reply_content)
                                            )
                                    )
                                    .child(
                                        Button::new("btn-cancel-reply")
                                            .icon(IconName::Close)
                                            .ghost()
                                            .xsmall()
                                            .tooltip("Cancel reply")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.cancel_reply(cx);
                                            })),
                                    )
                            )
                    )
                } else {
                    this
                }
            })
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

    fn render_simple_emoji_picker(
        message_id: String,
        app_entity: gpui::Entity<TripwireApp>,
        cx: &mut Context<gpui_component::popover::PopoverState>,
    ) -> gpui::AnyElement {
        let emojis = vec![
            vec!["😀", "😃", "😄", "😁", "😅", "😂", "🤣", "😊"],
            vec!["😇", "🙂", "🙃", "😉", "😌", "😍", "🥰", "😘"],
            vec!["😗", "😙", "😚", "😋", "😛", "😝", "😜", "🤪"],
            vec!["🤨", "🧐", "🤓", "😎", "🥳", "😏", "😒", "😞"],
            vec!["😔", "😟", "😕", "🙁", "😣", "😖", "😫", "😩"],
            vec!["🥺", "😢", "😭", "😤", "😠", "😡", "🤬", "🤯"],
            vec!["😳", "🥵", "🥶", "😱", "😨", "😰", "😥", "😓"],
            vec!["❤️", "🧡", "💛", "💚", "💙", "💜", "🖤", "🤍"],
            vec!["💔", "❣️", "💕", "💞", "💓", "💗", "💖", "💘"],
            vec!["👍", "👎", "👌", "✌️", "🤞", "🤟", "🤘", "🤙"],
            vec!["👈", "👉", "👆", "👇", "☝️", "✋", "🤚", "🖐️"],
            vec!["👋", "🤝", "🙏", "✍️", "💪", "👏", "🙌", "👐"],
            vec!["🎉", "🎊", "🎈", "🎁", "🏆", "🥇", "🥈", "🥉"],
            vec!["⚡", "✨", "💫", "⭐", "🌟", "💥", "🔥", "🌈"],
            vec!["✅", "❌", "⚠️", "❗", "❓", "💯", "🆒", "🆕"],
        ];

        div()
            .p_2()
            .w(gpui::px(280.0))
            .max_h(gpui::px(320.0))
            .overflow_y_scrollbar()
            .child(
                v_flex()
                    .gap_1()
                    .children(
                        emojis.iter().map(|row| {
                            h_flex()
                                .gap_1()
                                .children(
                                    row.iter().map(|emoji| {
                                        let emoji_str = emoji.to_string();
                                        let msg_id = message_id.clone();
                                        let app = app_entity.clone();
                                        let emoji_for_closure = emoji_str.clone();
                                        
                                        div()
                                            .p_2()
                                            .rounded(cx.theme().radius)
                                            .hover(|s| s.bg(cx.theme().accent).cursor_pointer())
                                            .on_mouse_down(
                                                gpui::MouseButton::Left,
                                                cx.listener(move |_state, _, window, cx| {
                                                    // Toggle the reaction on the app
                                                    let app = app.clone();
                                                    let msg_id = msg_id.clone();
                                                    let emoji_str = emoji_for_closure.clone();
                                                    
                                                    // Defer the update to avoid context lifetime issues
                                                    cx.defer(move |cx| {
                                                        _ = app.update(cx, |app, cx| {
                                                            app.toggle_reaction(msg_id, emoji_str, cx);
                                                        });
                                                    });
                                                    
                                                    // Dismiss the popover
                                                    cx.emit(gpui::DismissEvent);
                                                }),
                                            )
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .child(emoji_str)
                                            )
                                    })
                                )
                        })
                    )
            )
            .into_any_element()
    }
}
