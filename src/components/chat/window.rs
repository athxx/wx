use gpui::{
    div, prelude::FluentBuilder as _, App, AppContext, Context, Entity, InteractiveElement,
    IntoElement, ParentElement, Render, Styled, Window, WindowControlArea,
};
use gpui_component::{h_flex, v_flex, ActiveTheme, Icon};
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::app::state::{ChatStore, ChatStoreEvent};
use crate::infra::memory_repos::{MemoryContactsRepo, MemorySessionsRepo};
use crate::models::ChatSession;
use crate::ui::theme::Theme;

use super::{ChatArea, ChatAreaEvent};
use crate::models::Message;
// 记录当前已打开的聊天窗口，保证同一个会话 ID 只能同时打开一个窗口。
static OPEN_CHAT_WINDOWS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn chat_windows_registry() -> &'static Mutex<HashSet<String>> {
    OPEN_CHAT_WINDOWS.get_or_init(|| Mutex::new(HashSet::new()))
}

/// 独立聊天窗口：当在会话列表中双击会话时弹出。
///
/// 为了简单起见，这里根据 contact_id 重新构造一个 ChatSession，
/// 不与主窗口的 ChatState 进行实时同步。
pub struct ChatWindow {
    chat_area: Entity<ChatArea>,
    chat_title: String,
    contact_id: String,
    store: Entity<ChatStore>,
}

impl ChatWindow {
    /// 尝试为指定会话 ID 预留一个聊天窗口名额。
    /// 如果已经存在，则返回 false，不再打开新窗口。
    pub fn try_reserve(contact_id: &str) -> bool {
        let registry = chat_windows_registry();
        let mut set = registry.lock().unwrap();
        if set.contains(contact_id) {
            false
        } else {
            set.insert(contact_id.to_string());
            true
        }
    }

    fn release(contact_id: &str) {
        if let Some(registry) = OPEN_CHAT_WINDOWS.get() {
            if let Ok(mut set) = registry.lock() {
                set.remove(contact_id);
            }
        }
    }

    fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        store: Entity<ChatStore>,
        contact_id: String,
    ) -> Self {
        let chat_area = ChatArea::view(window, cx);

        // 1. 从 Store 获取会话数据
        let session = store.update(cx, |s, _| s.get_or_load_session(&contact_id));
        let chat_title = session.contact.display_title();

        // 初始化 ChatArea
        chat_area.update(cx, |area, cx| {
            area.set_session(Some(session), cx);
        });

        // 2. 监听 ChatArea 的发送动作 -> 调用 Store
        let store_clone = store.clone();
        let contact_id_clone = contact_id.clone();
        cx.subscribe(
            &chat_area,
            move |_, _, event: &ChatAreaEvent, cx| match event {
                ChatAreaEvent::SendMessage(content) => {
                    store_clone.update(cx, |s, cx| {
                        s.send_message(contact_id_clone.clone(), content.clone(), cx);
                    });
                }
                _ => {}
            },
        )
        .detach();

        // 3. 监听 Store 的更新 -> 更新 ChatArea
        // 无论是主窗口发的，还是本窗口发的，都会通过这个事件回来
        cx.subscribe(&store, |this, _, event: &ChatStoreEvent, cx| {
            match event {
                ChatStoreEvent::NewMessage {
                    contact_id,
                    message,
                } => {
                    // 只有当消息属于当前窗口的联系人时才更新
                    if contact_id == &this.contact_id {
                        this.chat_area.update(cx, |area, cx| {
                            area.add_message(message.clone(), cx);
                        });
                    }
                }
            }
        })
        .detach();

        Self {
            chat_area,
            chat_title,
            contact_id,
            store,
        }
    }

    pub fn view(
        window: &mut Window,
        cx: &mut App,
        store: Entity<ChatStore>,
        contact_id: String,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, store, contact_id))
    }
}

impl Drop for ChatWindow {
    fn drop(&mut self) {
        ChatWindow::release(&self.contact_id);
    }
}

impl Render for ChatWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use crate::ui::constants as UI;

        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);
        let is_maximized = window.is_maximized();
        let title_text = self.chat_title.clone();
        let has_session = !title_text.is_empty();

        let left_header = h_flex()
            .window_control_area(WindowControlArea::Drag)
            .h_full()
            .flex_1()
            .items_center()
            .pl_3()
            .when(has_session, |this| {
                this.child(
                    v_flex()
                        .items_start()
                        .gap_1()
                        .child(
                            div()
                                .p(UI::header_action_padding())
                                .rounded(UI::radius_md())
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.secondary))
                                .child(
                                    Icon::default()
                                        .w(UI::icon_xs())
                                        .h(UI::icon_xs())
                                        .path("nail.svg"),
                                ),
                        )
                        .child(div().text_color(theme.foreground).child(title_text)),
                )
            });

        let right_header = h_flex()
            .h_full()
            .flex_col()
            .items_center()
            .child(
                crate::ui::base::window_controls::WindowControls::new()
                    .maximized(is_maximized)
                    .show_pin(false),
            )
            .child(crate::ui::composites::chat_header_actions::ChatHeaderActions::new());

        v_flex()
            .size_full()
            .child(
                h_flex()
                    .h(UI::title_bar_height())
                    .w_full()
                    .bg(weixin_colors.chat_area_bg)
                    .items_center()
                    .child(left_header)
                    .child(right_header),
            )
            .child(
                // 下面直接复用 ChatArea，保持与主窗口右侧区域一致。
                self.chat_area.clone(),
            )
    }
}
