use gpui::{
    div, rgb, AppContext, Context, Entity, EventEmitter, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Window,
};
use gpui_component::{
    button::{Button, ButtonVariants},
    h_flex,
    input::{Input, InputState},
    v_flex, ActiveTheme,
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

        cx.emit(ChatInputEvent::SendMessage(content.to_string()));

        self.input_state.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
    }
}

impl Render for ChatInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .size_full()
            .child(
                div()
                    .w_full()
                    .px_3()
                    .py_1p5()
                    .child(crate::ui::composites::chat_toolbar::ChatToolbar::new()),
            )
            .child(
                div().flex_1().w_full().px_2().overflow_hidden().child(
                    Input::new(&self.input_state)
                        .text_sm()
                        .appearance(false)
                        .w_full()
                        .h_full(),
                ),
            )
            .child(
                div().w_full().flex().justify_end().px_4().pb_4().child(
                    Button::new("send")
                        .child(
                            h_flex()
                                .text_sm()
                                .items_center()
                                .text_color(rgb(0xFFFFFF))
                                .child("发送(S)"),
                        )
                        .w_24()
                        .primary()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.send_message(window, cx);
                        })),
                ),
            )
    }
}
