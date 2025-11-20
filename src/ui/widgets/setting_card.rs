use crate::ui::theme::Theme;
use gpui::*;
use gpui_component::{h_flex, ActiveTheme};

pub fn card<E: IntoElement>(cx: &App, content: E) -> impl IntoElement {
    let card_bg = Theme::weixin_colors(cx).session_list_bg;
    let border_color = cx.theme().border;

    let body = div().w_full().child(content);

    div()
        .w_full()
        .bg(card_bg)
        .rounded(crate::ui::constants::radius_lg())
        .border_1()
        .border_color(border_color)
        .child(body)
}

pub fn divider(cx: &App) -> impl IntoElement {
    let border_color = cx.theme().border;

    h_flex()
        .w_full()
        .h(crate::ui::constants::hairline())
        // Leave a small inset on the left so the divider does not touch the card padding.
        .child(div().w(px(12.)).h_full())
        // The divider line occupies the remaining width and aligns with the right edge of the content.
        .child(
            div()
                .flex_1()
                .h(crate::ui::constants::hairline())
                .bg(border_color),
        )
}

/// Shared horizontal layout for setting-card rows so children stay left/right and vertically centered.
pub fn row() -> Div {
    h_flex().w_full().items_center().justify_between().px_4()
}
