//! Left-most server icon strip — 72 px wide vertical column.
//!
//! Shows:
//! - A "home" icon at the top (DMs / discovery)
//! - A thin separator
//! - One icon per joined server, with unread badge and tooltip
//! - A "+" button to add a server at the bottom
//! - A settings button

use gpui::{
    AnyElement, Context, ElementId, IntoElement as _, SharedString, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui::InteractiveElement;
use gpui::StatefulInteractiveElement;
use gpui::ParentElement;
use gpui_component::StyledExt;
use gpui::Styled;
use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable as _, StyleSized as _,
    avatar::Avatar,
    button::Button,
    tooltip::Tooltip,
    v_flex,
};
use gpui_component::button::ButtonVariants;

use crate::app::TripwireApp;

// Width of the server strip in pixels.
const STRIP_WIDTH: f32 = 72.;

impl TripwireApp {
    /// Renders the leftmost server icon column.
    pub(crate) fn render_server_list(&mut self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .w(px(STRIP_WIDTH))
            .h_full()
            .flex_shrink_0()
            .bg(cx.theme().sidebar)
            .border_r_1()
            .border_color(cx.theme().sidebar_border)
            .items_center()
            .pt_3()
            .pb_3()
            .gap_2()
            .overflow_hidden()
            // ── Home / DM button ────────────────────────────────────────────
            .child(self.server_home_button(cx))
            // ── Separator ───────────────────────────────────────────────────
            .child(
                div()
                    .w(px(32.))
                    .h(px(2.))
                    .rounded_full()
                    .bg(cx.theme().border),
            )
            // ── Server icons ────────────────────────────────────────────────
            .children(
                self.servers
                    .iter()
                    .enumerate()
                    .map(|(ix, server)| {
                        let is_active = self.active_server == ix;
                        let name = server.name.clone();
                        let initials = server.initials();
                        let unread = server.notification_count
                            + server
                                .all_channels()
                                .iter()
                                .map(|c| c.unread)
                                .sum::<usize>();

                        div()
                            .id(ElementId::Name(SharedString::from(format!("server-{ix}"))))
                            .group("")
                            .relative()
                            .flex()
                            .items_center()
                            .w_full()
                            .px_2()
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.switch_server(ix, window, cx);
                            }))
                            // Active indicator bar on left edge
                            .child(
                                div()
                                    .absolute()
                                    .left_0()
                                    .w(px(4.))
                                    .rounded_r(px(4.))
                                    .bg(cx.theme().primary)
                                    .when(is_active, |s| s.h(px(40.)))
                                    .when(!is_active, |s| s.h(px(0.)))
                                    .transition_all(),
                            )
                            // Server avatar
                            .child(
                                div()
                                    .relative()
                                    .tooltip(move |window, cx| {
                                        Tooltip::text(name.clone(), window, cx)
                                    })
                                    .child(
                                        Avatar::new()
                                            .name(initials.as_str())
                                            .with_size(gpui_component::Size::Large)
                                            .when(is_active, |a| {
                                                a.rounded(cx.theme().radius)
                                            })
                                            .when(!is_active, |a| a.rounded_full()),
                                    )
                                    // Unread badge
                                    .when(unread > 0 && !is_active, |this| {
                                        this.child(
                                            div()
                                                .absolute()
                                                .bottom_0()
                                                .right_0()
                                                .w(px(16.))
                                                .h(px(16.))
                                                .rounded_full()
                                                .bg(cx.theme().destructive)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .text_color(gpui::white())
                                                .text_xs()
                                                .child(if unread > 9 {
                                                    "9+".to_string()
                                                } else {
                                                    unread.to_string()
                                                }),
                                        )
                                    }),
                            )
                    }),
            )
            // ── Spacer ──────────────────────────────────────────────────────
            .child(div().flex_1())
            // ── Add server ──────────────────────────────────────────────────
            .child(
                Button::new("btn-add-server")
                    .icon(IconName::Plus)
                    .ghost()
                    .xsmall()
                    .tooltip(|window, cx| Tooltip::text("Add a Server", window, cx))
                    .on_click(|_, _, _| {
                        // TODO: open add server dialog
                    }),
            )
            // ── Settings ────────────────────────────────────────────────────
            .child(
                Button::new("btn-settings")
                    .icon(IconName::Settings)
                    .ghost()
                    .xsmall()
                    .tooltip(|window, cx| Tooltip::text("User Settings", window, cx))
                    .on_click(|_, _, _| {
                        // TODO: open settings sheet
                    }),
            )
            .into_any_element()
    }

    fn server_home_button(&self, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Button::new("btn-home")
            .icon(IconName::Inbox)
            .ghost()
            .small()
            .tooltip(|window, cx| Tooltip::text("Direct Messages", window, cx))
            .on_click(|_, _, _| {
                // TODO: open DMs view
            })
    }
}
