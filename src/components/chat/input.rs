use gpui::{
    div, prelude::FluentBuilder, relative, rgb, AppContext, Context, Entity, EventEmitter,
    IntoElement, ParentElement, Render, Styled, Window,
};
use gpui_component::{
    button::{Button, ButtonVariants},
    h_flex,
    input::{Input, InputState},
    v_flex, ActiveTheme, Disableable,
};

pub struct ChatInput {
    input_state: Entity<InputState>,
}

pub enum ChatInputEvent {
    SendMessage(String),
}

impl EventEmitter<ChatInputEvent> for ChatInput {}

impl ChatInput {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx).auto_grow(1, 1));
        Self { input_state }
    }

    fn send_message(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let content = self.input_state.read(cx).value();
        if content.trim().is_empty() {
            return;
        }

        cx.emit(ChatInputEvent::SendMessage(content.trim().to_string()));

        self.input_state.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
    }
}

impl Render for ChatInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _theme = cx.theme();
        let content = self.input_state.read(cx).value();
        let is_empty = content.trim().is_empty();
        let weixin_colors = crate::ui::theme::Theme::weixin_colors(cx);

        v_flex()
            .size_full()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .px_3()
                    .py_1p5()
                    .child(crate::ui::composites::chat_toolbar::ChatToolbar::new()),
            )
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .px_2()
                    .overflow_hidden()
                    .text_sm()
                    .child(
                        Input::new(&self.input_state)
                            .line_height(relative(1.6))
                            .appearance(false)
                            .size_full(),
                    ),
            )
            .child(
                div().w_full().flex().justify_end().px_4().pb_4().child(
                    Button::new("send")
                        .disabled(is_empty)
                        .primary()
                        .border_color(gpui::transparent_black())
                        .when(is_empty, |this| {
                            this.bg(weixin_colors.send_button_disabled_bg)
                        })
                        .child(
                            h_flex()
                                .text_sm()
                                .items_center()
                                .text_color(if is_empty {
                                    weixin_colors.send_button_disabled_text
                                } else {
                                    rgb(0xFFFFFF).into()
                                })
                                .child("发送(S)"),
                        )
                        .w_24()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.send_message(window, cx);
                        })),
                ),
            )
    }
}
