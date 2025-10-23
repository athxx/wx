use gpui::{
    div, px, AnyElement, App, AppContext, Context, Entity, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, StatefulInteractiveElement, Styled, Window,
};
use gpui_component::{
    button::{Button, ButtonVariants},
    h_flex,
    highlighter::Language,
    input::{InputState, TabSize, TextInput},
    v_flex, ActiveTheme,
};

use crate::models::{ChatSession, Message};
use crate::ui::theme::Theme;

pub struct ChatArea {
    current_session: Option<ChatSession>,
    input_state: Entity<InputState>,
    on_send_message: Option<Box<dyn Fn(String) + 'static>>,
    current_input_height: Pixels,
    min_input_height: Pixels,
    max_input_height: Pixels,
    is_resizing: bool,
    drag_start_y: Pixels,
    drag_start_height: Pixels,
}

impl ChatArea {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let default_height = crate::ui::constants::chat_input_default_height();

        let input_state = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor(Language::Markdown)
                .line_number(false)
                .tab_size(TabSize {
                    tab_size: 2,
                    ..Default::default()
                })
        });

        Self {
            current_session: None,
            input_state,
            on_send_message: None,
            current_input_height: default_height,
            min_input_height: crate::ui::constants::chat_input_min_height(),
            max_input_height: crate::ui::constants::chat_input_max_height(),
            is_resizing: false,
            drag_start_y: px(0.),
            drag_start_height: default_height,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn set_session(&mut self, session: Option<ChatSession>, cx: &mut Context<Self>) {
        self.current_session = session;
        cx.notify();
    }

    pub fn on_send_message<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + 'static,
    {
        self.on_send_message = Some(Box::new(callback));
        self
    }

    pub fn add_message(&mut self, message: Message, cx: &mut Context<Self>) {
        if let Some(session) = &mut self.current_session {
            session.add_message(message);
            cx.notify();
        }
    }

    fn send_message(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let content = self.input_state.read(cx).value();
        if content.trim().is_empty() {
            return;
        }

        if let Some(callback) = &self.on_send_message {
            callback(content.to_string());
        }

        // 清空输入框
        self.input_state.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
    }

    fn render_input_area(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.theme();

        v_flex()
            .size_full()
            .child(
                div()
                    .w_full()
                    .px_3()
                    .py_1p5()
                    .child(crate::ui::widgets::chat_toolbar::chat_toolbar(theme)),
            )
            .child(
                div().flex_1().w_full().px_2().overflow_hidden().child(
                    TextInput::new(&self.input_state)
                        .text_sm()
                        .appearance(false)
                        .w_full()
                        .h_full(),
                ),
            )
            .child(
                div().w_full().flex().justify_end().px_2().pb_2().child(
                    Button::new("send")
                        .child(h_flex().text_sm().items_center().child("发送(S)"))
                        .w_24()
                        .success()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.send_message(window, cx);
                        })),
                ),
            )
            .into_any_element()
    }
}

impl ChatArea {
    fn begin_resize(&mut self, _window: &mut Window, _cx: &mut Context<Self>, start_y: Pixels) {
        self.is_resizing = true;
        self.drag_start_y = start_y;
        self.drag_start_height = self.current_input_height;
    }

    fn update_resize(&mut self, _window: &mut Window, cx: &mut Context<Self>, current_y: Pixels) {
        if !self.is_resizing {
            return;
        }
        let dy = current_y - self.drag_start_y;
        let mut new_h = self.drag_start_height - dy;
        if new_h < self.min_input_height {
            new_h = self.min_input_height;
        }
        if new_h > self.max_input_height {
            new_h = self.max_input_height;
        }
        self.current_input_height = new_h;

        cx.notify();
    }

    fn end_resize(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.is_resizing = false;
    }
}

impl Render for ChatArea {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);
        let no_session_text_color = theme.muted_foreground;
        let border_color = theme.border;
        let bg_color = weixin_colors.chat_area_bg;

        let messages_view = if let Some(session) = &self.current_session {
            let is_group = session.contact.is_group;
            crate::ui::widgets::message_list::message_list(
                &session.messages,
                is_group,
                theme,
                &weixin_colors,
            )
            .into_any_element()
        } else {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .text_color(no_session_text_color)
                .text_base()
                .child("请选择一个会话开始聊天")
                .into_any_element()
        };

        v_flex()
            .flex_1()
            .bg(bg_color)
            .on_mouse_up(
                gpui::MouseButton::Left,
                cx.listener(|this, _evt: &gpui::MouseUpEvent, _window, cx| {
                    this.end_resize(_window, cx);
                }),
            )
            .on_mouse_move(
                cx.listener(|this, evt: &gpui::MouseMoveEvent, _window, cx| {
                    let y = evt.position.y;
                    this.update_resize(_window, cx, y);
                }),
            )
            .child(
                div()
                    .id("chat-messages")
                    .flex_1()
                    .w_full()
                    .bg(bg_color)
                    .overflow_y_scroll()
                    .border_t_1()
                    .border_color(border_color)
                    .child(messages_view),
            )
            .child(
                div()
                    .h(crate::ui::constants::drag_handle_height())
                    .w_full()
                    .flex()
                    .bg(bg_color)
                    .items_center()
                    .cursor_n_resize()
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(|this, evt: &gpui::MouseDownEvent, window, cx| {
                            let y = evt.position.y;
                            this.begin_resize(window, cx, y);
                        }),
                    )
                    .child(
                        div()
                            .h(crate::ui::constants::hairline())
                            .w_full()
                            .bg(border_color),
                    ),
            )
            .child(
                div()
                    .h(self.current_input_height)
                    .w_full()
                    .bg(bg_color)
                    .child(self.render_input_area(cx)),
            )
    }
}
