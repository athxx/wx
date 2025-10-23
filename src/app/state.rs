use crate::components::{ChatArea, SessionList, ToolBar};
use crate::infra::memory_repos::{MemoryContactsRepo, MemorySessionsRepo};
use crate::models::{ChatSession, Contact, Message};
use crate::ui::theme::Theme;
use gpui::{App, AppContext, Context, Entity, Window};
use gpui_component::resizable::ResizableState;

pub struct WeixinApp {
    pub toolbar: Entity<ToolBar>,
    pub session_list: Entity<SessionList>,
    pub chat_area: Entity<ChatArea>,

    sessions_repo: MemorySessionsRepo,

    pub contacts: Vec<Contact>,
    pub current_session: Option<ChatSession>,

    pub session_resizable_state: Entity<ResizableState>,

    pub(crate) _theme_observer: Option<gpui::Subscription>,
}

impl WeixinApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let contacts_repo = MemoryContactsRepo::new();
        let sessions_repo = MemorySessionsRepo::new();

        let contacts = contacts_repo.get_all();

        let toolbar = ToolBar::view(window, cx);
        let session_list = SessionList::view(window, cx);
        let chat_area = ChatArea::view(window, cx);

        session_list.update(cx, |list, cx| {
            list.set_contacts(contacts.clone(), cx);
        });

        let session_resizable_state = ResizableState::new(cx);

        Self::setup_event_subscriptions(&toolbar, &session_list, cx);

        let theme_observer = cx.observe_global::<Theme>(|_this, cx| {
            cx.notify();
        });

        Self {
            toolbar,
            session_list,
            chat_area,
            sessions_repo,
            contacts,
            current_session: None,
            session_resizable_state,
            _theme_observer: Some(theme_observer),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn on_session_selected(&mut self, contact_id: &str, cx: &mut Context<Self>) {
        if let Some(contact) = self.contacts.iter().find(|c| c.id == contact_id).cloned() {
            let mut session = ChatSession::new(contact.clone());

            session.messages = self.sessions_repo.get_messages(&contact);

            self.current_session = Some(session.clone());

            self.chat_area.update(cx, |area, cx| {
                area.set_session(Some(session), cx);
            });
        }
    }

    pub fn on_send_message(&mut self, content: String, cx: &mut Context<Self>) {
        if let Some(session) = &mut self.current_session {
            let message = Message::new(
                format!("msg-{}", chrono::Utc::now().timestamp_millis()),
                "self",
                "我",
                content.clone(),
                true,
            );

            session.add_message(message.clone());

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
