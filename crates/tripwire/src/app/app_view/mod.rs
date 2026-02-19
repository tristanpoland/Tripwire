//! Main application view — the Discord-like layout.
//!
//! Sub-modules each implement one panel of the UI via `impl TripwireApp`.
//! This file wires them all together into the top-level `render_app` method.

pub mod channel_list;
pub mod chat_area;
pub mod dm_list;
pub mod members_panel;
pub mod profile_card;
pub mod server_list;
pub mod settings;
pub mod voice_channel;
pub mod stage_channel;

use gpui::{AnyElement, Context, IntoElement as _, Window, div, InteractiveElement};
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
            // Profile modal overlay (if open)
            .when(self.show_profile.is_some(), |this| {
                let current_user_id = self.auth.current_user.as_ref().map(|u| u.id.clone()).unwrap_or_default();
                this.child(
                    div()
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(gpui::rgba(0x00000099))
                        .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.close_profile(cx);
                        }))
                        .child(
                            div()
                                .occlude()
                                .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
                                .when_some(self.show_profile.clone(), |this: gpui::Div, profile| {
                                    this.child(Self::render_profile_card(&profile, &current_user_id, window, cx))
                                })
                        )
                )
            })
            // Settings modal overlay (if open)
            .when(self.show_settings, |this| {
                this.child(self.render_settings_modal(window, cx))
            })
            .into_any_element()
    }
}
