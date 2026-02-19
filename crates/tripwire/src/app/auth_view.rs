//! Login / registration screen.
//!
//! Implements `TripwireApp::render_auth`.

use gpui::{AnyElement, Context, IntoElement as _, SharedString, Window, div, prelude::*};
use gpui_component::{
    ActiveTheme as _,
    button::{Button, ButtonVariants as _},
    h_flex, v_flex,
    input::{Input, InputEvent, InputState},
};

use crate::app::TripwireApp;

impl TripwireApp {
    /// Render the full auth / login screen.
    pub(crate) fn render_auth(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let has_error = self.auth.login_error.is_some();
        let error_msg = self.auth.login_error.clone().unwrap_or_default();

        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .bg(cx.theme().background)
            .child(
                // ── Card ────────────────────────────────────────────────────
                v_flex()
                    .w(gpui::px(440.))
                    .gap_6()
                    .p_8()
                    .rounded(cx.theme().radius_lg)
                    .bg(cx.theme().card)
                    .shadow_lg()
                    // ── Branding ────────────────────────────────────────────
                    .child(
                        v_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_2xl()
                                    .font_bold()
                                    .text_color(cx.theme().foreground)
                                    .child("Tripwire"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("Welcome back! We're so excited to see you again."),
                            ),
                    )
                    // ── Fields ──────────────────────────────────────────────
                    .child(
                        v_flex()
                            .gap_4()
                            .child(self.email_field(cx))
                            .child(self.password_field(cx)),
                    )
                    // ── Error message ────────────────────────────────────────
                    .when(has_error, |this| {
                        this.child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().destructive)
                                .child(error_msg),
                        )
                    })
                    // ── Actions ──────────────────────────────────────────────
                    .child(self.login_actions(window, cx))
                    // ── Divider ─────────────────────────────────────────────
                    .child(
                        h_flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .flex_1()
                                    .h(gpui::px(1.))
                                    .bg(cx.theme().border),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("OR"),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .h(gpui::px(1.))
                                    .bg(cx.theme().border),
                            ),
                    )
                    // ── Dev bypass ──────────────────────────────────────────
                    .child(self.bypass_button(cx)),
            )
            .into_any_element()
    }

    // ── Helper builders ───────────────────────────────────────────────────────

    fn email_field(&self, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        v_flex()
            .gap_1()
            .child(
                div()
                    .text_xs()
                    .font_semibold()
                    .text_color(cx.theme().muted_foreground)
                    .child("EMAIL OR PHONE NUMBER"),
            )
            .child(Input::new(&self.email_input))
    }

    fn password_field(&self, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        v_flex()
            .gap_1()
            .child(
                div()
                    .text_xs()
                    .font_semibold()
                    .text_color(cx.theme().muted_foreground)
                    .child("PASSWORD"),
            )
            .child(Input::new(&self.password_input).mask_toggle())
    }

    fn login_actions(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        v_flex()
            .gap_2()
            .child(
                Button::new("btn-login")
                    .label("Log In")
                    .primary()
                    .w_full()
                    .on_click(cx.listener(|this, _, window, cx| {
                        let email = this.email_input.read(cx).value().to_string();
                        let password = this.password_input.read(cx).value().to_string();
                        this.auth.login(&email, &password);
                        cx.notify();
                    })),
            )
    }

    fn bypass_button(&self, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        v_flex()
            .items_center()
            .gap_3()
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child("Developing locally?"),
            )
            .child(
                Button::new("btn-bypass")
                    .label("Dev Bypass — Skip Login")
                    .ghost()
                    .w_full()
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.auth.bypass_login();
                        cx.notify();
                    })),
            )
    }
}
