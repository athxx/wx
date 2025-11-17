use gpui::{
    div, prelude::FluentBuilder as _, App, AppContext, Context, Entity, InteractiveElement,
    IntoElement, ParentElement, Render, Styled, Window, WindowControlArea,
};
use gpui_component::{h_flex, v_flex, ActiveTheme, Icon};
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::infra::memory_repos::{MemoryContactsRepo, MemorySessionsRepo};
use crate::models::ChatSession;
use crate::ui::theme::Theme;

use super::ChatArea;

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

    fn new(window: &mut Window, cx: &mut Context<Self>, contact_id: String) -> Self {
        let contacts_repo = MemoryContactsRepo::new();
        let sessions_repo = MemorySessionsRepo::new();
        let mut chat_title = String::new();

        let chat_area = ChatArea::view(window, cx);

        // 根据 contact_id 构造一个会话并挂到 ChatArea 上。
        if let Some(contact) = contacts_repo
            .get_all()
            .into_iter()
            .find(|c| c.id == contact_id)
        {
            // 构造标题文案，与主窗口保持一致。
            chat_title = if contact.is_group {
                if let Some(count) = contact.member_count {
                    format!("{} ~ ({})", contact.name, count)
                } else {
                    contact.name.clone()
                }
            } else {
                contact.name.clone()
            };

            let mut session = ChatSession::new(contact.clone());
            session.messages = sessions_repo.get_messages(&contact);

            chat_area.update(cx, |area, cx_chat| {
                area.set_session(Some(session), cx_chat);
            });
        }

        Self {
            chat_area,
            chat_title,
            contact_id,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App, contact_id: String) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, contact_id))
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
            .child(crate::ui::widgets::window_controls::window_controls(
                is_maximized,
                &theme,
                false, // 独立聊天窗口右上角不再显示原来的 Pin 按钮
            ))
            .child(crate::ui::widgets::chat_header_actions::chat_header_actions(&theme));

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
