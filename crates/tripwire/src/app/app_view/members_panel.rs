//! Right-side member list panel — 240 px wide, showing online / offline users.

use gpui::{AnyElement, Context, ElementId, IntoElement as _, SharedString, div, px};
use gpui::InteractiveElement;
use gpui::StatefulInteractiveElement;
use gpui::ParentElement;
use gpui_component::StyledExt;
use gpui::Styled;
use gpui_component::{
    ActiveTheme as _, Sizable as _,
    avatar::Avatar,
    h_flex, v_flex,
    scroll::ScrollableElement as _,
};

use crate::app::TripwireApp;
use crate::models::{User, UserStatus};

const PANEL_WIDTH: f32 = 240.;

impl TripwireApp {
    pub(crate) fn render_members_panel(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let members = self
            .active_server()
            .map(|s| s.members.clone())
            .unwrap_or_default();

        let (online, offline): (Vec<_>, Vec<_>) =
            members.iter().partition(|u| u.is_online());

        // Pre-compute all children before building the element tree to avoid
        // multiple mutable borrows of `cx` inside closures.
        let mut items: Vec<AnyElement> = Vec::new();

        if !online.is_empty() {
            items.push(
                self.render_section_header(&format!("ONLINE — {}", online.len()), cx)
                    .into_any_element(),
            );
            for u in &online {
                items.push(self.render_member_row(u, cx).into_any_element());
            }
        }

        if !offline.is_empty() {
            items.push(div().mt_4().into_any_element());
            items.push(
                self.render_section_header(&format!("OFFLINE — {}", offline.len()), cx)
                    .into_any_element(),
            );
            for u in &offline {
                items.push(self.render_member_row(u, cx).into_any_element());
            }
        }

        v_flex()
            .w(px(PANEL_WIDTH))
            .h_full()
            .flex_shrink_0()
            .bg(cx.theme().sidebar)
            .border_l_1()
            .border_color(cx.theme().sidebar_border)
            .overflow_hidden()
            .child(
                div()
                    .flex_1()
                    .overflow_y_scrollbar()
                    .px_2()
                    .py_4()
                    .children(items),
            )
            .into_any_element()
    }

    fn render_section_header(&self, label: &str, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        div()
            .px_2()
            .py_1()
            .text_xs()
            .font_semibold()
            .text_color(cx.theme().muted_foreground)
            .child(label.to_string())
    }

    fn render_member_row(&self, user: &User, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let username = user.username.clone();
        let avatar_name = username.clone();
        let status = user.status.clone();
        let user_id = user.id.clone();
        let user_clone = user.clone();

        let status_color = match status {
            UserStatus::Online => gpui::hsla(142. / 360., 0.71, 0.45, 1.),
            UserStatus::Idle => gpui::hsla(43. / 360., 0.85, 0.56, 1.),
            UserStatus::DoNotDisturb => gpui::hsla(0. / 360., 0.85, 0.60, 1.),
            UserStatus::Offline => gpui::hsla(0., 0., 0.55, 1.),
        };

        let status_label = match &status {
            UserStatus::Online => "Online",
            UserStatus::Idle => "Idle",
            UserStatus::DoNotDisturb => "Do Not Disturb",
            UserStatus::Offline => "Offline",
        };

        h_flex()
            .id(ElementId::Name(SharedString::from(format!(
                "member-{user_id}"
            ))))
            .px_2()
            .py_1()
            .gap_2()
            .items_center()
            .rounded(cx.theme().radius)
            .cursor_pointer()
            .hover(|s| s.bg(cx.theme().sidebar_accent))
            .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, _, cx| {
                this.show_user_profile(user_clone.clone(), cx);
            }))
            // Avatar with status dot
            .child(
                div()
                    .relative()
                    .flex_shrink_0()
                    .child(Avatar::new().name(avatar_name).xsmall())
                    // Status dot
                    .child(
                        div()
                            .absolute()
                            .bottom_0()
                            .right_0()
                            .w(px(10.))
                            .h(px(10.))
                            .rounded_full()
                            .bg(status_color)
                            .border_2()
                            .border_color(cx.theme().sidebar),
                    ),
            )
            // Name + status text
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_0()
                    .child(
                        div()
                            .text_sm()
                            .font_semibold()
                            .text_color(cx.theme().sidebar_foreground)
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(username),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(status_label),
                    ),
            )
    }
}
