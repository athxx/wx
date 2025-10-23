use gpui::{div, InteractiveElement, IntoElement, ParentElement, Styled};
use gpui_component::Icon;

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
                .text_color(theme.muted_foreground),
        )
}

pub fn narrow_icon_button(path: &'static str, theme: &gpui_component::Theme) -> impl IntoElement {
    div()
        .p(crate::ui::constants::icon_button_padding())
        .rounded(crate::ui::constants::radius_sm())
        .cursor_pointer()
        .w(crate::ui::constants::chat_toolbar_narrow_button_width())
        .hover(|this| this.bg(theme.secondary))
        .child(
            Icon::default()
                .path(path)
                .w(crate::ui::constants::icon_sm())
                .h(crate::ui::constants::icon_sm())
                .text_color(theme.muted_foreground),
        )
}
