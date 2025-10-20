mod assets;
mod components;
mod models;

use assets::Assets;

use chrono::Local;
use components::{ChatArea, CustomTitleBar, SessionList, ToolBar};
use gpui::{
    div, prelude::FluentBuilder, px, App, AppContext, Application, Bounds, Context, Entity,
    InteractiveElement, IntoElement, ParentElement, Render, Size, StatefulInteractiveElement,
    Styled, Window, WindowBounds, WindowControlArea, WindowKind, WindowOptions,
};
use gpui_component::{
    avatar::Avatar,
    h_flex,
    input::TextInput,
    resizable::{h_resizable, resizable_panel, ResizableState},
    v_flex, Icon, IconName, Root, Sizable, TitleBar,
};
use models::{ChatSession, Contact, Message};

use components::session_list::SessionSelectEvent;
use components::toolbar::ToolbarClickEvent;

struct WeixinApp {
    toolbar: Entity<ToolBar>,
    session_list: Entity<SessionList>,
    chat_area: Entity<ChatArea>,
    contacts: Vec<Contact>,
    current_session: Option<ChatSession>,
    session_resizable_state: Entity<ResizableState>, // 会话列表水平拖动
}

impl WeixinApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // 创建示例联系人数据
        let contacts = create_sample_contacts();

        let toolbar = ToolBar::view(window, cx);
        let session_list = SessionList::view(window, cx);
        let chat_area = ChatArea::view(window, cx);

        // 设置联系人列表
        session_list.update(cx, |list, cx| {
            list.set_contacts(contacts.clone(), cx);
        });

        let session_resizable_state = ResizableState::new(cx); // 只用于会话列表

        // 订阅工具栏事件
        cx.subscribe(
            &toolbar,
            |_this, _toolbar, event: &ToolbarClickEvent, _cx| {
                println!("Toolbar item clicked: {:?}", event.item);
            },
        )
        .detach();

        // 订阅会话选择事件
        cx.subscribe(
            &session_list,
            |this, _list, event: &SessionSelectEvent, cx| {
                this.on_session_selected(&event.contact_id, cx);
            },
        )
        .detach();

        Self {
            toolbar,
            session_list,
            chat_area,
            contacts,
            current_session: None,
            session_resizable_state,
        }
    }

    fn on_session_selected(&mut self, contact_id: &str, cx: &mut Context<Self>) {
        // 找到对应的联系人
        if let Some(contact) = self.contacts.iter().find(|c| c.id == contact_id).cloned() {
            // 创建或获取聊天会话
            let mut session = ChatSession::new(contact.clone());

            // 添加示例消息
            session.messages = create_sample_messages(&contact);

            self.current_session = Some(session.clone());

            // 更新聊天区域
            self.chat_area.update(cx, |area, cx| {
                area.set_session(Some(session), cx);
            });
        }
    }

    fn on_send_message(&mut self, content: String, cx: &mut Context<Self>) {
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

    fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for WeixinApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_maximized = window.is_maximized();
        let current_chat_title = self
            .current_session
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
            .unwrap_or_else(|| "选择一个会话".to_string());

        v_flex()
            .size_full()
            .bg(gpui::rgb(0xededed))
            .child(
                // 统一的标题栏 - 包含搜索框、聊天标题、窗口控制
                h_flex()
                    .w_full()
                    .h(px(64.))
                    .items_center()
                    .child(
                        // 顶部用户头像
                        div()
                            .window_control_area(WindowControlArea::Drag)
                            .w(px(64.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Avatar::new().name("HL")),
                    )
                    .child(
                        h_resizable("title-search-resizable", self.session_resizable_state.clone())
                            .child(
                                resizable_panel()
                                    .size(px(200.))
                                    .size_range(px(200.)..px(400.))
                                    .child(
                                        // 搜索框区域
                                        div()
                                            .bg(gpui::rgb(0xF7F7F7))
                                            .size_full()
                                            .window_control_area(WindowControlArea::Drag)
                                            .flex()
                                            .items_center()
                                            .px_3()
                                            .child(
                                                div()
                                                    .w_full()
                                                    .flex_1()
                                                    .bg(gpui::rgb(0xEAEAEA))
                                                    .rounded(px(4.))
                                                    .px_2()
                                                    .py_1()
                                                    .child(
                                                        TextInput::new(
                                                            &self
                                                                .session_list
                                                                .read(cx)
                                                                .search_input,
                                                        )
                                                        .appearance(false),
                                                    ),
                                            ),
                                    ),
                            )
                            .child(
                                resizable_panel().child(
                                    h_flex()
                                        .h_full()
                                        .flex_1()
                                        .items_center()
                                        .justify_end()
                                        .child(
                                            h_flex()
                                                .window_control_area(WindowControlArea::Drag)
                                                .h_full()
                                                .flex_1()
                                                .items_center()
                                                .pl_2()
                                                .child(current_chat_title),
                                        )
                                        .child(
                                            h_flex()
                                                .h_full()
                                                .items_center()
                                                .child(
                                                    // 最小化
                                                    div()
                                                        .id("win-btn-min")
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .h_full()
                                                        .w(px(45.))
                                                        .window_control_area(WindowControlArea::Min)
                                                        .cursor_pointer()
                                                        .hover(|s| s.bg(gpui::rgb(0xe0e0e0)))
                                                        .child(
                                                            Icon::new(IconName::WindowMinimize)
                                                                .xsmall(),
                                                        ),
                                                )
                                                .child(
                                                    // 最大化/还原
                                                    div()
                                                        .id("win-btn-max")
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .h_full()
                                                        .w(px(45.))
                                                        .window_control_area(WindowControlArea::Max)
                                                        .cursor_pointer()
                                                        .hover(|s| s.bg(gpui::rgb(0xe0e0e0)))
                                                        .child(
                                                            Icon::new(if is_maximized {
                                                                IconName::WindowRestore
                                                            } else {
                                                                IconName::WindowMaximize
                                                            })
                                                            .xsmall(),
                                                        ),
                                                )
                                                .child(
                                                    // 关闭
                                                    div()
                                                        .id("win-btn-close")
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .h_full()
                                                        .w(px(45.))
                                                        .window_control_area(
                                                            WindowControlArea::Close,
                                                        )
                                                        .cursor_pointer()
                                                        .hover(|s| {
                                                            s.bg(gpui::rgb(0xe81123))
                                                                .text_color(gpui::white())
                                                        })
                                                        .child(
                                                            Icon::new(IconName::WindowClose)
                                                                .xsmall(),
                                                        ),
                                                ),
                                        ),
                                ),
                            ),
                    ),
            )
            .child(
                // 主内容区域 - 可调整大小的布局
                h_flex()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(self.toolbar.clone())
                    .child(
                        h_resizable("session-list-resizable", self.session_resizable_state.clone())
                            .child(
                                resizable_panel()
                                    .size(px(200.))
                                    .size_range(px(200.)..px(400.))
                                    .child(self.session_list.clone()),
                            )
                            .child(resizable_panel().child(self.chat_area.clone())),
                    ),
            )
    }
}

