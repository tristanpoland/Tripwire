//! Channel list panel — 240 px wide sidebar showing categories, channels and
//! the current user's status bar at the bottom.

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
    ActiveTheme as _, Icon, IconName, Sizable as _,
    avatar::Avatar,
    button::Button,
    h_flex, v_flex,
    scroll::ScrollableElement as _,
};

use crate::app::TripwireApp;
use crate::models::ChannelCategory;

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

        // Pre-compute category elements to avoid FnMut borrow checker issues.
        let mut category_elements: Vec<AnyElement> = Vec::new();
        for cat in &server.categories {
            category_elements.push(self.render_category(cat, cx).into_any_element());
        }

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
                            .font_weight(gpui::FontWeight::SEMIBOLD)
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
                    .overflow_y_scrollbar()
                    .py_2()
                    .children(category_elements),
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
        let cat_name = cat.name.clone();
        let is_collapsed = cat.collapsed;

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
                    .hover(|s| s.bg(cx.theme().sidebar_accent))
                    .rounded(cx.theme().radius)
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.toggle_category(&cat_name, cx);
                    }))
                    .child(
                        Icon::new(if is_collapsed {
                            IconName::ChevronRight
                        } else {
                            IconName::ChevronDown
                        })
                        .xsmall()
                        .text_color(cx.theme().sidebar_foreground),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().muted_foreground)
                            .child(cat.name.to_uppercase()),
                    ),
            )
            // Channels in this category (hidden when collapsed)
            .when(!is_collapsed, |this| {
                let mut channel_elements: Vec<AnyElement> = Vec::new();
                
                for channel in &cat.channels {
                    let ch_id = channel.id.clone();
                    let ch_id_for_later = ch_id.clone(); // For voice check
                    let ch_name = channel.name.clone();
                    let is_active = active_id.as_deref() == Some(ch_id.as_str());
                    let has_unread = channel.unread > 0 && !is_active;
                    let kind = channel.kind.clone();
                    let members_connected = channel.members_connected;
                    let unread = channel.unread;

                    // Channel row
                    channel_elements.push(
                        div()
                            .id(ElementId::Name(SharedString::from(format!(
                                "ch-{}",
                                ch_id
                            ))))
                            .mx_2()
                            .px_2()
                            .py(px(6.))
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
                                    .child(
                                        Icon::new(kind.icon())
                                            .xsmall()
                                            .text_color(if is_active || has_unread {
                                                cx.theme().sidebar_foreground
                                            } else {
                                                cx.theme().muted_foreground
                                            }),
                                    )
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
                                    .when(members_connected > 0 && kind.is_voice_based(), |this| {
                                        this.child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .child(
                                                    Icon::new(IconName::User)
                                                        .text_color(cx.theme().muted_foreground)
                                                        .xsmall(),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(cx.theme().muted_foreground)
                                                        .child(members_connected.to_string()),
                                                ),
                                        )
                                    })
                                    .when(has_unread, |this| {
                                        this.child(
                                            div()
                                                .min_w(px(18.))
                                                .h(px(18.))
                                                .px(px(5.))
                                                .rounded_full()
                                                .bg(cx.theme().foreground)
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .text_color(cx.theme().background)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(unread.to_string()),
                                        )
                                    }),
                            )
                            .into_any_element()
                    );
                    
                    // Show connected users indented below voice channels (always show if there are participants)
                    if kind.is_voice_based() && !channel.voice_participants.is_empty() {
                        for participant in &channel.voice_participants {
                            channel_elements.push(
                                div()
                                    .ml(px(40.0))
                                    .mr(px(8.0))
                                    .px_2()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .hover(|s| s.bg(cx.theme().sidebar_accent))
                                    .cursor_pointer()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                div()
                                                    .size(px(8.0))
                                                    .rounded_full()
                                                    .bg(if participant.is_speaking {
                                                        gpui::hsla(0.36, 0.65, 0.50, 1.0) // Green for speaking
                                                    } else {
                                                        cx.theme().muted_foreground
                                                    })
                                            )
                                            .child(
                                                Avatar::new()
                                                    .name(participant.username.clone())
                                                    .xsmall()
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(cx.theme().foreground)
                                                    .child(participant.username.clone())
                                            )
                                            // Show muted/deafened indicators
                                            .when(participant.is_muted, |this| {
                                                this.child(
                                                    Icon::new(IconName::Minus)
                                                        .xsmall()
                                                        .text_color(cx.theme().danger)
                                                )
                                            })
                                            .when(participant.is_deafened, |this| {
                                                this.child(
                                                    Icon::new(IconName::Close)
                                                        .xsmall()
                                                        .text_color(cx.theme().danger)
                                                )
                                            })
                                    )
                                    .into_any_element()
                            );
                        }
                    }
                }
                
                this.children(channel_elements)
            })
     }

    pub(crate) fn render_user_bar(&self, cx: &mut Context<Self>) -> impl gpui::IntoElement {
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
                                    .map(|u| u.username.clone())
                                    .unwrap_or_else(|| "?".to_string()),
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
                            .font_weight(gpui::FontWeight::SEMIBOLD)
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
                    .tooltip("User Settings")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.open_settings(cx);
                    })),
            )
    }
}
