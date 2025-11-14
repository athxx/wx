use gpui::Action;

use crate::models::ToolbarItem;

/// 选择会话的全局动作，由会话列表触发，由 `WeixinApp` 统一处理。
#[derive(Action, Clone, PartialEq, Eq)]
#[action(namespace = weixin, no_json)]
pub struct SelectSession {
    pub contact_id: String,
}

/// 工具栏点击的全局动作，由工具栏触发，由 `WeixinApp` 统一处理。
#[derive(Action, Clone, Copy, PartialEq, Eq)]
#[action(namespace = weixin, no_json)]
pub struct ToolbarClicked {
    pub item: ToolbarItem,
}
