//! Tripwire — a Discord alternative built on GPUI.
//!
//! Module hierarchy:
//!
//! ```
//! main.rs
//! ├── models.rs           — Data types (User, Server, Channel, Message)
//! ├── auth_state.rs       — Local auth persistence to disk
//! ├── mock_data.rs        — Sample servers / channels / messages
//! └── app.rs              — TripwireApp entity + Render impl
//!     ├── auth_view.rs    — impl TripwireApp: login screen
//!     └── app_view/
//!         ├── mod.rs      — impl TripwireApp: top-level Discord layout
//!         ├── server_list.rs   — left icon strip
//!         ├── channel_list.rs  — channel/category sidebar
//!         ├── chat_area.rs     — message list + composer
//!         └── members_panel.rs — online/offline user list
//! ```

mod app;
mod auth_state;
mod mock_data;
mod models;

use gpui::{
    App, Application, Bounds, WindowBounds, WindowKind,
    WindowOptions, actions, px, size,
};
use gpui::Focusable;
use gpui::AppContext;
use gpui_component::{Root, TitleBar};
use gpui_component_assets::Assets;

use app::TripwireApp;

actions!(tripwire, [Quit]);

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // Initialize the gpui-component library (theme, icons, fonts, etc.)
        gpui_component::init(cx);

        cx.bind_keys([
            #[cfg(target_os = "macos")]
            gpui::KeyBinding::new("cmd-q", Quit, None),
            #[cfg(not(target_os = "macos"))]
            gpui::KeyBinding::new("alt-f4", Quit, None),
        ]);

        cx.on_action(|_: &Quit, cx: &mut App| {
            cx.quit();
        });

        cx.activate(true);

        open_window(cx);
    });
}

fn open_window(cx: &mut App) {
    let window_size = compute_window_size(cx);
    let bounds = Bounds::centered(None, window_size, cx);

    cx.spawn(async move |cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitleBar::title_bar_options()),
            window_min_size: Some(gpui::Size {
                width: px(800.),
                height: px(500.),
            }),
            kind: WindowKind::Normal,
            #[cfg(target_os = "linux")]
            window_background: gpui::WindowBackgroundAppearance::Transparent,
            #[cfg(target_os = "linux")]
            window_decorations: Some(gpui::WindowDecorations::Client),
            ..Default::default()
        };

        let window = cx
            .open_window(options, |window, cx| {
                let app_view = cx.new(|cx| TripwireApp::new(window, cx));

                // Focus the root entity so keyboard shortcuts work immediately
                let focus = app_view.focus_handle(cx);
                window.defer(cx, move |window, cx| {
                    focus.focus(window, cx);
                });

                cx.new(|cx| Root::new(app_view, window, cx))
            })
            .expect("failed to open Tripwire window");

        window
            .update(cx, |_, window, _| {
                window.activate_window();
                window.set_window_title("Tripwire");
            })
            .expect("failed to update window title");

        Ok::<_, anyhow::Error>(())
    })
    .detach();
}

fn compute_window_size(cx: &mut App) -> gpui::Size<gpui::Pixels> {
    let mut s = size(px(1400.), px(900.));
    if let Some(display) = cx.primary_display() {
        let ds = display.bounds().size;
        s.width = s.width.min(ds.width * 0.9);
        s.height = s.height.min(ds.height * 0.9);
    }
    s
}
