//! Custom title bar for Tripwire with theme and language selection.

use gpui::{
    Action, App, AppContext, Context, Corner, Entity, FocusHandle, InteractiveElement as _,
    IntoElement, MouseButton, ParentElement as _, Render, SharedString, Styled as _, Subscription,
    Window, div, px,
};
use gpui_component::{
    ActiveTheme as _, IconName, Side, Sizable as _, Theme, ThemeMode, ThemeRegistry, TitleBar,
    button::{Button, ButtonVariants as _},
    label::Label,
    menu::DropdownMenu as _,
};

#[derive(Action, Clone, PartialEq)]
#[action(namespace = tripwire_titlebar, no_json)]
pub struct SwitchTheme(pub SharedString);

#[derive(Action, Clone, PartialEq)]
#[action(namespace = tripwire_titlebar, no_json)]
pub struct SwitchThemeMode(pub ThemeMode);

#[derive(Action, Clone, PartialEq)]
#[action(namespace = tripwire_titlebar, no_json)]
pub struct SelectLanguage(pub SharedString);

pub struct TripwireTitleBar {
    theme_selector: Entity<ThemeSelector>,
    language_selector: Entity<LanguageSelector>,
    _subscriptions: Vec<Subscription>,
}

impl TripwireTitleBar {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let theme_selector = cx.new(|cx| ThemeSelector::new(window, cx));
        let language_selector = cx.new(|cx| LanguageSelector::new(window, cx));

        Self {
            theme_selector,
            language_selector,
            _subscriptions: vec![],
        }
    }
}

impl Render for TripwireTitleBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .child(
                        Label::new("Tripwire")
                            .text_base()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().foreground),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .px_2()
                    .gap_2()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child(self.language_selector.clone())
                    .child(self.theme_selector.clone()),
            )
    }
}

struct ThemeSelector {
    focus_handle: FocusHandle,
}

impl ThemeSelector {
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn on_switch_theme(&mut self, theme: &SwitchTheme, window: &mut Window, cx: &mut Context<Self>) {
        let theme_name = theme.0.clone();
        if let Some(theme_config) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
            Theme::global_mut(cx).apply_config(&theme_config);
            window.refresh();
        }
    }

    fn on_switch_theme_mode(
        &mut self,
        mode: &SwitchThemeMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        Theme::change(mode.0, None, cx);
        window.refresh();
    }
}

impl Render for ThemeSelector {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let current_theme = cx.theme().theme_name().clone();
        let mode = cx.theme().mode;

        div()
            .id("theme-selector")
            .track_focus(&focus_handle)
            .on_action(cx.listener(Self::on_switch_theme))
            .on_action(cx.listener(Self::on_switch_theme_mode))
            .child(
                Button::new("theme-btn")
                    .small()
                    .ghost()
                    .icon(IconName::Palette)
                    .dropdown_menu(move |mut this, _, _cx| {
                        this = this.scrollable(true)
                            .check_side(Side::Right)
                            .max_h(px(480.))
                            .label("Appearance")
                            .menu_with_check("Light", !mode.is_dark(), Box::new(SwitchThemeMode(ThemeMode::Light)))
                            .menu_with_check("Dark", mode.is_dark(), Box::new(SwitchThemeMode(ThemeMode::Dark)))
                            .separator()
                            .label("Themes")
                            .menu_with_check("Default Light", current_theme == "Default Light", Box::new(SwitchTheme("Default Light".into())))
                            .menu_with_check("Default Dark", current_theme == "Default Dark", Box::new(SwitchTheme("Default Dark".into())))
                            .menu_with_check("One Dark", current_theme == "One Dark", Box::new(SwitchTheme("One Dark".into())))
                            .menu_with_check("One Light", current_theme == "One Light", Box::new(SwitchTheme("One Light".into())));
                        
                        this
                    })
                    .anchor(Corner::TopRight),
            )
    }
}

struct LanguageSelector {
    focus_handle: FocusHandle,
}

impl LanguageSelector {
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn on_select_language(
        &mut self,
        lang: &SelectLanguage,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        rust_i18n::set_locale(lang.0.as_str());
        window.refresh();
    }
}

impl Render for LanguageSelector {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let current_locale = rust_i18n::locale().to_string();

        div()
            .id("language-selector")
            .track_focus(&focus_handle)
            .on_action(cx.listener(Self::on_select_language))
            .child(
                Button::new("lang-btn")
                    .small()
                    .ghost()
                    .icon(IconName::Globe)
                    .dropdown_menu(move |this, _, _cx| {
                        let current_locale = current_locale.clone();

                        this.check_side(Side::Right)
                            .label("Language")
                            .menu_with_check(
                                "English",
                                current_locale == "en",
                                Box::new(SelectLanguage("en".into())),
                            )
                            .menu_with_check(
                                "简体中文",
                                current_locale == "zh-CN",
                                Box::new(SelectLanguage("zh-CN".into())),
                            )
                            .menu_with_check(
                                "繁體中文",
                                current_locale == "zh-HK",
                                Box::new(SelectLanguage("zh-HK".into())),
                            )
                    })
                    .anchor(Corner::TopRight),
            )
    }
}
