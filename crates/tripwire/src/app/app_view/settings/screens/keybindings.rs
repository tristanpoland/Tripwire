use gpui::{div, px, AnyElement, Context, IntoElement, ParentElement, Styled, Window};
use gpui_component::{v_flex, ActiveTheme as _, StyledExt};

use crate::app::TripwireApp;

pub fn render(_app: &TripwireApp, _window: &mut Window, cx: &mut Context<TripwireApp>) -> AnyElement {
    v_flex()
        .gap_4()
        .max_w(px(700.0))
        .child(div().text_lg().font_weight(gpui::FontWeight::SEMIBOLD).text_color(cx.theme().foreground).child("Keybindings"))
        .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Keyboard shortcuts configuration coming soon..."))
        .into_any_element()
}
