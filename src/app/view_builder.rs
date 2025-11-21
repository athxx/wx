use crate::app::state::WeixinApp;
use crate::ui::fixed_resizable::fixed_h_resizable;
use crate::ui::theme::Theme;
use gpui::{
    div, px, Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window,
    WindowControlArea,
};
use gpui_component::{avatar::Avatar, h_flex, v_flex, ActiveTheme, Icon, Sizable};

impl Render for WeixinApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notification_layer = gpui_component::Root::render_notification_layer(window, cx);

        v_flex()
            .size_full()
            .on_mouse_down(gpui::MouseButton::Left, |_, window, _| {
                window.blur();
            })
            .child(div().w(px(0.)).h(px(0.)).track_focus(&self.focus_handle))
            .child(self.render_title_bar(window, cx))
            .child(self.render_main_content(cx))
            .children(notification_layer)
    }
}

impl WeixinApp {
    fn render_title_bar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use crate::ui::constants as UI;
        let current_chat_title = self.get_current_chat_title();
        let has_session = !current_chat_title.is_empty();

        h_flex()
            .w_full()
            .h(UI::title_bar_height())
            .items_center()
            .child(self.render_user_avatar(cx))
            .child(
                fixed_h_resizable("title-search-resizable", self.session_split_state.clone())
                    .width_range(
                        crate::ui::constants::session_list_min_width()
                            ..crate::ui::constants::session_list_max_width(),
                    )
                    .left(self.render_search_area(cx))
                    .right(self.render_chat_header(&current_chat_title, has_session, window, cx)),
            )
    }

    fn render_user_avatar(&self, cx: &Context<Self>) -> impl IntoElement {
        use crate::ui::constants as UI;
        let weixin_colors = Theme::weixin_colors(cx);
        div()
            .window_control_area(WindowControlArea::Drag)
            .w(UI::toolbar_width())
            .h_full()
            .bg(weixin_colors.toolbar_bg)
            .flex()
            .items_center()
            .justify_center()
            .child(
                Avatar::new()
                    .w(crate::ui::constants::title_avatar_size())
                    .h(crate::ui::constants::title_avatar_size())
                    .rounded(crate::ui::constants::radius_md())
                    .src(crate::ui::avatar::avatar_for_key("self")),
            )
    }

    fn render_search_area(&self, cx: &Context<Self>) -> impl IntoElement {
        let search_input = self.session_list.read(cx).search_input.clone();
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);

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
            .child(crate::ui::base::search_input::SearchInput::new(
                search_input,
            ))
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

    fn render_chat_header(
        &self,
        title: &str,
        has_session: bool,
        window: &Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);
        let is_maximized = window.is_maximized();
        let title_text = title.to_string();

        use crate::ui::constants as UI;

        // 左侧：拖动区域 + 可选标题文本
        let left_header: gpui::AnyElement = if has_session {
            h_flex()
                .window_control_area(WindowControlArea::Drag)
                .h_full()
                .flex_1()
                .items_center()
                .pl_3()
                .child(div().text_color(theme.foreground).child(title_text))
                .into_any_element()
        } else {
            h_flex()
                .window_control_area(WindowControlArea::Drag)
                .h_full()
                .flex_1()
                .items_center()
                .pl_3()
                .into_any_element()
        };

        // 右侧：窗口控制按钮 + （仅在有会话时）聊天头部按钮
        let right_header: gpui::AnyElement = if has_session {
            h_flex()
                .h_full()
                .flex_col()
                .items_center()
                .child(
                    crate::ui::base::window_controls::WindowControls::new()
                        .maximized(is_maximized)
                        .show_pin(true),
                )
                .child(crate::ui::composites::chat_header_actions::ChatHeaderActions::new())
                .into_any_element()
        } else {
            h_flex()
                .h_full()
                .flex_col()
                .items_center()
                .child(
                    crate::ui::base::window_controls::WindowControls::new()
                        .maximized(is_maximized)
                        .show_pin(true),
                )
                .into_any_element()
        };

        h_flex()
            .h(UI::title_bar_height())
            .w_full()
            .bg(weixin_colors.chat_area_bg)
            .items_center()
            .child(left_header)
            .child(right_header)
    }

    fn render_main_content(&self, cx: &Context<Self>) -> impl IntoElement {
        h_flex()
            .flex_1()
            .w_full()
            .overflow_hidden()
            .child(self.toolbar.clone())
            .child(
                fixed_h_resizable("session-list-resizable", self.session_split_state.clone())
                    .width_range(
                        crate::ui::constants::session_list_min_width()
                            ..crate::ui::constants::session_list_max_width(),
                    )
                    .left(self.session_list.clone())
                    .right(self.chat_area.clone()),
            )
    }
}
