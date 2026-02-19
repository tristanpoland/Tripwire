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

use crate::app::TripwireApp;

const STRIP_WIDTH: f32 = 72.;

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
            .pt_3()
            .pb_3()
            .gap_2()
            .overflow_hidden()
            .child(self.server_home_button(cx))
            .child(
                div()
                    .w(px(32.))
                    .h(px(2.))
                    .rounded_full()
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
                            .py_1()
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
                                    .when(is_active, |s| s.h(px(40.)))
                                    .when(!is_active, |s| s.h(px(0.))),
                            )
                            // Server avatar with tooltip
                            .child(
                                div()
                                    .id(ElementId::Name(SharedString::from(format!(
                                        "server-avatar-{ix}"
                                    ))))
                                    .relative()
                                    .tooltip(move |window, cx| {
                                        Tooltip::new(name.clone()).build(window, cx)
                                    })
                                    .child(Avatar::new().name(initials).with_size(
                                        gpui_component::Size::Large,
                                    ))
                                    // Unread badge
                                    .when(unread > 0 && !is_active, move |this| {
                                        this.child(
                                            div()
                                                .absolute()
                                                .bottom_0()
                                                .right_0()
                                                .w(px(16.))
                                                .h(px(16.))
                                                .rounded_full()
                                                .bg(danger_color)
                                                .border_2()
                                                .border_color(sidebar_color)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .text_color(gpui::white())
                                                .text_xs()
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
                    .tooltip(|window, cx| Tooltip::new("Add a Server").build(window, cx))
                    .on_click(|_, _, _| {}),
            )
            .child(
                Button::new("btn-settings")
                    .icon(IconName::Settings)
                    .ghost()
                    .xsmall()
                    .tooltip(|window, cx| Tooltip::new("User Settings").build(window, cx))
                    .on_click(|_, _, _| {}),
            )
            .into_any_element()
    }

    fn server_home_button(&self, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Button::new("btn-home")
            .icon(IconName::Inbox)
            .ghost()
            .small()
            .tooltip(|window, cx| Tooltip::new("Direct Messages").build(window, cx))
            .on_click(|_, _, _| {})
    }
}
