//! Forum channel view - Thread-based discussions

use gpui::{
    AnyElement, Context, ElementId, InteractiveElement, ParentElement as _, SharedString, Styled,
    Window, div, px, IntoElement,
};
use gpui_component::{
    h_flex, v_flex, ActiveTheme as _, Icon, IconName, Sizable as _,
    avatar::Avatar,
    button::{Button, ButtonVariants as _},
    scroll::ScrollableElement,
};

use crate::app::TripwireApp;
use crate::models::Message;

#[derive(Clone, Debug)]
pub struct ForumThread {
    pub id: String,
    pub title: String,
    pub author: String,
    pub reply_count: usize,
    pub last_reply_time: String,
    pub tags: Vec<String>,
    pub is_pinned: bool,
    pub is_locked: bool,
}

impl TripwireApp {
    pub(crate) fn render_forum_channel_ui(
        &mut self,
        _channel_name: &str,
        messages: &[Message],
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let threads = self.group_messages_into_threads(messages);

        v_flex()
            .flex_1()
            .overflow_hidden()
            .child(
                h_flex()
                    .w_full()
                    .h(px(56.0))
                    .px_4()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().background)
                    .flex_shrink_0()
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("{} threads", threads.len()))
                    )
                    .child(
                        Button::new("create-thread")
                            .label("New Thread")
                            .icon(IconName::Plus)
                            .primary()
                            .small()
                    )
            )
            .child(
                div()
                    .flex_1()
                    .overflow_y_scrollbar()
                    .children(threads.iter().map(|thread| {
                        self.render_forum_thread(thread, cx)
                    }))
            )
            .into_any_element()
    }

    fn group_messages_into_threads(&self, messages: &[Message]) -> Vec<ForumThread> {
        let mut threads = Vec::new();
        
        for (idx, msg) in messages.iter().enumerate() {
            if idx % 3 == 0 {
                threads.push(ForumThread {
                    id: msg.id.clone(),
                    title: msg.content.lines().next()
                        .unwrap_or("Untitled")
                        .chars()
                        .take(80)
                        .collect(),
                    author: msg.author.username.clone(),
                    reply_count: (idx % 15) + 1,
                    last_reply_time: "2 hours ago".to_string(),
                    tags: vec!["discussion".to_string()],
                    is_pinned: idx == 0,
                    is_locked: false,
                });
            }
        }
        
        threads
    }

    fn render_forum_thread(&self, thread: &ForumThread, cx: &Context<Self>) -> AnyElement {
        let thread_id = thread.id.clone();
        div()
            .id(ElementId::Name(SharedString::from(thread.id.clone())))
            .w_full()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(cx.theme().border)
            .cursor_pointer()
            .hover(|s| s.bg(cx.theme().muted))
            .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, _, cx| {
                this.open_forum_thread(thread_id.clone(), cx);
            }))
            .child(
                h_flex()
                    .w_full()
                    .gap_3()
                    .items_start()
                    .child(
                        Avatar::new()
                            .name(thread.author.clone())
                            .small()
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .gap_1()
                            .child({
                                let mut header = h_flex()
                                    .gap_2()
                                    .items_center();
                                
                                if thread.is_pinned {
                                    header = header.child(
                                        Icon::new(IconName::Star)
                                            .xsmall()
                                            .text_color(cx.theme().warning)
                                    );
                                }
                                
                                header.child(
                                    div()
                                        .text_sm()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .text_color(cx.theme().foreground)
                                        .child(thread.title.clone())
                                )
                            })
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(format!("by {}", thread.author))
                                    )
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                Icon::new(IconName::User)
                                    .xsmall()
                                    .text_color(cx.theme().muted_foreground)
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(cx.theme().foreground)
                                    .child(thread.reply_count.to_string())
                            )
                    )
            )
            .into_any_element()
    }
    
    pub(crate) fn open_forum_thread(&mut self, thread_id: String, cx: &mut Context<Self>) {
        // Reuse the thread sidebar system for forum threads
        self.open_thread(thread_id, cx);
    }
}