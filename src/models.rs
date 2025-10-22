use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub last_message: Option<String>,
    pub last_message_time: Option<DateTime<Local>>,
    pub unread_count: u32,
    // 群组支持
    pub is_group: bool,
    pub member_count: Option<u32>,
    pub avatar_members: Vec<String>, // 用于显示的成员名称（最多4个）
    pub last_sender_name: Option<String>, // 最后发送者名称（用于群组消息预览）
}

impl Contact {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            avatar_url: None,
            last_message: None,
            last_message_time: None,
            unread_count: 0,
            is_group: false,
            member_count: None,
            avatar_members: Vec::new(),
            last_sender_name: None,
        }
    }

    pub fn with_avatar(mut self, url: impl Into<String>) -> Self {
        self.avatar_url = Some(url.into());
        self
    }

    pub fn as_group(mut self, member_count: u32, members: Vec<String>) -> Self {
        self.is_group = true;
        self.member_count = Some(member_count);
        self.avatar_members = members.into_iter().take(4).collect();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub content: String,
    pub timestamp: DateTime<Local>,
    pub is_self: bool,
}

impl Message {
    pub fn new(
        id: impl Into<String>,
        sender_id: impl Into<String>,
        sender_name: impl Into<String>,
        content: impl Into<String>,
        is_self: bool,
    ) -> Self {
        Self {
            id: id.into(),
            sender_id: sender_id.into(),
            sender_name: sender_name.into(),
            content: content.into(),
            timestamp: Local::now(),
            is_self,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatSession {
    pub contact: Contact,
    pub messages: Vec<Message>,
}

impl ChatSession {
    pub fn new(contact: Contact) -> Self {
        Self {
            contact,
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarItem {
    Chat,
    Contacts,
    Favorites,
    Moments,
    Channels,
    Search,
    MiniProgram,
    Menu,
    Phone,
}

impl ToolbarItem {
    pub fn icon_path(&self) -> &'static str {
        match self {
            Self::Chat => "chat-round.svg",
            Self::Contacts => "user-list.svg",
            Self::Favorites => "favorite.svg",
            Self::Moments => "moments.svg",
            Self::Channels => "channels.svg",
            Self::Search => "search.svg",
            Self::MiniProgram => "mini-program.svg",
            Self::Menu => "menu.svg",
            Self::Phone => "phone.svg",
        }
    }

    pub fn icon_path_fill(&self) -> Option<&'static str> {
        match self {
            Self::Chat => Some("chat-round-fill.svg"),
            Self::Contacts => Some("user-list-fill.svg"),
            Self::Favorites => Some("favorite-fill.svg"),
            Self::Moments => None,
            Self::Channels => None,
            Self::Search => None,
            Self::MiniProgram => None,
            Self::Menu => None,
            Self::Phone => None,
        }
    }

    pub fn has_fill(&self) -> bool {
        self.icon_path_fill().is_some()
    }
}
