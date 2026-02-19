pub mod screens;

use gpui::{
    div, prelude::FluentBuilder as _, px, AnyElement, Context, IntoElement,
    InteractiveElement, ParentElement, Styled, Window,
};
use gpui_component::{
    h_flex, v_flex, ActiveTheme as _, IconName, Sizable as _, StyledExt,
    button::{Button, ButtonVariants}, scroll::ScrollableElement as _,
};

use crate::app::TripwireApp;

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsScreen {
    Account,
    Appearance,
    Notifications,
    Privacy,
    Keybindings,
    Language,
    About,
}

impl SettingsScreen {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Account => "My Account",
            Self::Appearance => "Appearance",
            Self::Notifications => "Notifications",
            Self::Privacy => "Privacy & Safety",
            Self::Keybindings => "Keybindings",
            Self::Language => "Language",
            Self::About => "About",
        }
    }
}

impl TripwireApp {
    pub(crate) fn render_settings_modal(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let current_screen = self.settings_screen.clone();
        
        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::rgba(0x00000099))
            .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, _, cx| {
                this.close_settings(cx);
            }))
            .child(
                div()
                    .occlude()
                    .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .w(px(1000.0))
                    .h(px(700.0))
                    .bg(cx.theme().background)
                    .rounded(cx.theme().radius_lg)
                    .border_1()
                    .border_color(cx.theme().border)
                    .overflow_hidden()
                    .shadow_lg()
                    .child(
                        h_flex()
                            .size_full()
                            .child(self.render_settings_sidebar(&current_screen, cx))
                            .child(self.render_settings_content(&current_screen, window, cx))
                    )
            )
            .into_any_element()
    }

    fn render_settings_sidebar(
        &self,
        current_screen: &SettingsScreen,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let screens = vec![
            (SettingsScreen::Account, "USER SETTINGS"),
            (SettingsScreen::Appearance, "APP SETTINGS"),
            (SettingsScreen::Notifications, "APP SETTINGS"),
            (SettingsScreen::Privacy, "APP SETTINGS"),
            (SettingsScreen::Keybindings, "ADVANCED"),
            (SettingsScreen::Language, "ADVANCED"),
            (SettingsScreen::About, "ADVANCED"),
        ];

        let mut last_category = "";
        let mut sidebar_items: Vec<AnyElement> = Vec::new();

        for (screen, category) in screens {
            if category != last_category {
                if !last_category.is_empty() {
                    sidebar_items.push(div().h(px(16.0)).into_any_element());
                }
                sidebar_items.push(
                    div()
                        .px_3()
                        .py_2()
                        .text_xs()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(cx.theme().muted_foreground)
                        .child(category.to_string())
                        .into_any_element(),
                );
                last_category = category;
            }

            let is_selected = &screen == current_screen;
            let screen_clone = screen.clone();
            let icon = match &screen {
                SettingsScreen::Account => IconName::User,
                SettingsScreen::Appearance => IconName::Palette,
                SettingsScreen::Notifications => IconName::Bell,
                SettingsScreen::Privacy => IconName::Eye,
                SettingsScreen::Keybindings => IconName::Settings2,
                SettingsScreen::Language => IconName::Globe,
                SettingsScreen::About => IconName::Info,
            };

            sidebar_items.push(
                div()
                    .px_2()
                    .py_1()
                    .mx_2()
                    .rounded(cx.theme().radius)
                    .cursor_pointer()
                    .when(is_selected, |this| this.bg(cx.theme().accent))
                    .hover(|s| s.bg(cx.theme().accent))
                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, _, cx| {
                        this.switch_settings_screen(screen_clone.clone(), cx);
                    }))
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .py_1()
                            .child(gpui_component::Icon::new(icon).small().text_color(if is_selected { cx.theme().accent_foreground } else { cx.theme().foreground }))
                            .child(div().text_sm().text_color(if is_selected { cx.theme().accent_foreground } else { cx.theme().foreground }).child(screen.label()))
                    )
                    .into_any_element(),
            );
        }

        sidebar_items.push(div().flex_1().into_any_element());
        sidebar_items.push(
            div()
                .px_2()
                .py_4()
                .border_t_1()
                .border_color(cx.theme().border)
                .child(Button::new("btn-logout-settings").label("Log Out").w_full().on_click(cx.listener(|this, _, window, cx| { this.logout(window, cx); })))
                .into_any_element(),
        );

        v_flex()
            .w(px(240.0))
            .h_full()
            .bg(cx.theme().sidebar)
            .border_r_1()
            .border_color(cx.theme().sidebar_border)
            .py_4()
            .children(sidebar_items)
    }

    fn render_settings_content(
        &self,
        screen: &SettingsScreen,
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
                    .child(div().flex_1().text_xl().font_weight(gpui::FontWeight::BOLD).text_color(cx.theme().foreground).child(screen.label()))
                    .child(Button::new("btn-close-settings").icon(IconName::Close).ghost().on_click(cx.listener(|this, _, _, cx| { this.close_settings(cx); })))
            )
            .child(
                div()
                    .flex_1()
                    .overflow_y_scrollbar()
                    .px_6()
                    .py_4()
                    .child(match screen {
                        SettingsScreen::Account => screens::account::render(self, window, cx),
                        SettingsScreen::Appearance => screens::appearance::render(self, window, cx),
                        SettingsScreen::Notifications => screens::notifications::render(self, window, cx),
                        SettingsScreen::Privacy => screens::privacy::render(self, window, cx),
                        SettingsScreen::Keybindings => screens::keybindings::render(self, window, cx),
                        SettingsScreen::Language => screens::language::render(self, window, cx),
                        SettingsScreen::About => screens::about::render(self, window, cx),
                    })
            )
    }
}
