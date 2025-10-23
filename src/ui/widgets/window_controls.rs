use gpui::{
    div, rgb, white, InteractiveElement, IntoElement, ParentElement, Styled, WindowControlArea,
};
use gpui_component::{h_flex, Icon, Sizable};

pub fn window_controls(is_maximized: bool, theme: &gpui_component::Theme) -> impl IntoElement {
    h_flex()
        .h_8()
        .items_center()
        .child(window_button(
            "win-btn-pin",
            "nail.svg",
            WindowControlArea::Min,
            theme,
        ))
        .child(window_button(
            "win-btn-min",
            "window-minimize.svg",
            WindowControlArea::Min,
            theme,
        ))
        .child(window_button(
            "win-btn-max",
            if is_maximized {
                "window-restore.svg"
            } else {
                "window-maximize.svg"
            },
            WindowControlArea::Max,
            theme,
        ))
        .child(
            div()
                .id("win-btn-close")
                .flex()
                .items_center()
                .justify_center()
                .h_full()
                .w(crate::ui::constants::window_button_width())
                .window_control_area(WindowControlArea::Close)
                .cursor_pointer()
                .hover(|s| s.bg(rgb(0xe81123)).text_color(white()))
                .child(
                    Icon::default()
                        .path("window-close.svg")
                        .text_color(theme.foreground)
                        .xsmall(),
                ),
        )
}

fn window_button(
    id: &'static str,
    icon: &'static str,
    control: WindowControlArea,
    theme: &gpui_component::Theme,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .h_full()
        .w(crate::ui::constants::window_button_width())
        .window_control_area(control)
        .cursor_pointer()
        .hover(|s| s.bg(theme.secondary))
        .child(
            Icon::default()
                .path(icon)
                .text_color(theme.foreground)
                .xsmall(),
        )
}
