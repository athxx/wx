use gpui::{
    actions, div, prelude::FluentBuilder, px, rgb, Action, AnyElement, App, AppContext, Context,
    Corner, DismissEvent, Element, Entity, EventEmitter, InteractiveElement, IntoElement,
    ParentElement, Render, StatefulInteractiveElement, Styled, Window,
};
use gpui_component::{
    avatar::Avatar,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    divider::Divider,
    h_flex,
    popover::{Popover, PopoverContent},
    v_flex, ContextModal, Icon, IconName, Sizable,
};

use crate::theme::Theme;
use serde::Deserialize;

use crate::models::ToolbarItem;

// 定义菜单相关的 Actions
actions!(
    toolbar,
    [
        VideoLiveCompanion,
        ChatFiles,
        ChatHistoryManagement,
        Lock,
        Feedback,
        Settings
    ]
);

#[derive(Clone)]
pub struct ToolbarClickEvent {
    pub item: ToolbarItem,
}

#[derive(Clone)]
pub struct OpenSettingsEvent;

pub struct ToolBar {
    active_item: ToolbarItem,
}

impl EventEmitter<ToolbarClickEvent> for ToolBar {}
impl EventEmitter<OpenSettingsEvent> for ToolBar {}

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
        let theme = Theme::get(cx);

        // 根据是否激活和是否有fill版本决定图标路径
        let icon_path = if is_active {
            item.icon_path_fill().unwrap_or(item.icon_path())
        } else {
            item.icon_path()
        };

        // 只有带fill的图标在激活时才变绿色
        let icon_color = if is_active && item.has_fill() {
            theme.colors.toolbar_icon_active
        } else {
            theme.colors.toolbar_icon_normal
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
                    .hover(|this| this.bg(theme.colors.toolbar_active_bg))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.active_item = item;
                        cx.emit(ToolbarClickEvent { item });
                        cx.notify();
                    }))
                    .child(
                        Icon::default()
                            .path(icon_path)
                            .w(px(21.))
                            .h(px(21.))
                            .text_color(icon_color),
                    ),
            )
    }

    fn render_phone_button(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx);

        // 为 phone 弹出菜单项创建 hover 状态（先创建再 clone，和 render_menu_button 一致）
        let phone_video_hovered =
            window.use_keyed_state("phone-video-call-hover", cx, |_, _| false);
        let phone_voice_hovered =
            window.use_keyed_state("phone-voice-call-hover", cx, |_, _| false);

        // 外层 clone，供 content 闭包捕获
        let phone_video_hovered = phone_video_hovered.clone();
        let phone_voice_hovered = phone_voice_hovered.clone();

        div().w_full().flex().items_center().justify_center().child(
            Popover::new("toolbar-phone")
                .anchor(Corner::BottomRight)
                .trigger(
                    Button::new("phone-trigger")
                        .ghost()
                        .w(px(41.))
                        .h(px(41.))
                        .child(
                            Icon::default()
                                .path("phone.svg")
                                .w(px(21.))
                                .h(px(21.))
                                .text_color(theme.colors.toolbar_icon_normal),
                        ),
                )
                .content(move |window, cx| {
                    // 内层再 clone，供 PopoverContent 的 move 闭包使用
                    let phone_video_hovered = phone_video_hovered.clone();
                    let phone_voice_hovered = phone_voice_hovered.clone();

                    cx.new(|cx| {
                        PopoverContent::new(window, cx, move |_, cx| {
                            v_flex()
                                .gap_1()
                                .child({
                                    let hovered = *phone_video_hovered.read(cx);
                                    let state = phone_video_hovered.clone();
                                    ToolBar::render_hover_menu_item(
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
                                    ToolBar::render_hover_menu_item(
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

    fn render_hover_menu_item<FSet, L>(
        id: &'static str,
        label: &'static str,
        hovered: bool,
        set_hover: FSet,
        on_mouse_down: L,
    ) -> impl IntoElement
    where
        FSet: Fn(bool, &mut App) + Clone + 'static,
        L: Fn(&gpui::MouseDownEvent, &mut Window, &mut App) + 'static,
    {
        div()
            .id(id)
            .w_full()
            .px_3()
            .py_2()
            .rounded(px(4.))
            .cursor_pointer()
            .text_sm()
            .when(hovered, |this| this.bg(rgb(0x07C160)))
            .on_hover(move |&is_hovering, _, cx| {
                set_hover(is_hovering, cx);
            })
            .on_mouse_down(gpui::MouseButton::Left, on_mouse_down)
            .child(
                div()
                    .when(hovered, |this| this.text_color(rgb(0xFFFFFF)))
                    .when(!hovered, |this| this.text_color(rgb(0x000000)))
                    .child(label),
            )
    }

    fn render_menu_button(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx);

        // 为每个菜单项创建 hover 状态
        let video_live_hovered = window.use_keyed_state("menu-video-live-hover", cx, |_, _| false);
        let chat_files_hovered = window.use_keyed_state("menu-chat-files-hover", cx, |_, _| false);
        let chat_history_hovered =
            window.use_keyed_state("menu-chat-history-hover", cx, |_, _| false);
        let lock_hovered = window.use_keyed_state("menu-lock-hover", cx, |_, _| false);
        let feedback_hovered = window.use_keyed_state("menu-feedback-hover", cx, |_, _| false);
        let settings_hovered = window.use_keyed_state("menu-settings-hover", cx, |_, _| false);

        // Clone for closure capture
        let video_live_hovered = video_live_hovered.clone();
        let chat_files_hovered = chat_files_hovered.clone();
        let chat_history_hovered = chat_history_hovered.clone();
        let lock_hovered = lock_hovered.clone();
        let feedback_hovered = feedback_hovered.clone();
        let settings_hovered = settings_hovered.clone();

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
                            .w(px(41.))
                            .h(px(41.))
                            .child(
                                Icon::default()
                                    .path("menu.svg")
                                    .w(px(21.))
                                    .h(px(21.))
                                    .text_color(theme.colors.toolbar_icon_normal),
                            ),
                    )
                    .content(move |window, cx| {
                        let video_live_hovered = video_live_hovered.clone();
                        let chat_files_hovered = chat_files_hovered.clone();
                        let chat_history_hovered = chat_history_hovered.clone();
                        let lock_hovered = lock_hovered.clone();
                        let feedback_hovered = feedback_hovered.clone();
                        let settings_hovered = settings_hovered.clone();
                        cx.new(|cx| {
                            PopoverContent::new(window, cx, move |_, cx| {
                                v_flex()
                                    .w(px(130.))
                                    .gap_0()
                                    .py_2()
                                    .child({
                                        let hovered = *video_live_hovered.read(cx);
                                        let state = video_live_hovered.clone();

                                        ToolBar::render_hover_menu_item(
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

                                        ToolBar::render_hover_menu_item(
                                            "menu-chat-files",
                                            "聊天文件",
                                            hovered,
                                            move |is_hovering, cx| {
                                                state.update(cx, |s, _| *s = is_hovering);
                                            },
                                            cx.listener(|_, _, window, cx| {
                                                window
                                                    .push_notification("聊天文件功能开发中...", cx);
                                                cx.emit(DismissEvent);
                                            }),
                                        )
                                    })
                                    .child({
                                        let hovered = *chat_history_hovered.read(cx);
                                        let state = chat_history_hovered.clone();

                                        ToolBar::render_hover_menu_item(
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

                                        ToolBar::render_hover_menu_item(
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

                                        ToolBar::render_hover_menu_item(
                                            "menu-feedback",
                                            "意见反馈",
                                            hovered,
                                            move |is_hovering, cx| {
                                                state.update(cx, |s, _| *s = is_hovering);
                                            },
                                            cx.listener(|_, _, window, cx| {
                                                window
                                                    .push_notification("意见反馈功能开发中...", cx);
                                                cx.emit(DismissEvent);
                                            }),
                                        )
                                    })
                                    .child({
                                        let hovered = *settings_hovered.read(cx);
                                        let state = settings_hovered.clone();

                                        ToolBar::render_hover_menu_item(
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
                            .bg(rgb(0xFFFFFF))
                            .rounded(px(6.))
                            .shadow_md()
                        })
                    }),
            )
    }

    fn on_settings(&mut self, _: &Settings, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(OpenSettingsEvent);
    }

    fn on_video_live(
        &mut self,
        _: &VideoLiveCompanion,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.push_notification("视频号直播伴侣功能开发中...", cx);
    }

    fn on_chat_files(&mut self, _: &ChatFiles, window: &mut Window, cx: &mut Context<Self>) {
        window.push_notification("聊天文件功能开发中...", cx);
    }

    fn on_chat_history(
        &mut self,
        _: &ChatHistoryManagement,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.push_notification("聊天记录管理功能开发中...", cx);
    }

    fn on_lock(&mut self, _: &Lock, window: &mut Window, cx: &mut Context<Self>) {
        window.push_notification("锁定功能开发中...", cx);
    }

    fn on_feedback(&mut self, _: &Feedback, window: &mut Window, cx: &mut Context<Self>) {
        window.push_notification("意见反馈功能开发中...", cx);
    }
}

impl Render for ToolBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .on_action(cx.listener(Self::on_settings))
            .on_action(cx.listener(Self::on_video_live))
            .on_action(cx.listener(Self::on_chat_files))
            .on_action(cx.listener(Self::on_chat_history))
            .on_action(cx.listener(Self::on_lock))
            .on_action(cx.listener(Self::on_feedback))
            .w(px(67.))
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
