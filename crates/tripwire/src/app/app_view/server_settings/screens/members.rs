use gpui::{div, px, AnyElement, Context, IntoElement, ParentElement, Styled, Window};
use gpui_component::{v_flex, ActiveTheme as _, StyledExt};

use crate::app::TripwireApp;

pub fn render(app: &TripwireApp, _window: &mut Window, cx: &mut Context<TripwireApp>) -> AnyElement {
    let members = app.active_server().map(|s| s.members.clone()).unwrap_or_default();
    
    v_flex()
        .gap_6()
        .max_w(px(700.0))
        .child(
            v_flex()
                .gap_4()
                .child(div().text_lg().font_weight(gpui::FontWeight::SEMIBOLD).text_color(cx.theme().foreground).child(format!("Members ({})", members.len())))
                .child(
                    v_flex()
                        .gap_2()
                        .children(members.into_iter().take(10).map(|member| {
                            div()
                                .p_3()
                                .rounded(cx.theme().radius)
                                .bg(cx.theme().muted)
                                .child(div().text_sm().text_color(cx.theme().foreground).child(member.username))
                        }))
                )
        )
        .into_any_element()
}
