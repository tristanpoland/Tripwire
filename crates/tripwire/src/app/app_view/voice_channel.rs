//! Voice channel UI - shows participants in voice call with controls

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
pub struct VoiceParticipant {
    pub user_id: String,
    pub username: String,
    pub is_speaking: bool,
    pub is_muted: bool,
    pub is_deafened: bool,
    pub is_video: bool,
}

impl TripwireApp {
    pub(crate) fn render_voice_channel_ui(
        &self,
        channel_name: &str,
        members_connected: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // Mock participants data
        let participants = vec![
            VoiceParticipant {
                user_id: "1".to_string(),
                username: "Alice".to_string(),
                is_speaking: true,
                is_muted: false,
                is_deafened: false,
                is_video: false,
            },
            VoiceParticipant {
                user_id: "2".to_string(),
                username: "Bob".to_string(),
                is_speaking: false,
                is_muted: true,
                is_deafened: false,
                is_video: false,
            },
            VoiceParticipant {
                user_id: "3".to_string(),
                username: "Charlie".to_string(),
                is_speaking: false,
                is_muted: false,
                is_deafened: true,
                is_video: true,
            },
        ];

        v_flex()
            .flex_1()
            .h_full()
            .gap_4()
            .p_6()
            .child(
                // Voice channel title and info
                v_flex()
                    .gap_2()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_2xl()
                                    .child("🔊")
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
                            .child(format!("{} members connected", members_connected.max(participants.len())))
                    )
            )
            .child(
                // Participants grid
                div()
                    .flex_1()
                    .overflow_y_scrollbar()
                    .child(
                        div()
                            .grid()
                            .grid_cols(3)
                            .gap_4()
                            .children(participants.into_iter().map(|p| {
                                self.render_voice_participant(p, cx)
                            }))
                    )
            )
            .child(
                // Voice controls bar
                self.render_voice_controls(cx)
            )
            .into_any_element()
    }

    fn render_voice_participant(
        &self,
        participant: VoiceParticipant,
        cx: &Context<Self>,
    ) -> AnyElement {
        let speaking_ring = if participant.is_speaking {
            Some(gpui::rgb(0x23a55a)) // Green for speaking
        } else {
            None
        };

        div()
            .w_full() // Full width of grid cell
            .pb(px(100.0)) // Padding bottom as percentage creates aspect ratio
            .relative()
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .p_4()
                    .rounded(cx.theme().radius_lg)
                    .bg(cx.theme().muted)
                    .border_2()
                    .when_some(speaking_ring, |this, color| {
                        this.border_color(color)
                    })
                    .when(speaking_ring.is_none(), |this| {
                        this.border_color(cx.theme().border)
                    })
                    .child(
                        v_flex()
                            .gap_3()
                            .items_center()
                            .justify_center()
                            .h_full()
                            .child(
                                // Avatar with status overlay
                                div()
                                    .relative()
                                    .flex_shrink_0()
                                    .child(
                                        Avatar::new()
                                            .name(participant.username.clone())
                                            .with_size(gpui_component::Size::Large)
                                    )
                                    .child(
                                        // Status indicators overlay
                                        div()
                                            .absolute()
                                            .bottom_0()
                                            .right_0()
                                            .child(
                                                h_flex()
                                                    .gap_1()
                                                    .when(participant.is_muted, |this| {
                                                        this.child(
                                                            div()
                                                                .w(px(24.0))
                                                                .h(px(24.0))
                                                                .rounded_full()
                                                                .bg(gpui::rgb(0xed4245))
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .child("🔇")
                                                                )
                                                        )
                                                    })
                                                    .when(participant.is_deafened, |this| {
                                                        this.child(
                                                            div()
                                                                .w(px(24.0))
                                                                .h(px(24.0))
                                                                .rounded_full()
                                                                .bg(gpui::rgb(0x5865f2))
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .child("🔇")
                                                                )
                                                        )
                                                    })
                                                    .when(participant.is_video, |this| {
                                                        this.child(
                                                            div()
                                                                .w(px(24.0))
                                                                .h(px(24.0))
                                                                .rounded_full()
                                                                .bg(gpui::rgb(0x23a55a))
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .child("📹")
                                                                )
                                                        )
                                                    })
                                            )
                                    )
                            )
                            .child(
                                // Username
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(cx.theme().foreground)
                                    .child(participant.username)
                            )
                    )
            )
            .into_any_element()
    }

    fn render_voice_controls(&self, cx: &Context<Self>) -> AnyElement {
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
                        Button::new("btn-mute")
                            .icon(IconName::User)
                            .tooltip("Mute")
                            .ghost()
                    )
                    .child(
                        Button::new("btn-deafen")
                            .icon(IconName::Minus)
                            .tooltip("Deafen")
                            .ghost()
                    )
                    .child(
                        Button::new("btn-settings")
                            .icon(IconName::Settings)
                            .tooltip("Voice Settings")
                            .ghost()
                    )
            )
            .child(
                Button::new("btn-leave-voice")
                    .label("Leave Voice")
                    .icon(IconName::Close)
                    .danger()
            )
            .into_any_element()
    }
}
