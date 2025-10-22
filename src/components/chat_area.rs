use gpui::{
    div, prelude::FluentBuilder, px, relative, rgb, AnyElement, App, AppContext, Context, Entity,
    Hsla, InteractiveElement, IntoElement, ParentElement, Pixels, Render,
    StatefulInteractiveElement, Styled, Window,
};
use gpui_component::{
    avatar::Avatar,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    h_flex,
    highlighter::Language,
    input::{InputState, TabSize, TextInput},
    v_flex, ActiveTheme, Icon, IconName, Sizable,
};

use crate::models::{ChatSession, Message};
use crate::theme::{Theme, WeixinThemeColors};

pub struct ChatArea {
    current_session: Option<ChatSession>,
    input_state: Entity<InputState>,
    on_send_message: Option<Box<dyn Fn(String) + 'static>>,
    // Zed-like manual sizing (no ResizableState coupling)
    current_input_height: Pixels,
    default_input_height: Pixels,
    min_input_height: Pixels,
    max_input_height: Pixels,
    is_resizing: bool,
    drag_start_y: Pixels,
    drag_start_height: Pixels,
}

impl ChatArea {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let default_height = px(200.);

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
            default_input_height: default_height,
            min_input_height: px(120.),
            max_input_height: px(420.),
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

    fn render_message(&self, message: &Message, cx: &mut Context<Self>) -> impl IntoElement {
        let is_self = message.is_self;
        let time_str = message.timestamp.format("%H:%M").to_string();
        let is_group = self
            .current_session
            .as_ref()
            .map(|s| s.contact.is_group)
            .unwrap_or(false);
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);

