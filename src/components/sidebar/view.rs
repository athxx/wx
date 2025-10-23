use gpui::{
    div, px, App, AppContext, Context, Corner, DismissEvent, Element, Entity, EventEmitter,
    InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement, Styled,
    Window,
};
use gpui_component::{
    button::{Button, ButtonVariants},
    popover::{Popover, PopoverContent},
    v_flex, ActiveTheme, ContextModal, Icon,
};

use crate::ui::theme::Theme;

use crate::models::ToolbarItem;

use crate::app::events::AppEvent;

pub struct ToolBar {
    active_item: ToolbarItem,
}

impl EventEmitter<AppEvent> for ToolBar {}

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
        id: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = self.active_item == item;
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);

        // 根据是否激活和是否有fill版本决定图标路径
        let icon_path = if is_active {
            item.icon_path_fill().unwrap_or(item.icon_path())
        } else {
            item.icon_path()
        };

        // 只有带fill的图标在激活时才变绿色
        let icon_color = if is_active && item.has_fill() {
            weixin_colors.weixin_green
        } else {
            theme.muted_foreground
        };

        div()
            .w_full()
            .flex()
            .items_center()
            .justify_center()
            .py(px(3.))
            .child(
                div()
                    .id(id)
                    .flex()
                    .items_center()
                    .justify_center()
                    .p(px(10.))
                    .rounded(px(6.))
                    .cursor_pointer()
                    .hover(|this| this.bg(theme.secondary))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.active_item = item;
                        cx.emit(AppEvent::ToolbarClicked { item });
                        cx.notify();
                    }))
                    .child(
                        Icon::default()
                            .path(icon_path)
                            .w(crate::ui::constants::icon_md())
                            .h(crate::ui::constants::icon_md())
                            .text_color(icon_color),
                    ),
            )
    }

    fn render_phone_button(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 为 phone 弹出菜单项创建 hover 状态（先创建再 clone，和 render_menu_button 一致）
        let theme = cx.theme();

        div().w_full().flex().items_center().justify_center().child(
            Popover::new("toolbar-phone")
                .anchor(Corner::BottomRight)
                .trigger(
                    Button::new("phone-trigger")
                        .ghost()
                        .w(crate::ui::constants::toolbar_trigger_size())
                        .h(crate::ui::constants::toolbar_trigger_size())
                        .child(
Icon::default()
                                .path("phone.svg")
                                .w(crate::ui::constants::icon_md())
                                .h(crate::ui::constants::icon_md())
                                .text_color(theme.muted_foreground),
                        ),
                )
.content(move |window, cx| {
                    // 每次打开 Popover 时重置 hover 状态
                    let phone_video_hovered = cx.new(|_| false);
                    let phone_voice_hovered = cx.new(|_| false);

                    cx.new(|cx| {
                        PopoverContent::new(window, cx, move |_, cx| {
                            v_flex()
                                .gap_1()
                                .child({
                                    let hovered = *phone_video_hovered.read(cx);
                                    let state = phone_video_hovered.clone();
                                    crate::ui::widgets::toolbar::hover_menu_item::hover_menu_item(
                                        "phone-video-call",
                                        "视频通话",
                                        hovered,
                                        move |is_hovering, cx| {
                                            state.update(cx, |s, _| *s = is_hovering);
                                        },
                                        cx.listener(|_, _, window, cx| {
                                            window.push_notification("视频通话功能开发中...", cx);
                                            cx.emit(DismissEvent);
                                        }),
                                    )
                                })
                                .child({
                                    let hovered = *phone_voice_hovered.read(cx);
                                    let state = phone_voice_hovered.clone();
                                    crate::ui::widgets::toolbar::hover_menu_item::hover_menu_item(
                                        "phone-voice-call",
                                        "语音通话",
                                        hovered,
                                        move |is_hovering, cx| {
                                            state.update(cx, |s, _| *s = is_hovering);
                                        },
                                        cx.listener(|_, _, window, cx| {
                                            window.push_notification("语音通话功能开发中...", cx);
                                            cx.emit(DismissEvent);
                                        }),
                                    )
                                })
                                .into_any()
                        })
                        .p_2()
                    })
                }),
        )
    }

    fn render_menu_button(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 为每个菜单项创建 hover 状态
        let theme = cx.theme();

        div()
            .w_full()
            .flex()
            .items_center()
            .justify_center()
            .py(px(4.))
            .child(
                Popover::new("toolbar-menu")
                    .anchor(Corner::BottomRight)
                    .trigger(
                        Button::new("menu-trigger")
                            .ghost()
                            .w(crate::ui::constants::toolbar_trigger_size())
                            .h(crate::ui::constants::toolbar_trigger_size())
                            .child(
Icon::default()
                                    .path("menu.svg")
                                    .w(crate::ui::constants::icon_md())
                                    .h(crate::ui::constants::icon_md())
                                    .text_color(theme.muted_foreground),
                            ),
                    )
.content(move |window, cx| {
                        let theme_popover = cx.theme().popover;
                        // 每次打开 Popover 时重置 hover 状态
                        let video_live_hovered = cx.new(|_| false);
                        let chat_files_hovered = cx.new(|_| false);
                        let chat_history_hovered = cx.new(|_| false);
                        let lock_hovered = cx.new(|_| false);
                        let feedback_hovered = cx.new(|_| false);
                        let settings_hovered = cx.new(|_| false);
                        cx.new(|cx| {
                            PopoverContent::new(window, cx, move |_, cx| {
                                let theme = cx.theme();
                                v_flex()
                                    .w(crate::ui::constants::toolbar_popover_width())
                                    .gap_0()
                                    .py_2()
                                    .text_color(theme.foreground)
                                    .child({
                                        let hovered = *video_live_hovered.read(cx);
                                        let state = video_live_hovered.clone();

                                        crate::ui::widgets::toolbar::hover_menu_item::hover_menu_item(
                                            "menu-video-live",
                                            "视频号直播伴侣",
                                            hovered,
                                            move |is_hovering, cx| {
                                                state.update(cx, |s, _| *s = is_hovering);
                                            },
                                            cx.listener(|_, _, window, cx| {
                                                window.push_notification(
                                                    "视频号直播伴侣功能开发中...",
                                                    cx,
                                                );
                                                cx.emit(DismissEvent);
                                            }),
                                        )
                                    })
                                    .child({
                                        let hovered = *chat_files_hovered.read(cx);
                                        let state = chat_files_hovered.clone();

                                        crate::ui::widgets::toolbar::hover_menu_item::hover_menu_item(
                                            "menu-chat-files",
                                            "聊天文件",
                                            hovered,
                                            move |is_hovering, cx| {
                                                state.update(cx, |s, _| *s = is_hovering);
                                            },
                                            cx.listener(|_, _, window, cx| {
                                                window.push_notification("聊天文件功能开发中...", cx);
                                                cx.emit(DismissEvent);
                                            }),
                                        )
                                    })
                                    .child({
                                        let hovered = *chat_history_hovered.read(cx);
                                        let state = chat_history_hovered.clone();

                                        crate::ui::widgets::toolbar::hover_menu_item::hover_menu_item(
                                            "menu-chat-history",
                                            "聊天记录管理",
                                            hovered,
                                            move |is_hovering, cx| {
                                                state.update(cx, |s, _| *s = is_hovering);
                                            },
                                            cx.listener(|_, _, window, cx| {
                                                window.push_notification(
                                                    "聊天记录管理功能开发中...",
                                                    cx,
                                                );
                                                cx.emit(DismissEvent);
                                            }),
                                        )
                                    })
                                    .child({
                                        let hovered = *lock_hovered.read(cx);
                                        let state = lock_hovered.clone();

                                        crate::ui::widgets::toolbar::hover_menu_item::hover_menu_item(
                                            "menu-lock",
                                            "锁定",
                                            hovered,
                                            move |is_hovering, cx| {
                                                state.update(cx, |s, _| *s = is_hovering);
                                            },
                                            cx.listener(|_, _, window, cx| {
                                                window.push_notification("锁定功能开发中...", cx);
                                                cx.emit(DismissEvent);
                                            }),
                                        )
                                    })
                                    .child({
                                        let hovered = *feedback_hovered.read(cx);
                                        let state = feedback_hovered.clone();

                                        crate::ui::widgets::toolbar::hover_menu_item::hover_menu_item(
                                            "menu-feedback",
                                            "意见反馈",
                                            hovered,
                                            move |is_hovering, cx| {
                                                state.update(cx, |s, _| *s = is_hovering);
                                            },
                                            cx.listener(|_, _, window, cx| {
                                                window.push_notification("意见反馈功能开发中...", cx);
                                                cx.emit(DismissEvent);
                                            }),
                                        )
                                    })
                                    .child({
                                        let hovered = *settings_hovered.read(cx);
                                        let state = settings_hovered.clone();

                                        crate::ui::widgets::toolbar::hover_menu_item::hover_menu_item(
                                            "menu-settings",
                                            "设置",
                                            hovered,
                                            move |is_hovering, cx| {
                                                state.update(cx, |s, _| *s = is_hovering);
                                            },
                                            cx.listener(|_, _, _, cx| {
                                                crate::app::WeixinApp::open_settings_window(cx);
                                                cx.emit(DismissEvent);
                                            }),
                                        )
                                    })
                                    .into_any()
                            })
                            .p_1()
                            .bg(theme_popover)
                            .rounded(px(6.))
                            .shadow_md()
                        })
                    }),
            )
    }
}

