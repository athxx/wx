use crate::models::{Contact, Message};
use chrono::Local;

pub fn create_sample_contacts() -> Vec<Contact> {
    let mut contacts = vec![
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
        Contact::new("1", "张三"),
        Contact::new("2", "李四"),
        Contact::new("3", "王五"),
        Contact::new("4", "赵六"),
        Contact::new("5", "钱七"),
    ];

    // 额外生成一些联系人，方便测试会话列表滚动条。
    for i in 0..20 {
        contacts.push(Contact::new(
            format!("auto{}", i),
            format!("自动联系人 {}", i + 1),
        ));
    }

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

pub fn create_sample_messages(contact: &Contact) -> Vec<Message> {
    let base_time = Local::now() - chrono::Duration::hours(2);

    let mut messages = vec![
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
    ];

    // 追加更多示例消息，方便测试聊天区域滚动条。
    for i in 0..40 {
        let is_self = i % 2 == 0;
        let (sender_id, sender_name) = if is_self {
            ("self".to_string(), "我".to_string())
        } else {
            (contact.id.clone(), contact.name.clone())
        };

        messages.push(Message {
            id: format!("{}", i + 6),
            sender_id,
            sender_name,
            content: format!(
                "测试消息 {}：这是一个较长的示例消息内容，用来测试滚动条效果。",
                i + 1
            ),
            timestamp: base_time + chrono::Duration::minutes(10 + i as i64),
            is_self,
        });
    }

    messages
}
