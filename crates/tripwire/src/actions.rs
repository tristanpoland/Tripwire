//! Application-wide actions for keyboard shortcuts and menu items.

use gpui::actions;
use gpui_component::{ThemeMode, scroll::ScrollbarShow};

/// Select a specific font size (px value).
#[derive(gpui::Action, Clone, PartialEq, Eq)]
#[action(namespace = tripwire, no_json)]
pub struct SelectFont(pub usize);

/// Select a specific border radius (px value).
#[derive(gpui::Action, Clone, PartialEq, Eq)]
#[action(namespace = tripwire, no_json)]
pub struct SelectRadius(pub usize);

/// Control scrollbar visibility mode.
#[derive(gpui::Action, Clone, PartialEq, Eq)]
#[action(namespace = tripwire, no_json)]
pub struct SelectScrollbarShow(pub ScrollbarShow);

/// Switch between light and dark theme mode.
#[derive(gpui::Action, Clone, PartialEq)]
#[action(namespace = tripwire, no_json)]
pub struct SwitchThemeMode(pub ThemeMode);

actions!(tripwire, [ToggleListActiveHighlight]);
