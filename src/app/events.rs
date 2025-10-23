// Unified application events used across UI and app layers
#[derive(Clone, Debug)]
pub enum AppEvent {
    SessionSelected { contact_id: String },
    ToolbarClicked { item: crate::models::ToolbarItem },
}
