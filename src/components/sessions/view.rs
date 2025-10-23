use gpui::{
    div, App, AppContext, Context, Entity, EventEmitter, InteractiveElement, IntoElement,
    ParentElement, Render, StatefulInteractiveElement, Styled, Window,
};
use gpui_component::{input::InputState, v_flex, ActiveTheme};

use crate::models::Contact;
use crate::ui::theme::Theme;

use crate::app::events::AppEvent;

pub struct SessionList {
    contacts: Vec<Contact>,
    selected_id: Option<String>,
    pub search_input: Entity<InputState>,
}

impl EventEmitter<AppEvent> for SessionList {}

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

        div()
            .id(("session-item", index))
            .w_full()
            .px_4()
            .py_3()
            .border_l_1()
            .border_color(theme.border)
            .bg(if is_selected {
                weixin_colors.item_selected // 选中颜色 DEDEDE
            } else {
                theme.transparent
            })
            .hover(move |style| {
                if !is_selected {
                    style.bg(weixin_colors.item_hover) // hover颜色 EAEAEA
                } else {
                    style
                }
            })
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                this.selected_id = Some(contact_id.clone());
                cx.emit(AppEvent::SessionSelected {
                    contact_id: contact_id.clone(),
                });
                cx.notify();
            }))
            .child(crate::ui::widgets::session_row::session_row_content(
                contact,
                &theme,
                &weixin_colors,
            ))
    }
}

impl Render for SessionList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);

        v_flex()
            .w_full()
            .h_full()
            .bg(weixin_colors.session_list_bg) // 中间会话列表背景 F7F7F7
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
