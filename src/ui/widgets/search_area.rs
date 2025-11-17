use gpui::{div, App, InteractiveElement, IntoElement, ParentElement, Styled, WindowControlArea};
use gpui_component::{h_flex, input::{Input, InputState}, ActiveTheme, Icon, Sizable};

pub fn search_area(search_input: &gpui::Entity<InputState>, cx: &App) -> impl IntoElement {
    let theme = cx.theme();
    let weixin_colors = crate::ui::theme::Theme::weixin_colors(cx);

    div()
        .bg(weixin_colors.session_list_bg)
        .size_full()
        .window_control_area(WindowControlArea::Drag)
        .flex()
        .border_l_1()
        .border_color(theme.border)
        .items_center()
        .px_3()
        .gap_2()
        .child(
            div()
                .flex_1()
                .bg(weixin_colors.search_bar_bg)
                .rounded(crate::ui::constants::radius_sm())
                .py_1()
                .child(
                    Input::new(search_input)
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
                ),
        )
        .child(
            h_flex()
                .bg(weixin_colors.search_bar_bg)
                .rounded(crate::ui::constants::radius_sm())
                .w(crate::ui::constants::search_plus_button_size())
                .h(crate::ui::constants::search_plus_button_size())
                .justify_center()
                .items_center()
                .hover(move |s| s.bg(weixin_colors.item_hover))
                .child(
                    Icon::default()
                        .path("plus.svg")
                        .w(crate::ui::constants::icon_xs())
                        .h(crate::ui::constants::icon_xs())
                        .text_color(theme.foreground),
                ),
        )
}
