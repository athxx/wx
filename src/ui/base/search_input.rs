use gpui::{
    div, App, Entity, Focusable, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    Styled, Window,
};
use gpui_component::{
    input::{Input, InputState},
    ActiveTheme, Icon, Sizable,
};

#[derive(IntoElement)]
pub struct SearchInput {
    state: Entity<InputState>,
}

impl SearchInput {
    pub fn new(state: Entity<InputState>) -> Self {
        Self { state }
    }
}

impl RenderOnce for SearchInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let weixin_colors = crate::ui::theme::Theme::weixin_colors(cx);
        let is_focused = self.state.read(cx).focus_handle(cx).is_focused(window);

        div()
            .flex_1()
            .bg(weixin_colors.search_bar_bg)
            .rounded(crate::ui::constants::radius_sm())
            .py_1()
            .border_1()
            .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .border_color(if is_focused {
                theme.primary
            } else {
                gpui::hsla(0., 0., 0., 0.)
            })
            .child(
                Input::new(&self.state)
                    .xsmall()
                    .prefix(
                        div().px_1().child(
                            Icon::default()
                                .path("search2.svg")
                                .text_color(theme.muted_foreground)
                                .xsmall(),
                        ),
                    )
                    .text_xs()
                    .cleanable(true)
                    .appearance(false),
            )
    }
}
