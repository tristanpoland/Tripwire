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
                .child(div().text_lg().font_weight(gpui::FontWeight::SEMIBOLD).text_color(cx.theme().foreground).child("Notification Settings"))
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
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Enable Notifications"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Receive notifications for new messages"))
                                )
                                .child(Button::new("btn-notifs").label("On").ghost().with_size(gpui_component::Size::Small))
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
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Sound"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Play a sound for notifications"))
                                )
                                .child(Button::new("btn-sound").label("On").ghost().with_size(gpui_component::Size::Small))
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .py_3()
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Show Badge"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Display unread count on app icon"))
                                )
                                .child(Button::new("btn-badge").label("On").ghost().with_size(gpui_component::Size::Small))
                        )
                )
        )
        .into_any_element()
}
