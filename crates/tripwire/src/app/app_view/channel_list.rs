//! Channel list panel — 240 px wide sidebar showing categories, channels and
//! the current user's status bar at the bottom.

use gpui::{
    AnyElement, Context, ElementId, IntoElement as _, SharedString, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_component::StyledExt;
use gpui::InteractiveElement;
use gpui::StatefulInteractiveElement;
use gpui::ParentElement;
use gpui_component::button::ButtonVariants;
use gpui::Styled;
use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable as _,
    avatar::Avatar,
    button::Button,
    h_flex, v_flex,
    tooltip::Tooltip,
};

use crate::app::TripwireApp;
use crate::models::{ChannelCategory, ChannelKind};

const PANEL_WIDTH: f32 = 240.;

impl TripwireApp {
    pub(crate) fn render_channel_list(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let server = match self.active_server() {
            Some(s) => s.clone(),
            None => {
                return div()
                    .w(px(PANEL_WIDTH))
                    .h_full()
                    .bg(cx.theme().sidebar)
                    .into_any_element()
            }
        };

        let current_user = self.auth.current_user.clone();

        v_flex()
            .w(px(PANEL_WIDTH))
            .h_full()
            .flex_shrink_0()
            .bg(cx.theme().sidebar)
            .border_r_1()
            .border_color(cx.theme().sidebar_border)
            .overflow_hidden()
            // ── Server header ────────────────────────────────────────────────
            .child(
                h_flex()
                    .h(px(48.))
                    .px_4()
                    .flex_shrink_0()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().sidebar_border)
                    .child(
                        div()
                            .text_sm()
                            .font_semibold()
                            .text_color(cx.theme().sidebar_foreground)
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(server.name.clone()),
                    )
                    .child(
                        Button::new("btn-server-menu")
                            .icon(IconName::EllipsisVertical)
                            .ghost()
                            .xsmall()
                            .on_click(|_, _, _| {
                                // TODO: server settings dropdown
                            }),
                    ),
            )
            // ── Categories + channels (scrollable) ───────────────────────────
            .child(
                div()
                    .flex_1()
                    .overflow_y_scroll()
                    .py_2()
                    .children(server.categories.iter().map(|cat| {
                        self.render_category(cat, cx)
                    })),
            )
            // ── User bar ─────────────────────────────────────────────────────
            .child(self.render_user_bar(cx))
            .into_any_element()
    }

    fn render_category(
        &mut self,
        cat: &ChannelCategory,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let active_id = self.active_channel_id.clone();

        v_flex()
            .w_full()
            .mb_2()
            // Category header
            .child(
                h_flex()
                    .id(ElementId::Name(SharedString::from(format!(
                        "cat-{}",
                        cat.name
                    ))))
                    .px_2()
                    .py_1()
                    .items_center()
                    .gap_1()
                    .cursor_pointer()
                    .child(
                        Icon::new(IconName::ChevronDown)
                            .xsmall()
                            .text_color(cx.theme().sidebar_foreground),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_semibold()
                            .text_color(cx.theme().muted_foreground)
                            .child(cat.name.to_uppercase()),
                    ),
            )
            // Channels in this category
            .children(cat.channels.iter().map(|channel| {
                let ch_id = channel.id.clone();
                let ch_name = channel.name.clone();
                let is_active = active_id.as_deref() == Some(ch_id.as_str());
                let has_unread = channel.unread > 0 && !is_active;
                let kind = channel.kind.clone();

                div()
                    .id(ElementId::Name(SharedString::from(format!(
                        "ch-{}",
                        ch_id
                    ))))
                    .mx_2()
                    .px_2()
                    .py_1()
                    .rounded(cx.theme().radius)
                    .cursor_pointer()
                    .when(is_active, |this| this.bg(cx.theme().sidebar_accent))
                    .hover(|s| s.bg(cx.theme().sidebar_accent))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.switch_channel(ch_id.clone(), window, cx);
                    }))
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            // Channel kind icon
                            .child(match kind {
                                ChannelKind::Announcement => div()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("!")
                                    .into_any_element(),
                                ChannelKind::Voice => div()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("♪")
                                    .into_any_element(),
                                ChannelKind::Text => div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("#")
                                    .into_any_element(),
                            })
                            // Channel name
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .font_weight(if has_unread {
                                        gpui::FontWeight::SEMIBOLD
                                    } else {
                                        gpui::FontWeight::NORMAL
                                    })
                                    .text_color(if is_active || has_unread {
                                        cx.theme().sidebar_foreground
                                    } else {
                                        cx.theme().muted_foreground
                                    })
                                    .child(ch_name),
                            )
                            // Unread badge
                            .when(has_unread, |this| {
                                this.child(
                                    div()
                                        .min_w(px(16.))
                                        .h(px(16.))
                                        .px(px(4.))
                                        .rounded_full()
                                        .bg(cx.theme().destructive)
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_color(gpui::white())
                                        .text_xs()
                                        .child(channel.unread.to_string()),
                                )
                            }),
                    )
            }))
    }

    fn render_user_bar(&self, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let user = self.auth.current_user.clone();

        h_flex()
            .h(px(52.))
            .flex_shrink_0()
            .px_2()
            .gap_2()
            .items_center()
            .border_t_1()
            .border_color(cx.theme().sidebar_border)
            .bg(cx.theme().sidebar)
            // Avatar + status
            .child(
                div()
                    .relative()
                    .child(
                        Avatar::new()
                            .name(
                                user.as_ref()
                                    .map(|u| u.username.as_str())
                                    .unwrap_or("?"),
                            )
                            .xsmall(),
                    )
                    // Green online dot
                    .child(
                        div()
                            .absolute()
                            .bottom_0()
                            .right_0()
                            .w(px(10.))
                            .h(px(10.))
                            .rounded_full()
                            .bg(gpui::hsla(142. / 360., 0.71, 0.45, 1.))
                            .border_2()
                            .border_color(cx.theme().sidebar),
                    ),
            )
            // Username + discriminator
            .child(
                v_flex()
                    .flex_1()
                    .gap_0()
                    .overflow_hidden()
                    .child(
                        div()
                            .text_sm()
                            .font_semibold()
                            .text_color(cx.theme().sidebar_foreground)
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(
                                user.as_ref()
                                    .map(|u| u.username.clone())
                                    .unwrap_or_default(),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child("Online"),
                    ),
            )
            // Settings icon
            .child(
                Button::new("btn-user-settings")
                    .icon(IconName::Settings)
                    .ghost()
                    .xsmall()
                    .tooltip(|window, cx| Tooltip::text("User Settings", window, cx))
                    .on_click(|_, _, _| {
                        // TODO: open user settings
                    }),
            )
    }
}
