use gpui::{div, InteractiveElement, IntoElement, ParentElement, Styled};
use gpui_component::{h_flex, Icon};

pub fn icon_button(path: &'static str, theme: &gpui_component::Theme) -> impl IntoElement {
    div()
        .p(crate::ui::constants::icon_button_padding())
        .rounded(crate::ui::constants::radius_sm())
        .cursor_pointer()
        .hover(|this| this.bg(theme.secondary))
        .child(
            Icon::default()
                .path(path)
                .w(crate::ui::constants::icon_sm())
                .h(crate::ui::constants::icon_sm())
                .text_color(theme.foreground),
        )
}

pub fn header_like_narrow_down_button(theme: &gpui_component::Theme) -> impl IntoElement {
    h_flex()
        .p(crate::ui::constants::header_action_padding())
        .rounded(crate::ui::constants::radius_md())
        .justify_center()
        .items_center()
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
        )
}
