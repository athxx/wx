use gpui::{
    div, prelude::FluentBuilder, px, rgb, App, AppContext, Context, Entity, EventEmitter,
    InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement, Styled,
    Window,
};
use gpui_component::{avatar::Avatar, v_flex, Icon, IconName, Sizable};

use crate::models::ToolbarItem;

#[derive(Clone)]
pub struct ToolbarClickEvent {
    pub item: ToolbarItem,
}

pub struct ToolBar {
    active_item: ToolbarItem,
}

impl EventEmitter<ToolbarClickEvent> for ToolBar {}

impl ToolBar {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            active_item: ToolbarItem::Chat,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn set_active_item(&mut self, item: ToolbarItem, cx: &mut Context<Self>) {
        self.active_item = item;
        cx.notify();
    }

    fn render_toolbar_button(
        &self,
        item: ToolbarItem,
        icon: IconName,
        id: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = self.active_item == item;

        div()
            .w_full()
            .flex()
            .items_center()
            .justify_center()
            .py(px(8.))
            .child(
                div()
                    .id(id)
                    .flex()
                    .items_center()
                    .justify_center()
                    .p(px(8.))
                    .rounded(px(8.))
                    .cursor_pointer()
                    .hover(|this| this.bg(rgb(0xdcdcdc)))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.active_item = item;
                        cx.emit(ToolbarClickEvent { item });
                        cx.notify();
                    }))
                    .child(Icon::new(icon).size_6().text_color(if is_active {
                        rgb(0x07c160)
                    } else {
                        rgb(0x666666)
                    })),
            )
    }
}

impl Render for ToolBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(px(64.))
            .h_full()
            .items_center()
            .py_2()
            .child(
                // 主要功能按钮
                v_flex()
                    .flex_1()
                    .w_full()
                    .gap_0()
                    .items_center()
                    .child(self.render_toolbar_button(
                        ToolbarItem::Chat,
                        IconName::Inbox,
                        "toolbar-chat",
                        cx,
                    ))
                    .child(self.render_toolbar_button(
                        ToolbarItem::Contacts,
                        IconName::User,
                        "toolbar-contacts",
                        cx,
                    ))
                    .child(self.render_toolbar_button(
                        ToolbarItem::Favorites,
                        IconName::Star,
                        "toolbar-favorites",
                        cx,
                    ))
                    .child(self.render_toolbar_button(
                        ToolbarItem::Files,
                        IconName::Folder,
                        "toolbar-files",
                        cx,
                    )),
            )
            .child(
                // 底部设置按钮
                v_flex()
                    .w_full()
                    .items_center()
                    .mb_2()
                    .child(self.render_toolbar_button(
                        ToolbarItem::Settings,
                        IconName::Settings,
                        "toolbar-settings",
                        cx,
                    )),
            )
    }
}
