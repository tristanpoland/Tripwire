use gpui::{div, px, AnyElement, Context, IntoElement, ParentElement, Styled, Window};
use gpui_component::{h_flex, v_flex, ActiveTheme as _, Sizable as _, StyledExt, avatar::Avatar, button::{Button, ButtonVariants}};
use gpui::prelude::FluentBuilder as _;

use crate::app::TripwireApp;

pub fn render(app: &TripwireApp, _window: &mut Window, cx: &mut Context<TripwireApp>) -> AnyElement {
    let user = app.auth.current_user.as_ref();
    
    v_flex()
        .gap_6()
        .max_w(px(700.0))
        // User profile card
        .child(
            div()
                .p_4()
                .rounded(cx.theme().radius_lg)
                .bg(cx.theme().muted)
                .border_1()
                .border_color(cx.theme().border)
                .child(
                    h_flex()
                        .gap_4()
                        .items_center()
                        .when_some(user, |this: gpui::Div, u| {
                            this.child(Avatar::new().name(u.username.clone()).with_size(gpui_component::Size::Large))
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_xl()
                                                .font_weight(gpui::FontWeight::BOLD)
                                                .text_color(cx.theme().foreground)
                                                .child(u.username.clone())
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(cx.theme().muted_foreground)
                                                .child(format!("{}#{}", u.username, u.discriminator))
                                        )
                                )
                        })
                        .when(user.is_none(), |this| {
                            this.child(div().text_color(cx.theme().muted_foreground).child("Not logged in"))
                        })
                )
        )
        // Account settings
        .child(
            v_flex()
                .gap_4()
                .child(
                    div()
                        .text_lg()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(cx.theme().foreground)
                        .child("Account Settings")
                )
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
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Email"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Change your email address"))
                                )
                                 .child(Button::new("btn-edit-email").label("Edit").ghost().with_size(gpui_component::Size::Small))
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
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Password"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Change your password"))
                                )
                                .child(Button::new("btn-edit-password").label("Change").ghost().with_size(gpui_component::Size::Small))
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .py_3()
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).text_color(cx.theme().foreground).child("Two-Factor Authentication"))
                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Add extra security to your account"))
                                )
                                .child(Button::new("btn-setup-2fa").label("Enable").ghost().with_size(gpui_component::Size::Small))
                        )
                )
        )
        .into_any_element()
}
