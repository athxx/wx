use gpui::{
    div, px, relative, size, App, AppContext, AvailableSpace, Context, Entity, EventEmitter,
    InteractiveElement, IntoElement, ParentElement, Pixels, Render, Styled, Window,
    WindowControlArea,
};
use gpui_component::{
    h_flex,
    scroll::{Scrollbar, ScrollbarAxis, ScrollbarState},
    v_flex, v_virtual_list, ActiveTheme, Icon, VirtualListScrollHandle,
};
use std::rc::Rc;

use crate::models::{ChatSession, Message};
use crate::ui::theme::Theme;
use crate::{
    components::chat::input::{ChatInput, ChatInputEvent},
    ui::constants,
};

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
    /// 缓存的消息高度列表，直接传给 virtual_list
    item_sizes: Rc<Vec<gpui::Size<Pixels>>>,
    /// 记录上次计算高度时的窗口宽度，用于判断是否需要重新计算折行
    last_layout_width: Option<Pixels>,
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
            item_sizes: Rc::new(Vec::new()),
            last_layout_width: None,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
    pub fn current_session(&self) -> Option<&ChatSession> {
        self.current_session.as_ref()
    }
    pub fn set_session(&mut self, session: Option<ChatSession>, cx: &mut Context<Self>) {
        self.current_session = session;
        self.item_sizes = Rc::new(Vec::new());
        if let Some(session) = &self.current_session {
            self.scroll_handle.scroll_to_item(
                session.messages.len().saturating_sub(1),
                gpui::ScrollStrategy::Top,
            );
        }
        cx.notify();
    }

    pub fn add_message(&mut self, message: Message, cx: &mut Context<Self>) {
        if let Some(session) = &mut self.current_session {
            session.add_message(message);
            self.scroll_handle.scroll_to_item(
                session.messages.len().saturating_sub(1),
                gpui::ScrollStrategy::Top,
            );
            cx.notify();
        }
    }
    /// 测量单条消息的高度
    fn measure_message(
        &self,
        message: &Rc<Message>,
        width: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::Size<Pixels> {
        // 1. 获取与 MessageBubble 一致的布局常量
        // 这些数值必须与 MessageBubble.rs 中的完全一致，否则高度计算会偏差
        let avatar_size = crate::ui::constants::avatar_small(); // w/h
        let bubble_max_width = crate::ui::constants::bubble_max_width();

        // 模拟 Avatar 占位
        let avatar_placeholder = div().w(avatar_size).h(avatar_size);

        let header_placeholder = div().h(px(16.0)).w_full();

        // 模拟消息气泡内容
        // 必须包含真实的文本内容，因为换行是高度变化的核心来源
        let content_proxy = div()
            .max_w(bubble_max_width)
            .whitespace_normal() // 关键：允许文本换行
            .text_sm() // 关键：字体大小必须一致
            .line_height(relative(1.4)) // 关键：行高必须一致
            .child(message.content.clone());

        let bubble_inner_padding = div()
            .px(px(12.)) // px_3
            .py(px(8.)) // py_2
            .child(content_proxy);

        // 组装整体结构
        let layout_proxy = div()
            .w_full()
            .px(px(20.)) // px_5
            .py(px(8.)) // py_2
            .child(
                div()
                    .flex()
                    .gap(px(12.)) // gap_3
                    // 注意：flex_row_reverse 改变的是位置，不改变高度计算，
                    // 所以测量时可以统一用默认方向，只要宽度约束正确即可。
                    .child(avatar_placeholder)
                    .child(
                        gpui_component::v_flex()
                            .gap(px(6.)) // gap_1p5
                            .child(header_placeholder)
                            .child(bubble_inner_padding),
                    ),
            );

        // 3. 执行测量
        let mut element = layout_proxy.into_any_element();

        // 给予确定的宽度，让 GPUI 计算内容所需的高度
        let available_space = size(AvailableSpace::Definite(width), AvailableSpace::MinContent);

        element.layout_as_root(available_space, window, cx)
    }

    /// 重新计算所有消息的高度（用于切换会话或窗口宽度改变）
    fn remeasure_all_messages(
        &mut self,
        width: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(session) = &self.current_session {
            let mut sizes = Vec::with_capacity(session.messages.len());

            // 这里虽然是循环，但只在特定时机触发，而不是每帧触发
            for msg in &session.messages {
                let size = self.measure_message(msg, width, window, cx);
                sizes.push(size);
            }

            self.item_sizes = Rc::new(sizes);
            self.last_layout_width = Some(width);
        } else {
            self.item_sizes = Rc::new(Vec::new());
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

        let window_width = window.viewport_size().width;
        let mut needs_remeasure = false;
        if let Some(last_w) = self.last_layout_width {
            if (last_w - window_width).abs() > px(1.0) {
                needs_remeasure = true;
            }
        } else {
            // 第一次渲染，必须计算
            needs_remeasure = true;
        }
        let msg_count = self
            .current_session
            .as_ref()
            .map(|s| s.messages.len())
            .unwrap_or(0);
        let cache_count = self.item_sizes.len();

        if needs_remeasure {
            // 情况A：宽度变了，全部重算
            self.remeasure_all_messages(window_width, window, cx);
        } else if msg_count > cache_count {
            // 情况B：宽度没变，只有新消息 -> 增量计算
            if let Some(session) = &self.current_session {
                // 复制现有的尺寸列表
                let mut new_sizes = (*self.item_sizes).clone();
                // 只计算新增的部分
                for i in cache_count..msg_count {
                    let msg = &session.messages[i];
                    let size = self.measure_message(msg, window_width, window, cx);
                    new_sizes.push(size);
                }
                self.item_sizes = Rc::new(new_sizes);
            }
        }
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
            // 这里不再进行 map layout，而是直接使用 self.item_sizes
            v_virtual_list(
                cx.entity().clone(),
                "chat-messages",
                self.item_sizes.clone(),
                move |view, visible_range, _window, cx| {
                    let Some(session) = &view.current_session else {
                        return Vec::new();
                    };
                    let is_group = session.contact.is_group;

                    visible_range
                        .map(|ix| {
                            // [核心优化] session.messages[ix] 是 Rc<Message>
                            // clone() 极其廉价
                            let bubble = crate::ui::composites::message_bubble::MessageBubble::new(
                                session.messages[ix].clone(),
                            )
                            .group(is_group);

                            div().w_full().child(bubble).into_any_element()
                        })
                        .collect()
                },
            )
            .pb_2()
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
