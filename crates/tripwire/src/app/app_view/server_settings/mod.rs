//! Server settings modal

pub mod screens;

use gpui::{
    div, prelude::FluentBuilder as _, px, AnyElement, Context, IntoElement,
    InteractiveElement, ParentElement, Styled, Window,
};
use gpui_component::{
    h_flex, v_flex, ActiveTheme as _, IconName, Sizable as _,StyledExt,
    button::{Button, ButtonVariants}, scroll::ScrollableElement as _,
};

use crate::app::TripwireApp;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerSettingsScreen {
    Overview,
    Roles,
    Channels,
    Members,
}

impl ServerSettingsScreen {
    pub fn label(&self) -> &'static str {
        match self {
            ServerSettingsScreen::Overview => "Overview",
            ServerSettingsScreen::Roles => "Roles",
            ServerSettingsScreen::Channels => "Channels",
            ServerSettingsScreen::Members => "Members",
        }
    }
}

impl TripwireApp {
    pub(crate) fn render_server_settings_modal(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let current_screen = self.server_settings_screen.clone();
        let server_name = self
            .active_server()
            .map(|s| s.name.clone())
            .unwrap_or_else(|| "Server".to_string());

        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::rgba(0x00000099))
            .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _, cx| {
                this.close_server_settings(cx);
            }))
            .child(
                div()
                    .occlude()
                    .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .w(px(900.0))
                    .h(px(650.0))
                    .bg(cx.theme().background)
                    .rounded(cx.theme().radius_lg)
                    .border_1()
                    .border_color(cx.theme().border)
                    .overflow_hidden()
                    .shadow_lg()
                    .child(
                        h_flex()
                            .size_full()
                            .child(self.render_server_settings_sidebar(&server_name, &current_screen, cx))
                            .child(self.render_server_settings_content(&current_screen, window, cx))
                    )
            )
            .into_any_element()
    }

    fn render_server_settings_sidebar(
        &self,
        server_name: &str,
        current_screen: &ServerSettingsScreen,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let screens = vec![
            ServerSettingsScreen::Overview,
            ServerSettingsScreen::Roles,
            ServerSettingsScreen::Channels,
            ServerSettingsScreen::Members,
        ];

        v_flex()
            .w(px(220.0))
            .h_full()
            .bg(cx.theme().sidebar)
            .border_r_1()
            .border_color(cx.theme().sidebar_border)
            .py_4()
            .gap_2()
            .child(
                div()
                    .px_3()
                    .py_2()
                    .text_sm()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(cx.theme().foreground)
                    .child(server_name.to_string())
            )
            .child(div().h(px(1.0)).bg(cx.theme().border))
            .children(screens.into_iter().map(|screen| {
                let is_selected = &screen == current_screen;
                let screen_clone = screen.clone();

                div()
                    .px_2()
                    .py_1()
                    .mx_2()
                    .rounded(cx.theme().radius)
                    .cursor_pointer()
                    .when(is_selected, |this| this.bg(cx.theme().accent))
                    .hover(|s| s.bg(cx.theme().accent))
                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, _, cx| {
                        this.switch_server_settings_screen(screen_clone.clone(), cx);
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(if is_selected {
                                cx.theme().accent_foreground
                            } else {
                                cx.theme().foreground
                            })
                            .child(screen.label())
                    )
            }))
            .child(div().flex_1())
            .child(
                div()
                    .px_2()
                    .pt_2()
                    .border_t_1()
                    .border_color(cx.theme().border)
                    .child(
                        Button::new("btn-delete-server")
                            .label("Delete Server")
                            .w_full()
                            .danger()
                    )
            )
    }

    fn render_server_settings_content(
        &self,
        screen: &ServerSettingsScreen,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .flex_1()
            .h_full()
            .overflow_hidden()
            .child(
                h_flex()
                    .h(px(60.0))
                    .flex_shrink_0()
                    .px_6()
                    .items_center()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(cx.theme().foreground)
                            .child(screen.label())
                    )
            )
            .child(
                div()
                    .flex_1()
                    .overflow_y_scrollbar()
                    .px_6()
                    .py_6()
                    .child(match screen {
                        ServerSettingsScreen::Overview => screens::overview::render(self, window, cx),
                        ServerSettingsScreen::Roles => screens::roles::render(self, window, cx),
                        ServerSettingsScreen::Channels => screens::channels::render(self, window, cx),
                        ServerSettingsScreen::Members => screens::members::render(self, window, cx),
                    })
            )
    }
}
