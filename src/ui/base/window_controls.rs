use gpui::{
    div, rgb, white, App, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled,
    Window, WindowControlArea,
};
use gpui_component::{h_flex, ActiveTheme, Icon, Sizable};

#[derive(IntoElement)]
pub struct WindowControls {
    is_maximized: bool,
    show_pin: bool,
}

impl WindowControls {
    pub fn new() -> Self {
        Self {
            is_maximized: false,
            show_pin: false,
        }
    }

    pub fn maximized(mut self, maximized: bool) -> Self {
        self.is_maximized = maximized;
        self
    }

    pub fn show_pin(mut self, show: bool) -> Self {
        self.show_pin = show;
        self
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
}

impl RenderOnce for WindowControls {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let mut header = h_flex().h_8().items_center();

        if self.show_pin {
            header = header.child(Self::window_button(
                "win-btn-pin",
                "nail.svg",
                WindowControlArea::Min,
                theme,
            ));
        }

        header
            .child(Self::window_button(
                "win-btn-min",
                "window-minimize.svg",
                WindowControlArea::Min,
                theme,
            ))
            .child(Self::window_button(
                "win-btn-max",
                if self.is_maximized {
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
}
