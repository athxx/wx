use crate::components::{ChatArea, ChatAreaEvent, SessionList, ToolBar};
use crate::infra::memory_repos::{MemoryContactsRepo, MemorySessionsRepo};
use crate::models::{ChatSession, Contact, Message};
use crate::ui::theme::Theme;
use gpui::{px, App, AppContext, Context, Entity, Window};
use crate::ui::fixed_resizable::{FixedResizableEvent, FixedResizableState};
use crate::app::actions::{SelectSession, ToolbarClicked};
use serde::{Deserialize, Serialize};

/// 持久化的布局状态，目前记录左侧会话区域宽度和聊天输入框高度。
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LayoutState {
    session_left_width: f32,
    /// 聊天输入框高度（Pixels -> f32），为兼容旧版本，使用可选字段。
    #[serde(default)]
    chat_input_height: Option<f32>,
}

#[cfg(debug_assertions)]
const LAYOUT_FILE: &str = "target/weixin_layout.json";
#[cfg(not(debug_assertions))]
const LAYOUT_FILE: &str = "weixin_layout.json";

/// 纯领域层的聊天状态，不依赖 UI 组件。
struct ChatState {
    sessions_repo: MemorySessionsRepo,
    contacts: Vec<Contact>,
    current_session: Option<ChatSession>,
}

impl ChatState {
    fn new() -> Self {
        let contacts_repo = MemoryContactsRepo::new();
        let sessions_repo = MemorySessionsRepo::new();
        let contacts = contacts_repo.get_all();

        Self {
            sessions_repo,
            contacts,
            current_session: None,
        }
    }

    fn contacts(&self) -> &Vec<Contact> {
        &self.contacts
    }

    fn select_session(&mut self, contact_id: &str) -> Option<ChatSession> {
        if let Some(contact) = self.contacts.iter().find(|c| c.id == contact_id).cloned() {
            let mut session = ChatSession::new(contact.clone());

            session.messages = self.sessions_repo.get_messages(&contact);

            self.current_session = Some(session.clone());

            Some(session)
        } else {
            None
        }
    }

    fn send_message(&mut self, content: String) -> Option<(ChatSession, Message)> {
        if let Some(session) = &mut self.current_session {
            let message = Message::new(
                format!("msg-{}", chrono::Utc::now().timestamp_millis()),
                "self",
                "我",
                content.clone(),
                true,
            );

            session.add_message(message.clone());

            Some((session.clone(), message))
        } else {
            None
        }
    }

    fn current_session(&self) -> Option<&ChatSession> {
        self.current_session.as_ref()
    }

    fn clear_session(&mut self) {
        self.current_session = None;
    }
}

pub struct WeixinApp {
    pub toolbar: Entity<ToolBar>,
    pub session_list: Entity<SessionList>,
    pub chat_area: Entity<ChatArea>,

    /// 聊天领域状态（会话列表、当前会话等）。
    chat_state: ChatState,

    /// 固定左侧宽度的 resizable 状态（用于顶部搜索栏 + 左侧会话列表）
    pub session_split_state: Entity<FixedResizableState>,

    pub(crate) _theme_observer: Option<gpui::Subscription>,
}

