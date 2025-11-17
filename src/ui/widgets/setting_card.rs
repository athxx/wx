use crate::ui::theme::Theme;
use gpui::*;
use gpui_component::ActiveTheme;

pub fn card<E: IntoElement>(cx: &App, content: E) -> impl IntoElement {
    let card_bg = Theme::weixin_colors(cx).session_list_bg;
    let border_color = cx.theme().border;

    div()
        .bg(card_bg)
        .rounded(crate::ui::constants::radius_lg())
        .border_1()
        .border_color(border_color)
        .p_4()
        .child(content)
}

pub fn divider(cx: &App) -> impl IntoElement {
    let border_color = cx.theme().border;

    div()
        .w_full()
        .h(crate::ui::constants::hairline())
        .bg(border_color)
}

#[allow(dead_code)]
pub fn section_title(cx: &App, title: &'static str) -> impl IntoElement {
    let foreground = cx.theme().foreground;

    div()
        .text_base()
        .font_weight(gpui::FontWeight::SEMIBOLD)
        .text_color(foreground)
        .child(title)
}
