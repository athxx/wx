use crate::app::state::WeixinApp;
use crate::ui::theme::Theme;
use gpui::{
    div, px, Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window,
    WindowControlArea,
};
use gpui_component::{
    avatar::Avatar,
    h_flex,
    resizable::{h_resizable, resizable_panel},
    v_flex, ActiveTheme,
};

impl Render for WeixinApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 不设置整体背景，让各个区域自己设置
        v_flex()
            .size_full()
            .child(self.render_title_bar(window, cx))
            .child(self.render_main_content(cx))
    }
}

impl WeixinApp {
    /// 渲染标题栏
    fn render_title_bar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use crate::ui::constants as UI;
        let current_chat_title = self.get_current_chat_title();

        h_flex()
            .w_full()
            .h(UI::title_bar_height())
            .items_center()
            .child(self.render_user_avatar(cx))
            .child(
                h_resizable(
                    "title-search-resizable",
                    self.session_resizable_state.clone(),
                )
                .child(
                    resizable_panel()
                        .size(crate::ui::constants::session_list_min_width())
                        .size_range(
                            crate::ui::constants::session_list_min_width()
                                ..crate::ui::constants::session_list_max_width(),
                        )
                        .child(self.render_search_area(cx)),
                )
                .child(resizable_panel().child(self.render_chat_header(
                    &current_chat_title,
                    window,
                    cx,
                ))),
            )
    }

    /// 渲染用户头像
    fn render_user_avatar(&self, cx: &Context<Self>) -> impl IntoElement {
        use crate::ui::constants as UI;
        let weixin_colors = Theme::weixin_colors(cx);
        div()
            .window_control_area(WindowControlArea::Drag)
            .w(UI::toolbar_width())
            .h_full()
            .bg(weixin_colors.toolbar_bg) // 左侧工具栏背景
            .flex()
            .items_center()
            .justify_center()
            .child(
                Avatar::new()
                    .w(crate::ui::constants::title_avatar_size())
                    .h(crate::ui::constants::title_avatar_size())
                    .rounded(crate::ui::constants::radius_md())
                    .name("HL"),
            )
    }

    /// 渲染搜索区域
    fn render_search_area(&self, cx: &Context<Self>) -> impl IntoElement {
        let search_input = self.session_list.read(cx).search_input.clone();
        crate::ui::widgets::search_area::search_area(&search_input, cx)
    }

    /// 渲染聊天头部（单行布局）
    fn render_chat_header(
        &self,
        title: &str,
        window: &Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);
        let is_maximized = window.is_maximized();
        let title_text = title.to_string();

        use crate::ui::constants as UI;
        h_flex()
            .h(UI::title_bar_height()) // 与左侧高度一致
            .w_full()
            .bg(weixin_colors.chat_area_bg) // 右侧聊天区域背景 EDEDED
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
                        div().text_color(theme.foreground).child(title_text),
                    ),
            )
            .child(
                h_flex()
                    .h_full()
                    .flex_col()
                    .items_center()
                    .child(crate::ui::widgets::window_controls::window_controls(
                        is_maximized,
                        theme,
                    ))
                    .child(crate::ui::widgets::chat_header_actions::chat_header_actions(theme)),
            )
    }

    /// 渲染主内容区域
    fn render_main_content(&self, _cx: &Context<Self>) -> impl IntoElement {
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
                        .size(crate::ui::constants::session_list_min_width())
                        .size_range(
                            crate::ui::constants::session_list_min_width()
                                ..crate::ui::constants::session_list_max_width(),
                        )
                        .child(self.session_list.clone()),
                )
                .child(resizable_panel().child(self.chat_area.clone())),
            )
    }
}