impl WeixinApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let chat_state = ChatState::new();

        let toolbar = ToolBar::view(window, cx);
        let session_list = SessionList::view(window, cx);
        let chat_area = ChatArea::view(window, cx);

        // 固定分隔状态，左侧宽度用绝对像素表示
        let session_split_state = FixedResizableState::new(cx);

        // 初始化会话列表联系人数据
        session_list.update(cx, |list, cx| {
            list.set_contacts(chat_state.contacts().clone(), cx);
        });

        // 尝试从本地文件加载布局（左侧宽度 + 输入框高度）
        Self::load_layout(&session_split_state, &chat_area, cx);

        // 监听分隔状态变更，在松开鼠标时持久化布局
        let session_split_state_for_save = session_split_state.clone();
        cx.subscribe(&session_split_state, move |this, _state, ev: &FixedResizableEvent, cx| {
            match ev {
                FixedResizableEvent::Resized => this.save_layout(&session_split_state_for_save, cx),
            }
        })
        .detach();

        // 监听聊天输入框高度变更，结束拖动时持久化布局
        let session_split_state_for_save2 = session_split_state.clone();
        let chat_area_for_save = chat_area.clone();
        cx.subscribe(&chat_area_for_save, move |this, _state, ev: &ChatAreaEvent, cx| {
            if let ChatAreaEvent::InputResized = ev {
                this.save_layout(&session_split_state_for_save2, cx);
            }
        })
        .detach();

        let theme_observer = cx.observe_global::<Theme>(|_this, cx| {
            cx.notify();
        });

        Self {
            toolbar,
            session_list,
            chat_area,
            chat_state,
            session_split_state,
            _theme_observer: Some(theme_observer),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn load_layout(
        session_split_state: &Entity<FixedResizableState>,
        chat_area: &Entity<ChatArea>,
        cx: &mut Context<Self>,
    ) {
        if let Ok(json) = std::fs::read_to_string(LAYOUT_FILE) {
            if let Ok(state) = serde_json::from_str::<LayoutState>(&json) {
                session_split_state.update(cx, |s, _| {
                    s.left_width = px(state.session_left_width);
                    s.drag_start_width = s.left_width;
                });

                if let Some(h) = state.chat_input_height {
                    let height = px(h);
                    chat_area.update(cx, |area, cx_chat| {
                        area.set_input_height(height, cx_chat);
                    });
                }
            }
        }
    }

    fn save_layout(&self, session_split_state: &Entity<FixedResizableState>, cx: &mut Context<Self>) {
        let left_width = session_split_state.read(cx).left_width;
        // 将 Pixels 转为标量宽度，依赖于 gpui 对 Pixels 的 Into<f32> 实现。
        let width: f32 = left_width.into();

        // 当前输入框高度也一并持久化。
        let chat_input_height: f32 = {
            let h = self.chat_area.read(cx).input_height();
            h.into()
        };

        let layout = LayoutState {
            session_left_width: width,
            chat_input_height: Some(chat_input_height),
        };

        if let Ok(json) = serde_json::to_string_pretty(&layout) {
            let _ = std::fs::write(LAYOUT_FILE, json);
        }
    }

    pub fn on_session_selected(&mut self, contact_id: &str, cx: &mut Context<Self>) {
        if let Some(session) = self.chat_state.select_session(contact_id) {
            self.chat_area.update(cx, |area, cx| {
                area.set_session(Some(session), cx);
            });
        }
    }

    pub fn on_send_message(&mut self, content: String, cx: &mut Context<Self>) {
        if let Some((session, message)) = self.chat_state.send_message(content.clone()) {
            self.chat_area.update(cx, |area, cx| {
                area.add_message(message, cx);
            });

            self.session_list.update(cx, |list, cx| {
                list.update_contact_last_message(&session.contact.id, content, cx);
            });

            cx.notify();
        }
    }

    pub fn get_current_chat_title(&self) -> String {
        self.chat_state
            .current_session()
            .map(|s| {
                if s.contact.is_group {
                    if let Some(count) = s.contact.member_count {
                        format!("{} ~ ({})", s.contact.name, count)
                    } else {
                        s.contact.name.clone()
                    }
                } else {
                    s.contact.name.clone()
                }
            })
            // 如果未选择会话，则不显示任何标题文本
            .unwrap_or_else(String::new)
    }

    /// Action: 选择会话，由根视图统一处理。
    /// 如果再次点击当前会话，则视为取消选择，恢复到欢迎界面。
    pub fn on_action_select_session(
        &mut self,
        action: &SelectSession,
        cx: &mut Context<Self>,
    ) {
        let is_same_as_current = self
            .chat_state
            .current_session()
            .map(|s| s.contact.id == action.contact_id)
            .unwrap_or(false);

        if is_same_as_current {
            // 取消选择当前会话：清空 ChatState，并让 ChatArea 显示欢迎页。
            self.chat_state.clear_session();
            self.chat_area.update(cx, |area, cx| {
                area.set_session(None, cx);
            });
        } else {
            // 选择新的会话
            self.on_session_selected(&action.contact_id, cx);
        }
    }

    /// Action: 工具栏点击，目前先简单打印，后续可以根据 item 做不同操作。
    pub fn on_action_toolbar_clicked(
        &mut self,
        action: &ToolbarClicked,
        _cx: &mut Context<Self>,
    ) {
        println!("Toolbar item clicked: {:?}", action.item);
    }
}
