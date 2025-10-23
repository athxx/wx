use gpui::{div, IntoElement, ParentElement, Styled};
use gpui_component::h_flex;

pub fn chat_toolbar(theme: &gpui_component::Theme) -> impl IntoElement {
    h_flex()
        .w_full()
        .items_center()
        .child(
            // 左侧图标组
            h_flex()
                .gap_2()
                .child(crate::ui::widgets::icon_buttons::icon_button(
                    "emoji.svg",
                    theme,
                ))
                .child(crate::ui::widgets::icon_buttons::icon_button(
                    "favorite.svg",
                    theme,
                ))
                .child(crate::ui::widgets::icon_buttons::icon_button(
                    "file.svg", theme,
                ))
                .child(crate::ui::widgets::icon_buttons::icon_button(
                    "scissors.svg",
                    theme,
                ))
                .child(crate::ui::widgets::icon_buttons::narrow_icon_button(
                    "down.svg", theme,
                )),
        )
        .child(
            // 中间空白区域
            div().flex_1(),
        )
        .child(
            // 右侧图标组
            h_flex()
                .gap_2()
                .child(crate::ui::widgets::icon_buttons::icon_button(
                    "circle.svg",
                    theme,
                ))
                .child(crate::ui::widgets::icon_buttons::icon_button(
                    "video-call.svg",
                    theme,
                )),
        )
}
