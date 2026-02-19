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
                .child(div().text_lg().font_weight(gpui::FontWeight::SEMIBOLD).text_color(cx.theme().foreground).child("Privacy Settings"))
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
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Direct Messages"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Who can send you direct messages"))
                                )
                                .child(Button::new("btn-dm-privacy").label("Everyone").ghost().with_size(gpui_component::Size::Small))
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .py_3()
                                .border_b_1()
                                .border_color(cx.theme().border)
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Read Receipts"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Let others know when you've read their messages"))
                                )
                                .child(Button::new("btn-receipts").label("On").ghost().with_size(gpui_component::Size::Small))
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .py_3()
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Typing Indicators"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Show when you're typing"))
                                )
                                .child(Button::new("btn-typing").label("On").ghost().with_size(gpui_component::Size::Small))
                        )
                )
        )
        .into_any_element()
}
