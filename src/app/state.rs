use std::collections::HashMap;

use gpui::{px, App, AppContext, Context, Entity, EventEmitter, FocusHandle, Focusable, Window};
use gpui_component::input::InputEvent;
// use serde::{Deserialize, Serialize};

use crate::app::actions::{SelectSession, ToolbarClicked};
use crate::app::config::LayoutState;
use crate::components::{ChatArea, ChatAreaEvent, SessionList, ToolBar};
use crate::infra::memory_repos::{MemoryContactsRepo, MemorySessionsRepo};
use crate::models::{ChatSession, Contact, Message};
use crate::ui::fixed_resizable::{FixedResizableEvent, FixedResizableState};
use crate::ui::theme::Theme;

pub enum ChatStoreEvent {
    NewMessage {
        contact_id: String,
        message: Message,
    },
}

/// 纯领域层的聊天状态，不依赖 UI 组件。
pub struct ChatStore {
    sessions_repo: MemorySessionsRepo,
    contacts: Vec<Contact>,
    // 缓存已加载的会话，确保多个窗口操作的是同一个对象
    sessions: HashMap<String, ChatSession>,
}
impl EventEmitter<ChatStoreEvent> for ChatStore {}
impl ChatStore {
    pub fn new() -> Self {
        let contacts_repo = MemoryContactsRepo::new();
        let sessions_repo = MemorySessionsRepo::new();
        let contacts = contacts_repo.get_all();

        Self {
            sessions_repo,
            contacts,
            sessions: HashMap::new(),
        }
    }

    pub fn contacts(&self) -> &Vec<Contact> {
        &self.contacts
    }

    // 获取或加载会话
    pub fn get_or_load_session(&mut self, contact_id: &str) -> ChatSession {
        if let Some(session) = self.sessions.get(contact_id) {
            return session.clone();
        }

        // 模拟从 Repo 加载
        let contact = self
            .contacts
            .iter()
            .find(|c| c.id == contact_id)
            .cloned()
            .expect("Contact not found");

        let mut session = ChatSession::new(contact.clone());

        session.messages = self
            .sessions_repo
            .get_messages(&contact)
            .into_iter()
            .map(std::rc::Rc::new) // <--- 这里进行包装转换
            .collect();

        self.sessions
            .insert(contact_id.to_string(), session.clone());
        session
    }
    // 发送消息：修改状态并发出事件
    pub fn send_message(&mut self, contact_id: String, content: String, cx: &mut Context<Self>) {
        let mut session = self.get_or_load_session(&contact_id);

        let message = Message::new(
            format!("msg-{}", chrono::Utc::now().timestamp_millis()),
            "self",
            "我",
            content.clone(),
            true,
        );

        // 1. 更新数据
        session.add_message(message.clone());
        self.sessions.insert(contact_id.clone(), session);

        // 2. 更新联系人列表的最后一条消息预览
        if let Some(contact) = self.contacts.iter_mut().find(|c| c.id == contact_id) {
            contact.last_message = Some(content);
            contact.last_message_time = Some(chrono::Local::now());
        }

        // 3. 通知订阅者
        cx.notify(); // 通知使用了 cx.observe 的视图
        cx.emit(ChatStoreEvent::NewMessage {
            contact_id,
            message,
        }); // 发送具体事件
    }
}
// 1. 定义一个全局结构体，用于持有主窗口的 Entity

pub struct WeixinApp {
    pub toolbar: Entity<ToolBar>,
    pub session_list: Entity<SessionList>,
    pub chat_area: Entity<ChatArea>,

    /// 聊天领域状态（会话列表、当前会话等）。
    pub store: Entity<ChatStore>,

    /// 固定左侧宽度的 resizable 状态（用于顶部搜索栏 + 左侧会话列表）
    pub session_split_state: Entity<FixedResizableState>,

    pub focus_handle: FocusHandle,

    pub(crate) _theme_observer: Option<gpui::Subscription>,
}

impl Focusable for WeixinApp {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl WeixinApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, store: Entity<ChatStore>) -> Self {
        // 1. 初始化子视图组件
        let toolbar = ToolBar::view(window, cx);
        let session_list = SessionList::view(window, cx);
        let chat_area = ChatArea::view(window, cx);

        // 2. 初始化布局状态
        let session_split_state = FixedResizableState::new(cx);
        let focus_handle = cx.focus_handle();

        // 3. [核心修改] 从 Store 初始化数据到会话列表
        //    直接读取 store 中的联系人数据，不再使用本地的 chat_state
        let contacts = store.read(cx).contacts().clone();
        session_list.update(cx, |list, cx| {
            list.set_contacts(contacts, cx);
        });

        // 4. [核心修改] 订阅 Store 的全局事件 (实现多窗口同步)
        cx.subscribe(&store, |this, _, event: &ChatStoreEvent, cx| {
            match event {
                ChatStoreEvent::NewMessage {
                    contact_id,
                    message,
                } => {
                    // A. 更新左侧列表的“最后一条消息”预览
                    this.session_list.update(cx, |list, cx| {
                        list.update_contact_last_message(contact_id, message.content.clone(), cx);
                    });

                    // B. 如果右侧聊天区域正好打开的是这个会话，则添加消息气泡
                    this.chat_area.update(cx, |area, cx| {
                        area.handle_new_message(contact_id, message.clone(), cx);
                    });
                }
            }
        })
        .detach();

