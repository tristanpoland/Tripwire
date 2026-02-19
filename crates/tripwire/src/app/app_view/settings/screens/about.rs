use gpui::{div, px, AnyElement, Context, IntoElement, ParentElement, Styled, Window};
use gpui_component::{h_flex, v_flex, ActiveTheme as _, StyledExt};

use crate::app::TripwireApp;

pub fn render(_app: &TripwireApp, _window: &mut Window, cx: &mut Context<TripwireApp>) -> AnyElement {
    v_flex()
        .gap_6()
        .max_w(px(700.0))
        .child(
            v_flex()
                .gap_2()
                .child(div().text_xl().font_weight(gpui::FontWeight::BOLD).text_color(cx.theme().foreground).child("Tripwire"))
                .child(div().text_sm().text_color(cx.theme().muted_foreground).child("A modern communication platform"))
        )
        .child(
            v_flex()
                .gap_2()
                .child(div().text_lg().font_weight(gpui::FontWeight::SEMIBOLD).text_color(cx.theme().foreground).child("Version Information"))
                .child(
                    v_flex()
                        .gap_1()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Version:"))
                                .child(div().text_sm().text_color(cx.theme().foreground).child("0.1.0"))
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Build:"))
                                .child(div().text_sm().text_color(cx.theme().foreground).child("Alpha"))
                        )
                )
        )
        .child(
            v_flex()
                .gap_2()
                .child(div().text_lg().font_weight(gpui::FontWeight::SEMIBOLD).text_color(cx.theme().foreground).child("Credits"))
                .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Built with GPUI Component Library"))
                .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Powered by Rust and GPUI"))
        )
        .into_any_element()
}
