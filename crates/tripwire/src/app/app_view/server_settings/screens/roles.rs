use gpui::{div, px, AnyElement, Context, IntoElement, ParentElement, Styled, Window};
use gpui_component::{h_flex, v_flex, ActiveTheme as _, StyledExt, button::Button};

use crate::app::TripwireApp;

pub fn render(_app: &TripwireApp, _window: &mut Window, cx: &mut Context<TripwireApp>) -> AnyElement {
    v_flex()
        .gap_6()
        .max_w(px(700.0))
        .child(
            v_flex()
                .gap_4()
                .child(div().text_lg().font_weight(gpui::FontWeight::SEMIBOLD).text_color(cx.theme().foreground).child("Roles"))
                .child(
                    div()
                        .p_4()
                        .rounded(cx.theme().radius_lg)
                        .bg(cx.theme().muted)
                        .border_1()
                        .border_color(cx.theme().border)
                        .child(
                            v_flex()
                                .gap_2()
                                .child(div().text_sm().text_color(cx.theme().foreground).child("Create and manage roles for your server"))
                                .child(Button::new("btn-create-role").label("Create Role"))
                        )
                )
        )
        .into_any_element()
}
