//! Stage channel UI - shows speakers on stage and audience members

use gpui::{
    div, prelude::FluentBuilder as _, px, AnyElement, Context, IntoElement,
    InteractiveElement, ParentElement, Styled, Window,
};
use gpui_component::{
    h_flex, v_flex, ActiveTheme as _, IconName, Sizable as _,
    avatar::Avatar,
    button::{Button, ButtonVariants},
    scroll::ScrollableElement as _,
    StyledExt,
};

use crate::app::TripwireApp;

#[derive(Debug, Clone)]
pub struct StageParticipant {
    pub user_id: String,
    pub username: String,
    pub is_speaking: bool,
    pub is_muted: bool,
    pub role: StageRole,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StageRole {
    Moderator,
    Speaker,
    Audience,
}

impl TripwireApp {
    pub(crate) fn render_stage_channel_ui(
        &self,
        channel_name: &str,
        members_connected: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // Mock stage participants
        let participants = vec![
            StageParticipant {
                user_id: "1".to_string(),
                username: "Host Alice".to_string(),
                is_speaking: true,
                is_muted: false,
                role: StageRole::Moderator,
            },
            StageParticipant {
                user_id: "2".to_string(),
                username: "Speaker Bob".to_string(),
                is_speaking: false,
                is_muted: false,
                role: StageRole::Speaker,
            },
            StageParticipant {
                user_id: "3".to_string(),
                username: "Speaker Carol".to_string(),
                is_speaking: true,
                is_muted: false,
                role: StageRole::Speaker,
            },
        ];

        let audience_count = members_connected.saturating_sub(participants.len());

        v_flex()
            .flex_1()
            .h_full()
            .gap_4()
            .p_6()
            .child(
                // Stage channel title and info
                v_flex()
                    .gap_2()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_2xl()
                                    .child("🎙️")
                            )
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(cx.theme().foreground)
                                    .child(channel_name.to_string())
                            )
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!(
                                "{} on stage • {} in audience",
                                participants.len(),
                                audience_count
                            ))
                    )
            )
            .child(
                // Stage area
                v_flex()
                    .flex_1()
                    .gap_4()
                    .overflow_y_scrollbar()
                    .child(
                        // Speakers section
                        v_flex()
                            .gap_3()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(cx.theme().foreground)
                                    .child("On Stage")
                            )
                            .child(
                                div()
                                    .p_6()
                                    .rounded(cx.theme().radius_lg)
                                    .bg(cx.theme().sidebar)
                                    .border_2()
                                    .border_color(gpui::rgb(0x5865f2))
                                    .child(
                                        div()
                                            .grid()
                                            .grid_cols(4)
                                            .gap_4()
                                            .children(participants.into_iter().map(|p| {
                                                self.render_stage_participant(p, cx)
                                            }))
                                    )
                            )
                    )
                    .child(
                        // Audience section
                        v_flex()
                            .gap_3()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(cx.theme().foreground)
                                    .child(format!("Audience ({})", audience_count))
                            )
                            .child(
                                div()
                                    .p_4()
                                    .rounded(cx.theme().radius_lg)
                                    .bg(cx.theme().muted)
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(cx.theme().muted_foreground)
                                            .text_center()
                                            .child("Audience members are listening")
                                    )
                            )
                    )
            )
            .child(
                // Stage controls
                self.render_stage_controls(cx)
            )
            .into_any_element()
    }

    fn render_stage_participant(
        &self,
        participant: StageParticipant,
        cx: &Context<Self>,
    ) -> AnyElement {
        let (badge_color, badge_text) = match participant.role {
            StageRole::Moderator => (gpui::rgb(0xf0b232), "MOD"),
            StageRole::Speaker => (gpui::rgb(0x5865f2), "SPEAKER"),
            StageRole::Audience => (gpui::rgb(0x80848e), ""),
        };

        let speaking_ring = if participant.is_speaking {
            Some(gpui::rgb(0x23a55a))
        } else {
            None
        };

        // 16:9 aspect ratio card
        v_flex()
            .w_full()
            .gap_3()
            .p_4()
            .rounded(cx.theme().radius_lg)
            .bg(cx.theme().background)
            .border_2()
            .when_some(speaking_ring, |this, color| {
                this.border_color(color)
            })
            .when(speaking_ring.is_none(), |this| {
                this.border_color(cx.theme().border)
            })
            .child(
                // Avatar with badges
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .relative()
                            .child(
                                Avatar::new()
                                    .name(participant.username.clone())
                                    .with_size(gpui_component::Size::Large)
                            )
                            .when(participant.role != StageRole::Audience, |this| {
                                this.child(
                                    div()
                                        .absolute()
                                        .top(px(-8.0))
                                        .right(px(-8.0))
                                        .px_2()
                                        .py_1()
                                        .rounded(px(4.0))
                                        .bg(badge_color)
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::BOLD)
                                                .text_color(gpui::rgb(0xFFFFFF))
                                                .child(badge_text)
                                        )
                                )
                            })
                            .when(participant.is_muted, |this| {
                                this.child(
                                    div()
                                        .absolute()
                                        .bottom(px(-8.0))
                                        .right(px(-8.0))
                                        .size(px(24.0))
                                        .rounded_full()
                                        .bg(gpui::rgb(0xed4245))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(div().text_xs().child("🔇"))
                                )
                            })
                    )
            )
            .child(
                // Username
                div()
                    .text_sm()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(cx.theme().foreground)
                    .text_center()
                    .child(participant.username)
            )
            .into_any_element()
    }

    fn render_stage_controls(&self, cx: &Context<Self>) -> AnyElement {
        h_flex()
            .w_full()
            .p_4()
            .gap_3()
            .rounded(cx.theme().radius_lg)
            .bg(cx.theme().sidebar)
            .border_1()
            .border_color(cx.theme().border)
            .child(
                h_flex()
                    .flex_1()
                    .gap_2()
                    .child(
                        Button::new("btn-request-speak")
                            .icon(IconName::User)
                            .label("Request to Speak")
                            .primary()
                    )
                    .child(
                        Button::new("btn-mute")
                            .icon(IconName::User)
                            .tooltip("Mute")
                            .ghost()
                    )
            )
            .child(
                Button::new("btn-leave-stage")
                    .label("Leave Stage")
                    .icon(IconName::ArrowLeft)
                    .danger()
            )
            .into_any_element()
    }
}
