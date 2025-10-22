use crate::app::state::WeixinApp;
use crate::theme::Theme;
use gpui::{
    div, prelude::FluentBuilder, px, rgb, Context, InteractiveElement, IntoElement, ParentElement,
    Render, StatefulInteractiveElement, Styled, Window, WindowControlArea,
};
use gpui_component::{
    avatar::Avatar,
    button::{Button, ButtonVariants},
    h_flex,
    input::TextInput,
    resizable::{h_resizable, resizable_panel},
    v_flex, ContextModal, Icon, Sizable,
};

impl Render for WeixinApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx);

        v_flex()
            .size_full()
            .bg(theme.colors.titlebar_background)
            .child(self.render_title_bar(window, cx))
            .child(self.render_main_content(cx))
    }
}

impl WeixinApp {
    /// 渲染标题栏
    fn render_title_bar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx);
        let current_chat_title = self.get_current_chat_title();

        h_flex()
            .w_full()
            .h(px(67.))
            .items_center()
            .child(self.render_user_avatar())
            .child(
                h_resizable(
                    "title-search-resizable",
                    self.session_resizable_state.clone(),
                )
                .child(
                    resizable_panel()
                        .size(px(200.))
                        .size_range(px(200.)..px(400.))
                        .child(self.render_search_area(cx)),
                )
                .child(resizable_panel().child(self.render_chat_header(
                    &current_chat_title,
                    window,
                    &theme,
                ))),
            )
    }

    /// 渲染用户头像
    fn render_user_avatar(&self) -> impl IntoElement {
        div()
            .window_control_area(WindowControlArea::Drag)
            .w(px(67.))
            .flex()
            .items_center()
            .justify_center()
            .child(
                Avatar::new()
                    .w(px(40.))
                    .h(px(40.))
                    .rounded(px(6.))
                    .name("HL"),
            )
    }

    /// 渲染搜索区域
    fn render_search_area(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx);

        div()
            .bg(theme.colors.session_list_background)
            .size_full()
            .window_control_area(WindowControlArea::Drag)
            .flex()
            .border_l_1()
            .border_color(gpui::rgb(0xE1E1E1))
            .items_center()
            .px_3()
            .gap_2()
            .child(
                div()
                    .flex_1()
                    .bg(theme.colors.session_list_search_bg)
                    .rounded(px(4.))
                    .py_1()
                    .child(
                        TextInput::new(&self.session_list.read(cx).search_input)
                            .xsmall()
                            .prefix(
                                div().px_1().child(
                                    Icon::default()
                                        .path("search2.svg")
                                        .text_color(gpui::rgb(0xC3C3C3))
                                        .xsmall(),
                                ),
                            )
                            .text_xs()
                            .cleanable()
                            .appearance(false),
                    ),
            )
            .child(
                // 加号按钮
                h_flex()
                    .bg(theme.colors.session_list_search_bg)
                    .rounded(px(4.))
                    .w(px(28.))
                    .h(px(28.))
                    .justify_center()
                    .items_center()
                    .hover(|s| s.bg(gpui::rgb(0xE4E4E4)))
                    .child(
                        Icon::default()
                            .path("plus.svg")
                            .w(px(16.))
                            .h(px(16.))
                            .text_color(theme.colors.session_list_text),
                    ),
            )
    }

    /// 渲染聊天头部（单行布局）
    fn render_chat_header(&self, title: &str, window: &Window, theme: &Theme) -> impl IntoElement {
        let is_maximized = window.is_maximized();
        let title_text = title.to_string();

        h_flex()
            .h(px(67.)) // 与左侧高度一致
            .w_full()
            .items_center()
            .child(
                // 左侧：标题和功能按钮
                h_flex()
                    .window_control_area(WindowControlArea::Drag)
                    .h_full()
                    .flex_1()
                    .items_center()
                    .pl_3()
                    .child(
                        // 标题
                        div().text_color(theme.colors.chat_text).child(title_text),
                    ),
            )
            .child(
                h_flex()
                    .h_full()
                    .flex_col()
                    .items_center()
                    .child(self.render_window_controls(is_maximized, theme))
                    .child(
                        h_flex()
                            .window_control_area(WindowControlArea::Drag)
                            .flex_1()
                            .w_full()
                            .items_center()
                            .justify_end()
                            .pr_2()
                            .child(
                                div()
                                    .p(px(5.))
                                    .rounded(px(6.))
                                    .cursor_pointer()
                                    .hover(|this| this.bg(theme.colors.toolbar_active_bg))
                                    .child(
                                        Icon::default()
                                            .w(px(20.))
                                            .h(px(20.))
                                            .path("chat.svg")
                                            .text_color(theme.colors.chat_text),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .p(px(5.))
                                    .rounded(px(6.))
                                    .justify_center()
                                    .items_center()
                                    .mr_2()
                                    .cursor_pointer()
                                    .w(px(15.))
                                    .h(px(33.))
                                    .hover(|this| this.bg(theme.colors.toolbar_active_bg))
                                    .child(
                                        Icon::default()
                                            .path("down.svg")
                                            .w(px(20.))
                                            .h(px(20.))
                                            .text_color(theme.colors.chat_text),
                                    ),
                            )
                            .child(
                                div()
                                    .p(px(5.))
                                    .rounded(px(6.))
                                    .cursor_pointer()
                                    .hover(|this| this.bg(theme.colors.toolbar_active_bg))
                                    .child(
                                        Icon::default()
                                            .w(px(20.))
                                            .h(px(20.))
                                            .path("ellipses.svg")
                                            .text_color(theme.colors.chat_text),
                                    ),
                            ),
                    ),
            )
    }

    /// 渲染窗口控制按钮
    fn render_window_controls(&self, is_maximized: bool, theme: &Theme) -> impl IntoElement {
        h_flex()
            .h_8()
            .items_center()
            // 固定按钮
            .child(self.render_window_button(
                "win-btn-pin",
                "nail.svg",
                WindowControlArea::Min,
                theme,
            ))
            // 最小化按钮
            .child(self.render_window_button(
                "win-btn-min",
                "window-minimize.svg",
                WindowControlArea::Min,
                theme,
            ))
            // 最大化/还原按钮
            .child(self.render_window_button(
                "win-btn-max",
                if is_maximized {
                    "window-restore.svg"
                } else {
                    "window-maximize.svg"
                },
                WindowControlArea::Max,
                theme,
            ))
            // 关闭按钮
            .child(
                div()
                    .id("win-btn-close")
                    .flex()
                    .items_center()
                    .justify_center()
                    .h_full()
                    .w(px(45.))
                    .window_control_area(WindowControlArea::Close)
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(theme.colors.titlebar_close_hover)
                            .text_color(gpui::white())
                    })
                    .child(
                        Icon::default()
                            .path("window-close.svg")
                            .text_color(theme.colors.chat_text)
                            .xsmall(),
                    ),
            )
    }

    /// 渲染单个窗口控制按钮
    fn render_window_button(
        &self,
        id: &'static str,
        icon: &'static str,
        control: WindowControlArea,
        theme: &Theme,
    ) -> impl IntoElement {
        div()
            .id(id)
            .flex()
            .items_center()
            .justify_center()
            .h_full()
            .w(px(45.))
            .window_control_area(control)
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.titlebar_button_hover))
            .child(
                Icon::default()
                    .path(icon)
                    .text_color(theme.colors.chat_text)
                    .xsmall(),
            )
    }

    /// 渲染功能按钮行
    fn render_action_buttons(&self, theme: &Theme) -> impl IntoElement {
        h_flex()
            .h(px(32.))
            .w_full()
            .window_control_area(WindowControlArea::Drag)
            .items_center()
            .justify_end()
            .pr_3()
            .child(
                h_flex()
                    .h_full()
                    .items_center()
                    .child(
                        Button::new("chat-btn")
                            .icon(
                                Icon::default()
                                    .path("chat.svg")
                                    .text_color(theme.colors.chat_text),
                            )
                            .ghost()
                            .xsmall(),
                    )
                    .child(
                        Button::new("down-btn")
                            .icon(
                                Icon::default()
                                    .path("down.svg")
                                    .text_color(theme.colors.chat_text),
                            )
                            .ghost()
                            .xsmall(),
                    )
                    .child(
                        Button::new("more-btn")
                            .icon(
                                Icon::default()
                                    .path("ellipses.svg")
                                    .text_color(theme.colors.chat_text),
                            )
                            .ghost()
                            .xsmall(),
                    ),
            )
    }

    /// 渲染主内容区域
    fn render_main_content(&self, cx: &Context<Self>) -> impl IntoElement {
        h_flex()
            .flex_1()
            .w_full()
            .overflow_hidden()
            .child(self.toolbar.clone())
            .child(
                h_resizable(
                    "session-list-resizable",
                    self.session_resizable_state.clone(),
                )
                .child(
                    resizable_panel()
                        .size(px(200.))
                        .size_range(px(200.)..px(400.))
                        .child(self.session_list.clone()),
                )
                .child(resizable_panel().child(self.chat_area.clone())),
            )
    }
}
