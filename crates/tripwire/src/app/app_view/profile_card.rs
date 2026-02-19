use gpui::{
    div, prelude::FluentBuilder as _, px, AnyElement, Context, IntoElement,
    ParentElement, Styled, Window, InteractiveElement,
};
use gpui_component::{
    h_flex, v_flex, ActiveTheme as _, IconName, Sizable as _,
    avatar::Avatar,
    button::{Button, ButtonVariants},
};

use crate::app::TripwireApp;
use crate::models::UserProfile;

impl TripwireApp {
    pub(crate) fn render_profile_card(
        profile: &UserProfile,
        current_user_id: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let username = profile.user.username.clone();
        let tag = profile.user.tag();
        let status = profile.user.status.clone();
        let status_color = status.color_hex();
        let status_label = status.label();
        
        let custom_status = profile.custom_status.clone();
        let custom_status_emoji = profile.custom_status_emoji.clone();
        let bio = profile.bio.clone();
        let member_since = profile.member_since.clone();
        let roles = profile.roles.clone();
        let badges = profile.badges.clone();
        let accent_color = profile.accent_color.clone().unwrap_or_else(|| "#5865F2".to_string());
        
        let profile_user_id = profile.user.id.clone();
        let is_self = current_user_id == profile_user_id;

        div()
            .w(px(340.0))
            .bg(cx.theme().background)
            .rounded(cx.theme().radius_lg)
            .border_1()
            .border_color(cx.theme().border)
            .overflow_hidden()
            .shadow_lg()
            .child(
                v_flex()
                    .gap_0()
                    // Banner
                    .child(
                        div()
                            .h(px(80.0))
                            .w_full()
                            .bg(gpui::rgb(
                                u32::from_str_radix(&accent_color[1..], 16).unwrap_or(0x5865F2)
                            ))
                    )
                    // Profile content
                    .child(
                        v_flex()
                            .px_4()
                            .pb_4()
                            .gap_3()
                            // Avatar (overlapping banner)
                            .child(
                                div()
                                    .mt(px(-40.0))
                                    .relative()
                                    .child(
                                        div()
                                            .p_1()
                                            .rounded_full()
                                            .bg(cx.theme().background)
                                            .child(
                                                Avatar::new()
                                                    .name(username.clone())
                                                    .with_size(gpui_component::Size::Large)
                                            )
                                            .child(
                                                // Status indicator
                                                div()
                                                    .absolute()
                                                    .bottom(px(4.0))
                                                    .right(px(4.0))
                                                    .w(px(20.0))
                                                    .h(px(20.0))
                                                    .rounded_full()
                                                    .border_4()
                                                    .border_color(cx.theme().background)
                                                    .bg(gpui::rgb(
                                                        u32::from_str_radix(&status_color[1..], 16).unwrap_or(0x80848e)
                                                    ))
                                            )
                                    )
                            )
                            // User info
                            .child(
                                div()
                                    .px_2()
                                    .py_2()
                                    .rounded(cx.theme().radius)
                                    .bg(cx.theme().muted)
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(
                                                h_flex()
                                                    .gap_2()
                                                    .items_center()
                                                    .child(
                                                        div()
                                                            .text_lg()
                                                            .font_weight(gpui::FontWeight::BOLD)
                                                            .text_color(cx.theme().foreground)
                                                            .child(username)
                                                    )
                                                    .when(!badges.is_empty(), |this| {
                                                        this.child(
                                                            h_flex()
                                                                .gap_1()
                                                                .children(
                                                                    badges.iter().take(3).map(|badge| {
                                                                        gpui_component::Icon::new(badge.icon())
                                                                            .xsmall()
                                                                            .text_color(gpui::rgb(
                                                                                u32::from_str_radix(&badge.color()[1..], 16)
                                                                                    .unwrap_or(0x5865F2)
                                                                            ))
                                                                            .into_any_element()
                                                                    })
                                                                )
                                                        )
                                                    })
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child(tag)
                                            )
                                    )
                            )
                            // Divider
                            .child(div().h(px(1.0)).w_full().bg(cx.theme().border))
                            // Custom status (if set)
                            .when(custom_status.is_some(), |this| {
                                this.child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .text_color(cx.theme().muted_foreground)
                                                .child("CUSTOM STATUS")
                                        )
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .items_center()
                                                .when_some(custom_status_emoji, |this, emoji| {
                                                    this.child(
                                                        div()
                                                            .text_base()
                                                            .child(emoji)
                                                    )
                                                })
                                                .when_some(custom_status.clone(), |this, status| {
                                                    this.child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(cx.theme().foreground)
                                                            .child(status)
                                                    )
                                                })
                                        )
                                )
                            })
                            // Bio (if set)
                            .when(bio.is_some(), |this| {
                                this.child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .text_color(cx.theme().muted_foreground)
                                                .child("ABOUT ME")
                                        )
                                        .when_some(bio, |this, bio_text| {
                                            this.child(
                                                div()
                                                    .text_sm()
                                                    .text_color(cx.theme().foreground)
                                                    .child(bio_text)
                                            )
                                        })
                                )
                            })
                            // Roles (if any)
                            .when(!roles.is_empty(), |this| {
                                this.child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .text_color(cx.theme().muted_foreground)
                                                .child("ROLES")
                                        )
                                        .child(
                                            h_flex()
                                                .gap_1()
                                                .flex_wrap()
                                                .children(
                                                    roles.iter().map(|role| {
                                                        div()
                                                            .px_2()
                                                            .py_1()
                                                            .rounded(cx.theme().radius)
                                                            .bg(cx.theme().secondary)
                                                            .border_1()
                                                            .border_color(gpui::rgb(
                                                                u32::from_str_radix(&role.color[1..], 16)
                                                                    .unwrap_or(0x5865F2)
                                                            ))
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_weight(gpui::FontWeight::MEDIUM)
                                                                    .text_color(gpui::rgb(
                                                                        u32::from_str_radix(&role.color[1..], 16)
                                                                            .unwrap_or(0x5865F2)
                                                                    ))
                                                                    .child(role.name.clone())
                                                            )
                                                            .into_any_element()
                                                    })
                                                )
                                        )
                                )
                            })
                            // Member since
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_color(cx.theme().muted_foreground)
                                            .child("MEMBER SINCE")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(cx.theme().foreground)
                                            .child(member_since)
                                    )
                            )
                            // Action buttons (if not viewing own profile)
                            .when(!is_self, |this| {
                                this.child(
                                    h_flex()
                                        .gap_2()
                                        .mt_2()
                                        .child(
                                            Button::new("btn-send-message-profile")
                                                .label("Send Message")
                                                .primary()
                                                .small()
                                                .on_click(|_, _, _| {
                                                    // TODO: Open DM channel
                                                })
                                        )
                                        .child(
                                            Button::new("btn-add-note-profile")
                                                .icon(IconName::Plus)
                                                .ghost()
                                                .small()
                                                .tooltip("Add Note")
                                                .on_click(|_, _, _| {
                                                    // TODO: Add note functionality
                                                })
                                        )
                                )
                            })
                    )
            )
            .into_any_element()
    }
}
