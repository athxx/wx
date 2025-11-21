use gpui::{
    div, px, size, App, AppContext, AvailableSpace, Context, Entity, EventEmitter,
    InteractiveElement, IntoElement, ParentElement, Pixels, Render, Styled, Window,
    WindowControlArea,
};
use gpui_component::{
    h_flex,
    scroll::{Scrollbar, ScrollbarAxis, ScrollbarState},
    v_flex, v_virtual_list, ActiveTheme, Icon, VirtualListScrollHandle,
};
use std::rc::Rc;

use crate::components::chat::input::{ChatInput, ChatInputEvent};
use crate::models::{ChatSession, Message};
use crate::ui::theme::Theme;

pub struct ChatArea {
    current_session: Option<ChatSession>,
    chat_input: Entity<ChatInput>,
    /// 当前输入区域高度（下方输入框整体区域）。
    current_input_height: Pixels,
    /// 输入区域最小/最大高度，用于约束拖动。
    min_input_height: Pixels,
    max_input_height: Pixels,
    is_resizing: bool,
    drag_start_y: Pixels,
    drag_start_height: Pixels,
    /// 聊天消息虚拟列表的滚动句柄和滚动条状态。
    scroll_handle: VirtualListScrollHandle,
    scroll_state: ScrollbarState,
}

/// ChatArea 对外发送的事件（例如输入框高度调整完成）。
#[derive(Clone, Debug)]
pub enum ChatAreaEvent {
    InputResized,
    SendMessage(String),
}

impl EventEmitter<ChatAreaEvent> for ChatArea {}

impl ChatArea {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let default_input_height = crate::ui::constants::chat_input_default_height();

        let chat_input = cx.new(|cx| ChatInput::new(window, cx));

        cx.subscribe(&chat_input, |_, _, ev: &ChatInputEvent, cx| match ev {
            ChatInputEvent::SendMessage(content) => {
                cx.emit(ChatAreaEvent::SendMessage(content.clone()));
            }
        })
        .detach();

        Self {
            current_session: None,
            chat_input,
            current_input_height: default_input_height,
            min_input_height: crate::ui::constants::chat_input_min_height(),
            max_input_height: crate::ui::constants::chat_input_max_height(),
            is_resizing: false,
            drag_start_y: px(0.),
            drag_start_height: default_input_height,
            scroll_handle: VirtualListScrollHandle::new(),
            scroll_state: ScrollbarState::default(),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn set_session(&mut self, session: Option<ChatSession>, cx: &mut Context<Self>) {
        self.current_session = session;
        cx.notify();
    }

    pub fn add_message(&mut self, message: Message, cx: &mut Context<Self>) {
        if let Some(session) = &mut self.current_session {
            session.add_message(message);
            cx.notify();
        }
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
        // 拖动时调整下方输入区域高度，消息区域使用剩余空间。
        // 注意：Y 轴向下为正，所以这里需要反向计算，才能做到鼠标往上拖动时输入区域变大。
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

    fn end_resize(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.is_resizing = false;
        // 告知上层聊天区域高度已经调整完成，可用于持久化。
        cx.emit(ChatAreaEvent::InputResized);
    }

    /// 当前输入区域高度，供上层持久化使用。
    pub fn input_height(&self) -> Pixels {
        self.current_input_height
    }

    /// 从持久化状态恢复输入区域高度（会自动按最小/最大高度裁剪）。
    pub fn set_input_height(&mut self, height: Pixels, cx: &mut Context<Self>) {
        let mut h = height;
        if h < self.min_input_height {
            h = self.min_input_height;
        }
        if h > self.max_input_height {
            h = self.max_input_height;
        }
        self.current_input_height = h;
        self.drag_start_height = h;
        cx.notify();
    }
}

impl Render for ChatArea {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let weixin_colors = Theme::weixin_colors(cx);
        let no_session_text_color = theme.muted_foreground;
        let border_color = theme.border;
        let bg_color = weixin_colors.chat_area_bg;

        // 没有选中会话时：右侧只显示居中的微信图标，不显示消息和输入框。
        if self.current_session.is_none() {
            let icon_color = no_session_text_color.opacity(0.35);

            return v_flex().flex_1().size_full().bg(bg_color).child(
                h_flex()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .window_control_area(WindowControlArea::Drag)
                    .child(
                        Icon::default()
                            .path("weixin.svg")
                            .w(px(100.))
                            .h(px(100.))
                            .text_color(icon_color),
                    ),
            );
        }

        // 选中会话时：上面是消息列表（虚拟列表），下面是拖动条 + 输入框。
        let messages_view = {
            let session = self.current_session.as_ref().unwrap();
            let is_group = session.contact.is_group;

            // 仿照 `list.rs` 的做法，提前测量每个消息气泡的真实高度，
            // 然后把测量结果作为 `item_sizes` 交给 VirtualList。
            let available_space = size(
                // 宽度按气泡最大宽度来测量，这样可以正确计算换行后的高度。
                AvailableSpace::Definite(crate::ui::constants::bubble_max_width()),
                AvailableSpace::MinContent,
            );

            let item_sizes: Rc<Vec<gpui::Size<Pixels>>> = Rc::new(
                session
                    .messages
                    .iter()
                    .map(|msg| {
                        let bubble =
                            crate::ui::composites::message_bubble::MessageBubble::new(msg.clone())
                                .group(is_group);
                        let mut el = div().w_full().child(bubble).into_any_element();
                        let bubble_size = el.layout_as_root(available_space, window, cx);
                        // 为了避免底部消息被裁剪，给每一项略微增加 4px 高度作为安全边距。
                        gpui::size(bubble_size.width, bubble_size.height + px(4.))
                    })
                    .collect::<Vec<_>>(),
            );
            v_virtual_list(
                cx.entity().clone(),
                "chat-messages",
                item_sizes,
                move |view, visible_range, _window, cx| {
                    let Some(session) = &view.current_session else {
                        return Vec::new();
                    };
                    let is_group = session.contact.is_group;

                    visible_range
                        .map(|ix| {
                            // 这里直接复用原来的气泡布局，不再手动设置高度，
                            // 高度由 VirtualList 根据预先测量的 item_sizes 控制。
                            crate::ui::composites::message_bubble::MessageBubble::new(
                                session.messages[ix].clone(),
                            )
                            .group(is_group)
                        })
                        .collect()
                },
            )
            .pb_10()
            .track_scroll(&self.scroll_handle)
            .into_any_element()
        };

        v_flex()
            .flex_1()
            .size_full()
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
                    .flex_1()
                    .w_full()
                    .bg(bg_color)
                    .border_t_1()
                    .border_color(border_color)
                    .child(
                        div()
                            .relative()
                            .size_full()
                            .w_full()
                            .bg(bg_color)
                            .child(messages_view)
                            .child(
                                div()
                                    .absolute()
                                    .top_0()
                                    .left_0()
                                    .right_0()
                                    .bottom_0()
                                    .child(
                                        Scrollbar::both(&self.scroll_state, &self.scroll_handle)
                                            .axis(ScrollbarAxis::Vertical),
                                    ),
                            ),
                    ),
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
                            cx.stop_propagation();
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
                    .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .bg(bg_color)
                    .child(self.chat_input.clone()),
            )
    }
}