impl Render for ToolBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let weixin_colors = Theme::weixin_colors(cx);

        v_flex()
            .bg(weixin_colors.toolbar_bg) // 左侧工具栏背景 EDEDED
            .w(crate::ui::constants::toolbar_width())
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
                    .child(self.render_toolbar_button(ToolbarItem::Chat, "toolbar-chat", cx))
                    .child(self.render_toolbar_button(
                        ToolbarItem::Contacts,
                        "toolbar-contacts",
                        cx,
                    ))
                    .child(self.render_toolbar_button(
                        ToolbarItem::Favorites,
                        "toolbar-favorites",
                        cx,
                    ))
                    .child(self.render_toolbar_button(ToolbarItem::Moments, "toolbar-moments", cx))
                    .child(self.render_toolbar_button(
                        ToolbarItem::Channels,
                        "toolbar-channels",
                        cx,
                    ))
                    .child(self.render_toolbar_button(ToolbarItem::Search, "toolbar-search", cx))
                    .child(self.render_toolbar_button(
                        ToolbarItem::MiniProgram,
                        "toolbar-miniprogram",
                        cx,
                    )),
            )
            .child(
                // 底部按钮
                v_flex()
                    .w_full()
                    .items_center()
                    .gap_0()
                    .mb_2()
                    .child(self.render_phone_button(_window, cx))
                    .child(self.render_menu_button(_window, cx)),
            )
    }
}
