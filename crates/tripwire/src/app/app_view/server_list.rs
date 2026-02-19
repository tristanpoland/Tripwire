//! Left-most server icon strip — 72 px wide vertical column.

use gpui::{
    AnyElement, AppContext as _, Context, ElementId, InteractiveElement as _, IntoElement as _,
    ParentElement as _, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _, px,
};
use gpui_component::{
    ActiveTheme as _, IconName, Sizable as _, StyledExt as _,
    avatar::Avatar,
    button::{Button, ButtonVariants as _},
    scroll::ScrollableElement as _,
    tooltip::Tooltip,
    v_flex,
};

use crate::app::{AppView, TripwireApp};

const STRIP_WIDTH: f32 = 56.;
const SERVER_ICON_SIZE: f32 = 48.;

impl TripwireApp {
    pub(crate) fn render_server_list(&mut self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .w(px(STRIP_WIDTH))
            .h_full()
            .flex_shrink_0()
            .bg(cx.theme().sidebar)
            .border_r_1()
            .border_color(cx.theme().sidebar_border)
            .items_center()
            .pt_2()
            .pb_2()
            .gap_2()
            .overflow_hidden()
            .child(self.server_home_button(cx))
            .child(
                div()
                    .w(px(32.))
                    .h(px(2.))
                    .rounded(px(2.))
                    .bg(cx.theme().border),
            )
            .children(
                self.servers
                    .iter()
                    .enumerate()
                    .map(|(ix, server)| {
                        let is_active = self.active_server == ix;
                        let name = server.name.clone();
                        let initials = server.initials();
                        let unread: usize = server.notification_count
                            + server.all_channels().iter().map(|c| c.unread).sum::<usize>();
                        let danger_color = cx.theme().danger;
                        let primary_color = cx.theme().primary;
                        let sidebar_color = cx.theme().sidebar;

                        div()
                            .id(ElementId::Name(SharedString::from(format!("server-{ix}"))))
                            .relative()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w_full()
                            .py(px(4.))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.switch_server(ix, window, cx);
                            }))
                            // Active indicator pill on left edge
                            .child(
                                div()
                                    .absolute()
                                    .left_0()
                                    .w(px(4.))
                                    .rounded_r(px(4.))
                                    .bg(primary_color)
                                    .when(is_active, |s| s.h(px(36.)))
                                    .when(!is_active, |s| s.h(px(0.))),
                            )
                            // Server avatar with tooltip - SQUIRCLE SHAPE
                            .child(
                                div()
                                    .id(ElementId::Name(SharedString::from(format!(
                                        "server-avatar-{ix}"
                                    ))))
                                    .relative()
                                    .tooltip(move |window, cx| {
                                        Tooltip::new(name.clone()).build(window, cx)
                                    })
                                    .child(
                                        div()
                                            .size(px(SERVER_ICON_SIZE))
                                            .rounded(px(16.))
                                            .bg(primary_color)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_color(gpui::white())
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_base()
                                            .when(!is_active, |this| {
                                                this.rounded(px(16.))
                                                    .hover(|s| s.rounded(px(12.)).bg(primary_color))
                                            })
                                            .when(is_active, |this| this.rounded(px(12.)))
                                            .child(initials),
                                    )
                                    // Unread badge
                                    .when(unread > 0 && !is_active, move |this| {
                                        this.child(
                                            div()
                                                .absolute()
                                                .bottom(px(-2.))
                                                .right(px(-2.))
                                                .min_w(px(18.))
                                                .h(px(18.))
                                                .px(px(4.))
                                                .rounded_full()
                                                .bg(danger_color)
                                                .border_2()
                                                .border_color(sidebar_color)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .text_color(gpui::white())
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::BOLD)
                                                .child(if unread > 9 {
                                                    "9+".to_string()
                                                } else {
                                                    unread.to_string()
                                                }),
                                        )
                                    }),
                            )
                    }),
            )
            .child(div().flex_1())
            .child(
                Button::new("btn-add-server")
                    .icon(IconName::Plus)
                    .ghost()
                    .xsmall()
                    .tooltip("Add a Server")
                    .on_click(|_, _, _| {}),
            )
            .child(
                Button::new("btn-settings")
                    .icon(IconName::Settings)
                    .ghost()
                    .xsmall()
                    .tooltip("User Settings")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.open_settings(cx);
                    })),
            )
            .into_any_element()
    }

    fn server_home_button(&self, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let is_dm_view = self.current_view == AppView::DirectMessages;
        let primary_color = cx.theme().primary;
        
        div()
            .id(ElementId::Name(SharedString::from("dm-home-button")))
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .tooltip(|window, cx| Tooltip::new("Direct Messages").build(window, cx))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.switch_to_dms(cx);
            }))
            .child(
                div()
                    .size(px(SERVER_ICON_SIZE))
                    .rounded(px(16.))
                    .bg(cx.theme().accent)
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(!is_dm_view, |this| {
                        this.rounded(px(16.))
                            .hover(|s| s.rounded(px(12.)).bg(primary_color))
                    })
                    .when(is_dm_view, |this| this.rounded(px(12.)).bg(primary_color))
                    .child(
                        gpui_component::Icon::new(IconName::Inbox)
                            .small()
                            .text_color(gpui::white()),
                    ),
            )
    }
}
