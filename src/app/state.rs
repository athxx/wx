use crate::components::{ChatArea, SessionList, ToolBar};
use crate::data::sample_data;
use crate::models::{ChatSession, Contact, Message};
use crate::theme::Theme;
use gpui::{App, AppContext, Context, Entity, Window};
use gpui_component::resizable::ResizableState;

/// 主应用状态
pub struct WeixinApp {
    // 主要组件
    pub toolbar: Entity<ToolBar>,
    pub session_list: Entity<SessionList>,
    pub chat_area: Entity<ChatArea>,

    // 数据状态
    pub contacts: Vec<Contact>,
    pub current_session: Option<ChatSession>,

    // UI 状态
    pub session_resizable_state: Entity<ResizableState>,

    // 订阅者
    pub(crate) _theme_observer: Option<gpui::Subscription>,
}

impl WeixinApp {
    /// 创建新的应用实例
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // 创建示例联系人数据
        let contacts = sample_data::create_sample_contacts();

        // 创建各个视图组件
        let toolbar = ToolBar::view(window, cx);
        let session_list = SessionList::view(window, cx);
        let chat_area = ChatArea::view(window, cx);

        // 设置联系人列表
        session_list.update(cx, |list, cx| {
            list.set_contacts(contacts.clone(), cx);
        });

        // 创建可调整大小状态
        let session_resizable_state = ResizableState::new(cx);

        // 设置事件监听器
        Self::setup_event_subscriptions(&toolbar, &session_list, cx);

        // 订阅全局主题变化
        let theme_observer = cx.observe_global::<Theme>(|_this, cx| {
            cx.notify();
        });

        Self {
            toolbar,
            session_list,
            chat_area,
            contacts,
            current_session: None,
            session_resizable_state,
            _theme_observer: Some(theme_observer),
        }
    }

    /// 创建视图实体
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    /// 处理会话选择
    pub fn on_session_selected(&mut self, contact_id: &str, cx: &mut Context<Self>) {
        if let Some(contact) = self.contacts.iter().find(|c| c.id == contact_id).cloned() {
            // 创建或获取聊天会话
            let mut session = ChatSession::new(contact.clone());

            // 添加示例消息
            session.messages = sample_data::create_sample_messages(&contact);

            self.current_session = Some(session.clone());

            // 更新聊天区域
            self.chat_area.update(cx, |area, cx| {
                area.set_session(Some(session), cx);
            });
        }
    }

    /// 处理发送消息
    pub fn on_send_message(&mut self, content: String, cx: &mut Context<Self>) {
        if let Some(session) = &mut self.current_session {
            // 创建新消息
            let message = Message::new(
                format!("msg-{}", chrono::Utc::now().timestamp_millis()),
                "self",
                "我",
                content.clone(),
                true,
            );

            // 添加到当前会话
            session.add_message(message.clone());

            // 更新聊天区域
            self.chat_area.update(cx, |area, cx| {
                area.add_message(message, cx);
            });

            // 更新会话列表中的最后一条消息
            self.session_list.update(cx, |list, cx| {
                list.update_contact_last_message(&session.contact.id, content, cx);
            });

            cx.notify();
        }
    }

    /// 获取当前聊天标题
    pub fn get_current_chat_title(&self) -> String {
        self.current_session
            .as_ref()
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
            .unwrap_or_else(|| "选择一个会话".to_string())
    }
}
