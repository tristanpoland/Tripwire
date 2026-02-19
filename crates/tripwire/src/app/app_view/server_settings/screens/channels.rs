use gpui::{div, px, AnyElement, Context, IntoElement, ParentElement, Styled, Window};
use gpui_component::{h_flex, v_flex, ActiveTheme as _, StyledExt, button::Button};

use crate::app::TripwireApp;

pub fn render(app: &TripwireApp, _window: &mut Window, cx: &mut Context<TripwireApp>) -> AnyElement {
    let channel_count = app.active_server().map(|s| {
        s.categories.iter().map(|c| c.channels.len()).sum::<usize>()
    }).unwrap_or(0);
    
    v_flex()
        .gap_6()
        .max_w(px(700.0))
        .child(
            v_flex()
                .gap_4()
                .child(div().text_lg().font_weight(gpui::FontWeight::SEMIBOLD).text_color(cx.theme().foreground).child("Channels"))
                .child(
                    div()
                        .p_4()
                        .rounded(cx.theme().radius_lg)
                        .bg(cx.theme().muted)
                        .border_1()
                        .border_color(cx.theme().border)
                        .child(
                            v_flex()
                                .gap_2()
                                .child(div().text_sm().text_color(cx.theme().foreground).child(format!("This server has {} channels", channel_count)))
                                .child(Button::new("btn-create-channel").label("Create Channel"))
                        )
                )
        )
        .into_any_element()
}
