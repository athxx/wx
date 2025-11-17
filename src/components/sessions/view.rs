use gpui::{
    div, App, AppContext, AvailableSpace, ClickEvent, Context, Entity, InteractiveElement,
    IntoElement, ParentElement, Pixels, Render, StatefulInteractiveElement, Styled, Window,
};
use gpui_component::{
    input::InputState,
    scroll::{Scrollbar, ScrollbarAxis, ScrollbarState},
    v_flex, v_virtual_list, ActiveTheme, StyledExt as _, VirtualListScrollHandle,
};
use std::rc::Rc;

use crate::models::Contact;
use crate::ui::theme::Theme;
use crate::app::actions::{OpenChatWindow, SelectSession};

pub struct SessionList {
    contacts: Vec<Contact>,
    selected_id: Option<String>,
    pub search_input: Entity<InputState>,
    scroll_handle: VirtualListScrollHandle,
    scroll_state: ScrollbarState,
}

impl SessionList {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("搜索"));
        Self {
            contacts: Vec::new(),
            selected_id: None,
            search_input,
            scroll_handle: VirtualListScrollHandle::new(),
            scroll_state: ScrollbarState::default(),
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
                weixin_colors.item_selected
            } else {
                theme.transparent
            })
            .hover(move |style| {
                if !is_selected {
                    style.bg(weixin_colors.item_hover)
                } else {
                    style
                }
            })
            .cursor_pointer()
            .on_click(cx.listener(move |this, ev: &ClickEvent, window, cx| {
                // 再次点击已选中的会话 -> 取消选中；否则选中该会话
                let toggling_off = this
                    .selected_id
                    .as_ref()
                    .map(|id| id == &contact_id)
                    .unwrap_or(false);

                if toggling_off {
                    this.selected_id = None;
                } else {
                    this.selected_id = Some(contact_id.clone());
                }

                // 单击：仍然选中会话
                window.dispatch_action(
                    Box::new(SelectSession {
                        contact_id: contact_id.clone(),
                    }),
                    cx,
                );

                // 双击：在保持选中状态的同时，打开独立聊天窗口
                if ev.click_count() == 2 {
                    window.dispatch_action(
                        Box::new(OpenChatWindow {
                            contact_id: contact_id.clone(),
                        }),
                        cx,
                    );
                }

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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let weixin_colors = Theme::weixin_colors(cx);

        let available_space = gpui::size(
            AvailableSpace::MinContent,
            AvailableSpace::MinContent,
        );

        let item_sizes: Rc<Vec<gpui::Size<Pixels>>> = Rc::new(
            self.contacts
                .iter()
                .enumerate()
                .map(|(i, contact)| {
                    let row = self.render_session_item(contact, i, cx);
                    let mut el = row.into_any_element();
                    el.layout_as_root(available_space, window, cx)
                })
                .collect(),
        );
        let item_sizes_for_render = item_sizes.clone();

        let list_view = v_virtual_list(
            cx.entity().clone(),
            "session-list",
            item_sizes,
            move |view, visible_range, _window, cx| {
                visible_range
                    .map(|ix| {
                        let contact = &view.contacts[ix];
                        view.render_session_item(contact, ix, cx)
                    })
                    .collect()
            },
        )
        .track_scroll(&self.scroll_handle);

        v_flex()
            .w_full()
            .h_full()
            .bg(weixin_colors.session_list_bg)
            .border_color(theme.border)
            .child(
                div()
                    .relative()
                    .size_full()
                    .child(list_view)
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .right_0()
                            .bottom_0()
                            .child(
                                Scrollbar::both(&self.scroll_state, &self.scroll_handle)
                                    .axis(ScrollbarAxis::Vertical),
                            ),
                    ),
            )
    }
}
