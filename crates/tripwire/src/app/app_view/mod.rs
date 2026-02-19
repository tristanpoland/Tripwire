//! Main application view — the Discord-like layout.
//!
//! Sub-modules each implement one panel of the UI via `impl TripwireApp`.
//! This file wires them all together into the top-level `render_app` method.

pub mod channel_list;
pub mod chat_area;
pub mod dm_list;
pub mod members_panel;
pub mod server_list;

use gpui::{AnyElement, Context, IntoElement as _, Window};
use gpui::prelude::FluentBuilder;
use gpui::ParentElement;
use gpui::Styled;
use gpui_component::{ActiveTheme as _, h_flex};
use crate::app::{AppView, TripwireApp};

impl TripwireApp {
    /// Top-level Discord-style layout:
    ///
    /// ```
    /// ┌──────┬────────────┬───────────────────────────┬────────────────┐
    /// │      │            │  Channel Header           │                │
    /// │  S   │  Channel   │───────────────────────────│  Members List  │
    /// │  e   │  List      │  Messages (scrollable)    │                │
    /// │  r   │   OR       │                           │                │
    /// │  v   │  DM List   │───────────────────────────│                │
    /// │  e   │            │  Message Input            │                │
    /// │  r   ├────────────┴───────────────────────────┴────────────────┤
    /// │  s   │                 User Bar                                │
    /// └──────┴─────────────────────────────────────────────────────────┘
    /// ```
    pub(crate) fn render_app(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        h_flex()
            .size_full()
            .overflow_hidden()
            .bg(cx.theme().background)
            // Left strip: server icon list
            .child(self.render_server_list(cx))
            // Channel/DM panel based on current view
            .child(match self.current_view {
                AppView::Servers => self.render_channel_list(window, cx),
                AppView::DirectMessages => self.render_dm_list(window, cx),
            })
            // Main content: header + messages + input
            .child(self.render_chat_area(window, cx))
            // Right panel: members list (only show for servers, not DMs)
            .when(self.show_members && self.current_view == AppView::Servers, |this| {
                this.child(self.render_members_panel(cx))
            })
            .into_any_element()
    }
}
