use gpui::{div, InteractiveElement, IntoElement, ParentElement, Styled, WindowControlArea};
use gpui_component::{h_flex, Icon};

pub fn chat_header_actions(theme: &gpui_component::Theme) -> impl IntoElement {
    h_flex()
        .window_control_area(WindowControlArea::Drag)
        .flex_1()
        .w_full()
        .items_center()
        .justify_end()
        .pr_2()
        .child(
            div()
                .p(crate::ui::constants::header_action_padding())
                .rounded(crate::ui::constants::radius_md())
                .cursor_pointer()
                .hover(|this| this.bg(theme.secondary))
                .child(
                    Icon::default()
                        .w(crate::ui::constants::icon_sm())
                        .h(crate::ui::constants::icon_sm())
                        .path("chat.svg")
                        .text_color(theme.foreground),
                ),
        )
        .child(
            h_flex()
                .p(crate::ui::constants::header_action_padding())
                .rounded(crate::ui::constants::radius_md())
                .justify_center()
                .items_center()
                .mr_2()
                .cursor_pointer()
                .w(crate::ui::constants::header_narrow_button_width())
                .h(crate::ui::constants::header_narrow_button_height())
                .hover(|this| this.bg(theme.secondary))
                .child(
                    Icon::default()
                        .path("down.svg")
                        .w(crate::ui::constants::icon_sm())
                        .h(crate::ui::constants::icon_sm())
                        .text_color(theme.foreground),
                ),
        )
        .child(
            div()
                .p(crate::ui::constants::header_action_padding())
                .rounded(crate::ui::constants::radius_md())
                .cursor_pointer()
                .hover(|this| this.bg(theme.secondary))
                .child(
                    Icon::default()
                        .w(crate::ui::constants::icon_sm())
                        .h(crate::ui::constants::icon_sm())
                        .path("ellipses.svg")
                        .text_color(theme.foreground),
                ),
        )
}