        div().w_full().px_5().py_2().child(
            div()
                .flex()
                .w_full()
                .when(is_self, |this| this.flex_row_reverse())
                .gap_3()
                .child(Avatar::new().with_size(px(35.)).rounded(px(5.)))
                .child(
                    v_flex()
                        .gap_1p5()
                        .max_w(px(480.))
                        .when(is_self, |this| this.items_end())
                        .child(
                            // 时间戳和发送者名称（群组中显示）
                            h_flex()
                                .gap_2()
                                .when(is_self, |this| this.flex_row_reverse())
                                .when(is_group && !is_self, |this| {
                                    this.child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.muted_foreground)
                                            .font_weight(gpui::FontWeight::MEDIUM)
                                            .child(message.sender_name.clone()),
                                    )
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.muted_foreground)
                                        .child(time_str),
                                ),
                        )
                        .child(
                            // 消息气泡
                            div().relative().child(
                                div()
                                    .px_3()
                                    .py_2()
                                    .rounded(px(4.))
                                    .bg(if is_self {
                                        weixin_colors.message_bubble_self
                                    } else {
                                        weixin_colors.message_bubble_other
                                    })
                                    .text_color(if is_self {
                                        weixin_colors.message_text_self
                                    } else {
                                        weixin_colors.message_text_other
                                    })
                                    .text_base()
                                    .line_height(relative(1.6))
                                    .child(message.content.clone()),
                            ),
                        ),
                ),
        )
    }

    fn render_input_area(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.theme();

        v_flex()
            .size_full()
            .child(
                // 工具栏
                div().w_full().px_3().py_1p5().child(
                    h_flex()
                        .w_full()
                        .items_center()
                        .child(
                            // 左侧图标组
                            h_flex()
                                .gap_2()
                                .child(
                                    div()
                                        .p(px(6.))
                                        .rounded(px(4.))
                                        .cursor_pointer()
                                        .hover(|this| this.bg(theme.secondary))
                                        .child(
                                            Icon::default()
                                                .path("emoji.svg")
                                                .w(px(20.))
                                                .h(px(20.))
                                                .text_color(theme.muted_foreground),
                                        ),
                                )
                                .child(
                                    div()
                                        .p(px(6.))
                                        .rounded(px(4.))
                                        .cursor_pointer()
                                        .hover(|this| this.bg(theme.secondary))
                                        .child(
                                            Icon::default()
                                                .path("favorite.svg")
                                                .w(px(20.))
                                                .h(px(20.))
                                                .text_color(theme.muted_foreground),
                                        ),
                                )
                                .child(
                                    div()
                                        .p(px(6.))
                                        .rounded(px(4.))
                                        .cursor_pointer()
                                        .hover(|this| this.bg(theme.secondary))
                                        .child(
                                            Icon::default()
                                                .path("file.svg")
                                                .w(px(20.))
                                                .h(px(20.))
                                                .text_color(theme.muted_foreground),
                                        ),
                                )
                                .child(
                                    div()
                                        .p(px(6.))
                                        .rounded(px(4.))
                                        .cursor_pointer()
                                        .hover(|this| this.bg(theme.secondary))
                                        .child(
                                            Icon::default()
                                                .path("scissors.svg")
                                                .w(px(20.))
                                                .h(px(20.))
                                                .text_color(theme.muted_foreground),
                                        ),
                                )
                                .child(
                                    h_flex()
                                        .p(px(6.))
                                        .rounded(px(4.))
                                        .justify_center()
                                        .items_center()
                                        .cursor_pointer()
                                        .w(px(15.))
                                        .hover(|this| this.bg(theme.secondary))
                                        .child(
                                            Icon::default()
                                                .path("down.svg")
                                                .w(px(20.)) // 宽度为其他图标的一半
                                                .h(px(20.))
                                                .text_color(theme.muted_foreground),
                                        ),
                                ),
                        )
                        .child(
                            // 中间空白区域
                            div().flex_1(),
                        )
                        .child(
                            // 右侧图标组
                            h_flex()
                                .gap_2()
                                .child(
                                    div()
                                        .p(px(6.))
                                        .rounded(px(4.))
                                        .cursor_pointer()
                                        .hover(|this| this.bg(theme.secondary))
                                        .child(
                                            Icon::default()
                                                .path("circle.svg")
                                                .w(px(20.))
                                                .h(px(20.))
                                                .text_color(theme.muted_foreground),
                                        ),
                                )
                                .child(
                                    div()
                                        .p(px(6.))
                                        .rounded(px(4.))
                                        .cursor_pointer()
                                        .hover(|this| this.bg(theme.secondary))
                                        .child(
                                            Icon::default()
                                                .path("video-call.svg")
                                                .w(px(20.))
                                                .h(px(20.))
                                                .text_color(theme.muted_foreground),
                                        ),
                                ),
                        ),
                ),
            )
            .child(
                // 输入框容器 - 占据剩余空间，内部自动滚动
                div().flex_1().w_full().px_2().overflow_hidden().child(
                    TextInput::new(&self.input_state)
                        .appearance(false)
                        .w_full()
                        .h_full(),
                ),
            )
            .child(
                // 发送按钮 - 固定在底部右侧
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

    fn update_resize(&mut self, window: &mut Window, cx: &mut Context<Self>, current_y: Pixels) {
        if !self.is_resizing {
            return;
        }
        let dy = current_y - self.drag_start_y;
        let mut new_h = self.drag_start_height - dy; // 往下拖动（dy>0）输入区变小，往上拖动（dy<0）输入区变大
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
        let bg_color = weixin_colors.chat_area_bg; // 右侧聊天区域背景 EDEDED

        let messages_view = if let Some(session) = &self.current_session {
            v_flex()
                .w_full()
                .pt_4()
                .pb_2()
                .children(
                    session
                        .messages
                        .iter()
                        .map(|msg| self.render_message(msg, cx)),
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

        // 主聊天区域：顶部消息列表，分隔条，底部固定高输入区
        v_flex()
            .flex_1()
            .bg(bg_color)
            // 捕获全局鼠标事件以在拖拽时更新高度
            .on_mouse_up(
                gpui::MouseButton::Left,
                cx.listener(|this, _evt: &gpui::MouseUpEvent, _window, cx| {
                    this.end_resize(_window, cx);
                }),
            )
            .on_mouse_move(
                cx.listener(|this, evt: &gpui::MouseMoveEvent, _window, cx| {
                    // 使用窗口坐标系的 y 值
                    let y = evt.position.y;
                    this.update_resize(_window, cx, y);
                }),
            )
            .child(
                // 消息区域
                div()
                    .id("chat-messages")
                    .flex_1()
                    .w_full()
                    .bg(bg_color) // 聊天消息区域背景 EDEDED
                    .overflow_y_scroll()
                    .border_t_1()
                    .border_color(border_color)
                    .child(messages_view),
            )
            .child(
                // 分隔条：用于拖拽调整输入区高度（外层加大检测区域）
                div()
                    .h(px(8.)) // 较大的检测区域
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
                        // 内层实际显示的1px分割线
                        div().h(px(1.)).w_full().bg(border_color),
                    ),
            )
            .child(
                // 固定高度的输入区域
                div()
                    .h(self.current_input_height)
                    .w_full()
                    .bg(bg_color) // 输入区域背景 EDEDED
                    .child(self.render_input_area(cx)),
            )
    }
}