fn create_sample_contacts() -> Vec<Contact> {
    let mut contacts = vec![
        // 群组
        Contact::new("group1", "Rust学习小组").as_group(
            156,
            vec![
                "张三".to_string(),
                "李四".to_string(),
                "王五".to_string(),
                "赵六".to_string(),
            ],
        ),
        Contact::new("group2", "项目讨论组").as_group(
            68,
            vec![
                "Alice".to_string(),
                "Bob".to_string(),
                "Charlie".to_string(),
                "David".to_string(),
            ],
        ),
        Contact::new("group3", "技术交流群").as_group(
            234,
            vec![
                "前端".to_string(),
                "后端".to_string(),
                "测试".to_string(),
                "运维".to_string(),
            ],
        ),
        // 个人
        Contact::new("1", "张三"),
        Contact::new("2", "李四"),
        Contact::new("3", "王五"),
        Contact::new("4", "赵六"),
        Contact::new("5", "钱七"),
    ];

    // 设置消息预览和时间
    for (i, contact) in contacts.iter_mut().enumerate() {
        if contact.is_group {
            contact.last_sender_name = Some(
                contact
                    .avatar_members
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "某人".to_string()),
            );
            contact.last_message = Some("大家好，今天讨论一下项目进度".to_string());
        } else {
            contact.last_message = Some(format!("这是来自{}的最后一条消息", contact.name));
        }
        contact.last_message_time = Some(Local::now() - chrono::Duration::minutes((i * 10) as i64));
        contact.unread_count = if i % 3 == 0 { (i as u32) % 5 + 1 } else { 0 };
    }

    contacts
}

fn create_sample_messages(contact: &Contact) -> Vec<Message> {
    let base_time = Local::now() - chrono::Duration::hours(2);

    vec![
        Message {
            id: "1".into(),
            sender_id: contact.id.clone(),
            sender_name: contact.name.clone(),
            content: "你好！".into(),
            timestamp: base_time,
            is_self: false,
        },
        Message {
            id: "2".into(),
            sender_id: "self".into(),
            sender_name: "我".into(),
            content: "你好，有什么事吗？".into(),
            timestamp: base_time + chrono::Duration::minutes(1),
            is_self: true,
        },
        Message {
            id: "3".into(),
            sender_id: contact.id.clone(),
            sender_name: contact.name.clone(),
            content: "想问一下你那个项目进度如何了？".into(),
            timestamp: base_time + chrono::Duration::minutes(2),
            is_self: false,
        },
        Message {
            id: "4".into(),
            sender_id: "self".into(),
            sender_name: "我".into(),
            content: "项目进展很顺利，预计下周就能完成了。".into(),
            timestamp: base_time + chrono::Duration::minutes(3),
            is_self: true,
        },
        Message {
            id: "5".into(),
            sender_id: contact.id.clone(),
            sender_name: contact.name.clone(),
            content: "太好了！到时候记得通知我一声。".into(),
            timestamp: base_time + chrono::Duration::minutes(5),
            is_self: false,
        },
    ]
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        cx.activate(true);

        let window_size = Size {
            width: px(900.0),
            height: px(650.0),
        };

        let window_bounds = Bounds::centered(None, window_size, cx);

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(TitleBar::title_bar_options()),
            window_min_size: Some(Size {
                width: px(800.),
                height: px(600.),
            }),
            kind: WindowKind::Normal,
            ..Default::default()
        };

        cx.open_window(options, |window, cx| {
            let app_view = WeixinApp::view(window, cx);
            cx.new(|cx| Root::new(app_view.into(), window, cx))
        })
        .expect("failed to open window");
    });
}
