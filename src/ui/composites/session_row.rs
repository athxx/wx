use crate::models::Contact;
use crate::utils::time::format_time_friendly;
use gpui::{div, App, IntoElement, ParentElement, RenderOnce, Styled, Window};
use gpui_component::{h_flex, v_flex, ActiveTheme};

#[derive(IntoElement)]
pub struct SessionRow {
    contact: Contact,
}

impl SessionRow {
    pub fn new(contact: Contact) -> Self {
        Self { contact }
    }
}

impl RenderOnce for SessionRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let time_str = self.contact
            .last_message_time
            .as_ref()
            .map(|time| format_time_friendly(time))
            .unwrap_or_default();

        h_flex()
            .w_full()
            .gap_3()
            .items_center()
            .child({
                let avatar = if self.contact.is_group {
                    crate::ui::base::avatar::Avatar::group(self.contact.avatar_members.clone())
                } else {
                    crate::ui::base::avatar::Avatar::new(crate::ui::avatar::avatar_for_key(&self.contact.id))
                };
                avatar.unread_count(self.contact.unread_count as usize)
            })
            .child(
                v_flex()
                    .flex_1()
                    .gap_1()
                    .overflow_hidden()
                    .min_w_0()
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .flex_1()
                                    .text_base()
                                    .font_weight(gpui::FontWeight::NORMAL)
                                    .text_color(theme.foreground)
                                    .overflow_hidden()
                                    .whitespace_nowrap()
                                    .text_ellipsis()
                                    .text_sm()
                                    .child(if self.contact.is_group {
                                        if let Some(count) = self.contact.member_count {
                                            format!("{} ({})", self.contact.name, count)
                                        } else {
                                            self.contact.name.clone()
                                        }
                                    } else {
                                        self.contact.name.clone()
                                    }),
                            )
                            .child(
                                div()
                                    .flex_shrink_0()
                                    .text_xs()
                                    .text_color(theme.muted_foreground)
                                    .child(time_str),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .text_xs()
                            .text_color(theme.muted_foreground)
                            .overflow_hidden()
                            .whitespace_nowrap()
                            .text_ellipsis()
                            .child({
                                let message = self
                                    .contact
                                    .last_message
                                    .as_ref()
                                    .map(|m| m.lines().next().unwrap_or("").to_string())
                                    .unwrap_or_else(|| "暂无消息".to_string());
                                if self.contact.is_group {
                                    if let Some(sender) = &self.contact.last_sender_name {
                                        format!("{}: {}", sender, message)
                                    } else {
                                        message
                                    }
                                } else {
                                    message
                                }
                            }),
                    ),
            )
    }
}