        // 5. 监听搜索框焦点事件 (保持原有逻辑)
        let search_input = session_list.read(cx).search_input.clone();
        cx.subscribe(&search_input, |_, _, event: &InputEvent, cx| match event {
            InputEvent::Focus | InputEvent::Blur => cx.notify(),
            _ => {}
        })
        .detach();

        // 6. 加载持久化的布局配置 (保持原有逻辑)
        Self::load_layout(&session_split_state, &chat_area, cx);

        // 7. 监听分割线拖拽事件 (保持原有逻辑)
        let session_split_state_for_save = session_split_state.clone();
        cx.subscribe(
            &session_split_state,
            move |this, _state, ev: &FixedResizableEvent, cx| match ev {
                FixedResizableEvent::Resized => this.save_layout(&session_split_state_for_save, cx),
            },
        )
        .detach();

        // 8. 监听聊天区域事件 (输入框调整高度 + 发送消息)
        let session_split_state_for_save2 = session_split_state.clone();
        let chat_area_for_save = chat_area.clone();
        cx.subscribe(
            &chat_area_for_save,
            move |this, _state, ev: &ChatAreaEvent, cx| match ev {
                ChatAreaEvent::InputResized => {
                    this.save_layout(&session_split_state_for_save2, cx);
                }
                ChatAreaEvent::SendMessage(content) => {
                    // 调用本结构体的 on_send_message 方法
                    this.on_send_message(content.clone(), cx);
                }
            },
        )
        .detach();

        // 9. 监听主题变化 (保持原有逻辑)
        let theme_observer = cx.observe_global::<Theme>(|_this, cx| {
            cx.notify();
        });

        // 10. 返回结构体
        Self {
            toolbar,
            session_list,
            chat_area,

            session_split_state,
            focus_handle,
            _theme_observer: Some(theme_observer),
            store, // [新增] 保存共享 Store 的引用
        }
    }

    pub fn view(window: &mut Window, cx: &mut App, store: Entity<ChatStore>) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, store))
    }

    fn load_layout(
        session_split_state: &Entity<FixedResizableState>,
        chat_area: &Entity<ChatArea>,
        cx: &mut Context<Self>,
    ) {
        let state = LayoutState::load_or(LayoutState {
            session_left_width: 200.0, // Default if file not found or parse error
            chat_input_height: None,
            theme_mode: None,
            font_size: None,
        });

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

    fn save_layout(
        &self,
        session_split_state: &Entity<FixedResizableState>,
        cx: &mut Context<Self>,
    ) {
        let left_width = session_split_state.read(cx).left_width;
        // 将 Pixels 转为标量宽度，依赖于 gpui 对 Pixels 的 Into<f32> 实现。
        let width: f32 = left_width.into();

        // 当前输入区域高度一并持久化。
        let chat_input_height: f32 = {
            let h = self.chat_area.read(cx).input_height();
            h.into()
        };

        // 读取已有的状态以保留主题设置，如果不存在则创建默认值。
        let mut state = LayoutState::load_or(LayoutState {
            session_left_width: width,
            chat_input_height: Some(chat_input_height),
            theme_mode: None,
            font_size: None,
        });

        state.session_left_width = width;
        state.chat_input_height = Some(chat_input_height);

        state.save();
    }

    pub fn on_session_selected(&mut self, contact_id: &str, cx: &mut Context<Self>) {
        // [修改] 从 Store 获取会话
        let session = self
            .store
            .update(cx, |store, _| store.get_or_load_session(contact_id));

        self.chat_area.update(cx, |area, cx| {
            area.set_session(Some(session), cx);
        });
    }

    pub fn on_send_message(&mut self, content: String, cx: &mut Context<Self>) {
        // [修改] 这里改为调用 current_session() 方法
        let current_contact_id = self
            .chat_area
            .read(cx)
            .current_session() // <--- 加上括号
            .map(|s| s.contact.id.clone());

        if let Some(contact_id) = current_contact_id {
            self.store.update(cx, |store, cx| {
                store.send_message(contact_id, content, cx);
            });
        }
    }

    pub fn get_current_chat_title(&self, cx: &App) -> String {
        // [修改] 这里改为调用 current_session() 方法
        self.chat_area
            .read(cx)
            .current_session() // <--- 加上括号
            .map(|s| s.contact.display_title())
            .unwrap_or_else(String::new)
    }

    pub fn on_action_select_session(&mut self, action: &SelectSession, cx: &mut Context<Self>) {
        // [修改] 这里改为调用 current_session() 方法
        let current_contact_id = self
            .chat_area
            .read(cx)
            .current_session() // <--- 加上括号
            .map(|s| s.contact.id.clone());

        let is_same = current_contact_id.as_deref() == Some(&action.contact_id);

        if is_same {
            self.chat_area
                .update(cx, |area, cx| area.set_session(None, cx));
        } else {
            self.on_session_selected(&action.contact_id, cx);
        }
    }

    /// Action: 工具栏点击，目前先简单打印，后续可以根据 item 做不同操作。
    pub fn on_action_toolbar_clicked(&mut self, action: &ToolbarClicked, _cx: &mut Context<Self>) {
        println!("Toolbar item clicked: {:?}", action.item);
    }
}
