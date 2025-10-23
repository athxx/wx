use crate::ui::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::*;

pub fn toggle(cx: &App, enabled: bool) -> impl IntoElement {
    let weixin_green = Theme::weixin_colors(cx).weixin_green;
    let toggle_off: Hsla = rgb(0xcccccc).into();

    div()
        .w(crate::ui::constants::toggle_width())
        .h(crate::ui::constants::toggle_height())
        .rounded(crate::ui::constants::toggle_radius())
        .cursor_pointer()
        .bg(if enabled { weixin_green } else { toggle_off })
        .flex()
        .items_center()
        .px(crate::ui::constants::toggle_padding_x())
        .child(
            div()
                .w(crate::ui::constants::toggle_handle_size())
                .h(crate::ui::constants::toggle_handle_size())
                .rounded(crate::ui::constants::toggle_handle_radius())
                .bg(gpui::white())
                .when(enabled, |this| this.ml_auto()),
        )
}
