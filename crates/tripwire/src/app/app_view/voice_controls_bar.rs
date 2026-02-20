//! Persistent voice controls bar - shows in lower-left when connected to voice

use gpui::{
    div, prelude::FluentBuilder as _, px, AnyElement, Context, IntoElement,
    InteractiveElement, ParentElement, Styled,
};
use gpui_component::{
    h_flex, v_flex, ActiveTheme as _, IconName, Sizable as _, StyledExt,
    avatar::Avatar,
    button::{Button, ButtonVariants},
};

use crate::app::TripwireApp;

impl TripwireApp {
    pub(crate) fn render_voice_controls_bar(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let voice = self.voice_state.as_ref()?;
        
        // Don't show if we're not connected yet
        if !voice.is_connected() {
            return None;
        }
        
        let channel_name = voice.channel_name.clone();
        let server_name = voice.server_name.clone();
        let is_muted = voice.is_muted;
        let is_deafened = voice.is_deafened;
        let is_video = voice.is_video_enabled;
        
        Some(
            div()
                .absolute()
                .bottom(px(52.0)) // Position directly above 52px user bar
                .left(px(60.0)) // Server list width (60px)
                .w(px(240.0)) // Sidebar width
                .p_2()
                .child(
                    v_flex()
                        .w_full()
                        .p_3()
                        .gap_2()
                        .rounded(cx.theme().radius_lg)
                        .bg(cx.theme().sidebar)
                        .border_1()
                        .border_color(cx.theme().sidebar_border)
                        .shadow_lg()
                        // Voice channel info
                        .child(
                            v_flex()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .text_color(cx.theme().muted_foreground)
                                        .child("VOICE CONNECTED")
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .text_color(cx.theme().foreground)
                                        .child(channel_name)
                                )
                                .when_some(server_name, |this, name| {
                                    this.child(
                                        div()
                                            .text_xs()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(name)
                                    )
                                })
                        )
                        // Divider
                        .child(
                            div()
                                .h(px(1.0))
                                .w_full()
                                .bg(cx.theme().border)
                        )
                        // Controls row
                        .child(
                            h_flex()
                                .gap_1()
                                .items_center()
                                .justify_between()
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Button::new("voice-bar-mute")
                                                .icon(if is_muted { IconName::Minus } else { IconName::Plus })
                                                .when(is_muted, |this| this.danger())
                                                .when(!is_muted, |this| this.ghost())
                                                .xsmall()
                                                .tooltip(if is_muted { "Unmute" } else { "Mute" })
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.toggle_mute(cx);
                                                }))
                                        )
                                        .child(
                                            Button::new("voice-bar-deafen")
                                                .icon(if is_deafened { IconName::Minus } else { IconName::Plus })
                                                .when(is_deafened, |this| this.danger())
                                                .when(!is_deafened, |this| this.ghost())
                                                .xsmall()
                                                .tooltip(if is_deafened { "Undeafen" } else { "Deafen" })
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.toggle_deafen(cx);
                                                }))
                                        )
                                        .child(
                                            Button::new("voice-bar-settings")
                                                .icon(IconName::Settings)
                                                .ghost()
                                                .xsmall()
                                                .tooltip("Voice Settings")
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.open_settings(cx);
                                                }))
                                        )
                                )
                                .child(
                                    Button::new("voice-bar-disconnect")
                                        .icon(IconName::Close)
                                        .danger()
                                        .xsmall()
                                        .tooltip("Disconnect")
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.leave_voice_channel(cx);
                                        }))
                                )
                        )
                )
                .into_any_element(),
        )
    }
}
