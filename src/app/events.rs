#![allow(dead_code)]

#[derive(Clone, Debug)]
pub enum AppEvent {
    SessionSelected { contact_id: String },
    ToolbarClicked { item: crate::models::ToolbarItem },
}
