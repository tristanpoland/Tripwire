use gpui::{div, px, AnyElement, Context, IntoElement, ParentElement, Styled, Window};
use gpui_component::{h_flex, v_flex, ActiveTheme as _, StyledExt, button::{Button, ButtonVariants}};

use crate::app::TripwireApp;

pub fn render(app: &TripwireApp, _window: &mut Window, cx: &mut Context<TripwireApp>) -> AnyElement {
    let server = app.active_server();
    
    v_flex()
        .gap_6()
        .max_w(px(700.0))
        .child(
            v_flex()
                .gap_4()
                .child(div().text_lg().font_weight(gpui::FontWeight::SEMIBOLD).text_color(cx.theme().foreground).child("Server Information"))
                .child(
                    v_flex()
                        .gap_3()
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
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Server Name"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child(server.map(|s| s.name.clone()).unwrap_or_else(|| "Unknown".to_string())))
                                )
                                .child(Button::new("btn-edit-name").label("Edit").ghost())
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
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Server Icon"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Change your server icon"))
                                )
                                .child(Button::new("btn-upload-icon").label("Upload").ghost())
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .py_3()
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Member Count"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child(format!("{} members", server.map(|s| s.members.len()).unwrap_or(0))))
                                )
                        )
                )
        )
        .into_any_element()
}
