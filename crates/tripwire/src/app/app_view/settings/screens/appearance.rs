use gpui::{div, px, AnyElement, Context, IntoElement, ParentElement, Styled, Window};
use gpui_component::{h_flex, v_flex, ActiveTheme as _, Sizable as _, StyledExt, button::{Button, ButtonVariants}};

use crate::app::TripwireApp;

pub fn render(_app: &TripwireApp, _window: &mut Window, cx: &mut Context<TripwireApp>) -> AnyElement {
    v_flex()
        .gap_6()
        .max_w(px(700.0))
        .child(
            v_flex()
                .gap_4()
                .child(div().text_lg().font_weight(gpui::FontWeight::SEMIBOLD).text_color(cx.theme().foreground).child("Theme"))
                .child(
                    h_flex()
                        .gap_3()
                        .child(
                            div()
                                .w(px(120.0))
                                .h(px(80.0))
                                .rounded(cx.theme().radius)
                                .border_2()
                                .border_color(cx.theme().accent)
                                .bg(cx.theme().background)
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(div().text_sm().text_color(cx.theme().foreground).child("Dark"))
                        )
                        .child(
                            div()
                                .w(px(120.0))
                                .h(px(80.0))
                                .rounded(cx.theme().radius)
                                .border_1()
                                .border_color(cx.theme().border)
                                .bg(gpui::rgb(0xFFFFFF))
                                .flex()
                                .items_center()
                                .justify_center()
                                .cursor_pointer()
                                .child(div().text_sm().text_color(gpui::rgb(0x000000)).child("Light"))
                        )
                )
        )
        .child(
            v_flex()
                .gap_4()
                .child(div().text_lg().font_weight(gpui::FontWeight::SEMIBOLD).text_color(cx.theme().foreground).child("Message Display"))
                .child(
                    v_flex()
                        .gap_2()
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .py_3()
                                .border_b_1()
                                .border_color(cx.theme().border)
                                .child(div().text_sm().text_color(cx.theme().foreground).child("Compact Mode"))
                                .child(Button::new("btn-compact").label("Toggle").ghost().with_size(gpui_component::Size::Small))
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .py_3()
                                .border_b_1()
                                .border_color(cx.theme().border)
                                .child(div().text_sm().text_color(cx.theme().foreground).child("Show Avatars"))
                                .child(Button::new("btn-avatars").label("On").ghost().with_size(gpui_component::Size::Small))
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .py_3()
                                .child(div().text_sm().text_color(cx.theme().foreground).child("Show Timestamps"))
                                .child(Button::new("btn-timestamps").label("On").ghost().with_size(gpui_component::Size::Small))
                        )
                )
        )
        .into_any_element()
}
