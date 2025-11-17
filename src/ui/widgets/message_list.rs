use gpui::{IntoElement, ParentElement, Styled};
use gpui_component::v_flex;

use crate::models::Message;

#[allow(dead_code)]
pub fn message_list(
    messages: &[Message],
    is_group: bool,
    theme: &gpui_component::Theme,
    weixin_colors: &crate::ui::theme::WeixinThemeColors,
) -> impl IntoElement {
    v_flex()
        .w_full()
        .pt_4()
        .pb_2()
        .children(messages.iter().map(|msg| {
            crate::ui::widgets::message_bubble::message_bubble(msg, is_group, theme, weixin_colors)
        }))
}
