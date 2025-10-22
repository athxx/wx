use gpui::{
    div, prelude::FluentBuilder, px, App, AppContext, Context, Entity, EventEmitter,
    InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement, Styled,
    Window,
};
use gpui_component::{
    avatar::Avatar,
    badge::Badge,
    h_flex,
    input::{InputState, TextInput},
    v_flex, ActiveTheme, Sizable,
};

use crate::components::GroupAvatar;
use crate::models::Contact;
use crate::theme::{Theme, WeixinThemeColors};

#[derive(Clone)]
pub struct SessionSelectEvent {
    pub contact_id: String,
}

pub struct SessionList {
    contacts: Vec<Contact>,
    selected_id: Option<String>,
    pub search_input: Entity<InputState>,
}

impl EventEmitter<SessionSelectEvent> for SessionList {}

impl SessionList {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("搜索"));
        Self {
            contacts: Vec::new(),
            selected_id: None,
            search_input,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn set_contacts(&mut self, contacts: Vec<Contact>, cx: &mut Context<Self>) {
        self.contacts = contacts;
        cx.notify();
    }

    pub fn set_selected(&mut self, contact_id: Option<String>, cx: &mut Context<Self>) {
        self.selected_id = contact_id;
        cx.notify();
    }

    pub fn update_contact_last_message(
        &mut self,
        contact_id: &str,
        message: String,
        cx: &mut Context<Self>,
    ) {
        if let Some(contact) = self.contacts.iter_mut().find(|c| c.id == contact_id) {
            contact.last_message = Some(message);
            contact.last_message_time = Some(chrono::Local::now());
            cx.notify();
        }
    }

    fn render_session_item(
        &self,
        contact: &Contact,
        index: usize,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_selected = self
            .selected_id
            .as_ref()
            .map(|id| id == &contact.id)
            .unwrap_or(false);
        let contact_id = contact.id.clone();
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);

        let time_str = contact
            .last_message_time
            .as_ref()
            .map(|time| {
                let now = chrono::Local::now();
                let duration = now.signed_duration_since(*time);

                if duration.num_days() > 0 {
                    time.format("%m/%d").to_string()
                } else if duration.num_hours() > 0 {
                    time.format("%H:%M").to_string()
                } else {
                    time.format("%H:%M").to_string()
                }
            })
            .unwrap_or_default();

        div()
            .id(("session-item", index))
            .w_full()
            .px_4()
            .py_3()
            .border_l_1()
            .border_color(theme.border)
            .bg(if is_selected {
                weixin_colors.item_selected  // 选中颜色 DEDEDE
            } else {
                theme.transparent
            })
            .hover(move |style| {
                if !is_selected {
                    style.bg(weixin_colors.item_hover)  // hover颜色 EAEAEA
                } else {
                    style
                }
            })
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                this.selected_id = Some(contact_id.clone());
                cx.emit(SessionSelectEvent {
                    contact_id: contact_id.clone(),
                });
                cx.notify();
            }))
            .child(
                h_flex()
                    .w_full()
                    .gap_3()
                    .items_start()
                    .child(
                        // 头像 - 群组或个人，带未读徽章
                        div()
                            .flex_shrink_0()
                            .when(contact.unread_count > 0, |this| {
                                this.child(
                                    Badge::new()
                                        .count(contact.unread_count as usize)
                                        .max(99)
                                        .color(weixin_colors.unread_badge)
                                        .when(contact.is_group, |badge| {
                                            badge.child(GroupAvatar::new(
                                                contact.avatar_members.clone(),
                                            ))
                                        })
                                        .when(!contact.is_group, |badge| {
                                            badge.child(
                                                div()
                                                    .rounded(px(4.))
                                                    .overflow_hidden()
                                                    .child(Avatar::new().with_size(px(46.))),
                                            )
                                        }),
                                )
                            })
                            .when(contact.unread_count == 0, |this| {
                                this.when(contact.is_group, |div_| {
                                    div_.child(GroupAvatar::new(contact.avatar_members.clone()))
                                })
                                .when(!contact.is_group, |div_| {
                                    div_.child(
                                        div()
                                            .rounded(px(4.))
                                            .overflow_hidden()
                                            .child(Avatar::new().with_size(px(46.))),
                                    )
                                })
                            }),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .gap_1()
                            .overflow_hidden()
                            .min_w_0()
                            .child(
                                // 第一行：名称、时间、未读徽章
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
                                            .child(if contact.is_group {
                                                if let Some(count) = contact.member_count {
                                                    format!("{} ({})", contact.name, count)
                                                } else {
                                                    contact.name.clone()
                                                }
                                            } else {
                                                contact.name.clone()
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
                                // 第二行：消息预览（群组显示发送者）
                                div()
                                    .w_full()
                                    .text_sm()
                                    .text_color(theme.muted_foreground)
                                    .overflow_hidden()
                                    .whitespace_nowrap()
                                    .text_ellipsis()
                                    .child({
                                        let message = contact
                                            .last_message
                                            .clone()
                                            .unwrap_or_else(|| "暂无消息".to_string());

                                        if contact.is_group {
                                            if let Some(sender) = &contact.last_sender_name {
                                                format!("{}: {}", sender, message)
                                            } else {
                                                message
                                            }
                                        } else {
                                            message
                                        }
                                    }),
                            ),
                    ),
            )
    }
}

impl Render for SessionList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);
        
        v_flex()
            .w_full()
            .h_full()
            .bg(weixin_colors.session_list_bg)  // 中间会话列表背景 F7F7F7
            // .border_r_1()
            .border_color(theme.border)
            .child(
                // 会话列表 - 搜索栏已移到 TitleBar
                div()
                    .id("session-list-scroll")
                    .size_full()
                    .overflow_y_scroll()
                    .child(
                        v_flex().children(
                            self.contacts
                                .iter()
                                .enumerate()
                                .map(|(i, contact)| self.render_session_item(contact, i, cx)),
                        ),
                    ),
            )
    }
}
