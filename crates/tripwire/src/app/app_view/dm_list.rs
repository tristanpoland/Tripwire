//! Direct Messages panel — 240 px wide sidebar showing recent DMs.

use gpui::{
    AnyElement, Context, ElementId, IntoElement as _, SharedString, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui::InteractiveElement;
use gpui::StatefulInteractiveElement;
use gpui::ParentElement;
use gpui_component::button::ButtonVariants;
use gpui::Styled;
use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable as _, StyledExt as _,
    avatar::Avatar,
    button::Button,
    h_flex, v_flex,
    scroll::ScrollableElement as _,
    tooltip::Tooltip,
};

use crate::app::TripwireApp;

const PANEL_WIDTH: f32 = 240.;

impl TripwireApp {
    pub(crate) fn render_dm_list(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let active_dm = self.active_dm_id.clone();
        
        // Pre-compute DM elements to avoid FnMut borrow checker issues
        let mut dm_elements: Vec<AnyElement> = Vec::new();
        for dm in &self.dm_channels {
            let dm_id = dm.id.clone();
            let recipient = dm.recipient.clone();
            let is_active = active_dm.as_deref() == Some(dm_id.as_str());
            let has_unread = dm.unread > 0;
            let last_message = dm.last_message.clone();
            let status = recipient.status.clone();
            
            let status_color = match status {
                crate::models::UserStatus::Online => gpui::hsla(142. / 360., 0.71, 0.45, 1.),
                crate::models::UserStatus::Idle => gpui::hsla(43. / 360., 0.85, 0.56, 1.),
                crate::models::UserStatus::DoNotDisturb => gpui::hsla(0. / 360., 0.85, 0.60, 1.),
                crate::models::UserStatus::Offline => gpui::hsla(0., 0., 0.55, 1.),
            };

            dm_elements.push(
                div()
                    .id(ElementId::Name(SharedString::from(dm_id.clone())))
                    .mx_2()
                    .px_2()
                    .py_2()
                    .rounded(cx.theme().radius)
                    .cursor_pointer()
                    .when(is_active, |this| this.bg(cx.theme().sidebar_accent))
                    .hover(|s| s.bg(cx.theme().sidebar_accent))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.switch_dm(dm_id.clone(), window, cx);
                    }))
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            // Avatar with status dot
                            .child(
                                div()
                                    .relative()
                                    .flex_shrink_0()
                                    .child(Avatar::new().name(recipient.username.clone()).small())
                                    // Status dot
                                    .child(
                                        div()
                                            .absolute()
                                            .bottom(px(-2.))
                                            .right(px(-2.))
                                            .w(px(12.))
                                            .h(px(12.))
                                            .rounded_full()
                                            .bg(status_color)
                                            .border_2()
                                            .border_color(cx.theme().sidebar),
                                    ),
                            )
                            // Name + last message
                            .child(
                                v_flex()
                                    .flex_1()
                                    .min_w_0()
                                    .gap_0()
                                    .child(
                                        div()
                                            .text_sm()
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
                                            .overflow_hidden()
                                            .text_ellipsis()
                                            .child(recipient.username),
                                    )
                                    .when_some(last_message, |this, msg| {
                                        this.child(
                                            div()
                                                .text_xs()
                                                .text_color(cx.theme().muted_foreground)
                                                .overflow_hidden()
                                                .text_ellipsis()
                                                .child(msg),
                                        )
                                    }),
                            )
                            // Unread badge
                            .when(has_unread, |this| {
                                this.child(
                                    div()
                                        .min_w(px(18.))
                                        .h(px(18.))
                                        .px(px(5.))
                                        .rounded_full()
                                        .bg(cx.theme().danger)
                                        .flex()
                                        .flex_shrink_0()
                                        .items_center()
                                        .justify_center()
                                        .text_color(gpui::white())
                                        .text_xs()
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .child(dm.unread.to_string()),
                                )
                            }),
                    )
                    .into_any_element(),
            );
        }

        v_flex()
            .w(px(PANEL_WIDTH))
            .h_full()
            .flex_shrink_0()
            .bg(cx.theme().sidebar)
            .border_r_1()
            .border_color(cx.theme().sidebar_border)
            .overflow_hidden()
            // ── Header ────────────────────────────────────────────────────────
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
                            .child("Direct Messages"),
                    )
                    .child(
                        Button::new("btn-new-dm")
                            .icon(IconName::Plus)
                            .ghost()
                            .xsmall()
                            .tooltip("New DM")
                            .on_click(|_, _, _| {
                                // TODO: open new DM dialog
                            }),
                    ),
            )
            // ── DM list (scrollable) ──────────────────────────────────────────
            .child(
                div()
                    .flex_1()
                    .overflow_y_scrollbar()
                    .py_2()
                    .children(dm_elements),
            )
            // ── User bar ──────────────────────────────────────────────────────
            .child(self.render_user_bar(cx))
            .into_any_element()
    }
}
