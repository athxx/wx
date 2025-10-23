use crate::ui::theme::{Theme, ThemeMode};
use gpui::{prelude::FluentBuilder, DismissEvent, EventEmitter, *};
use gpui_component::{
    button::Button,
    h_flex,
    popover::{Popover, PopoverContent},
    v_flex, ActiveTheme, ContextModal, Icon, Sizable,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum SettingsTab {
    AccountAndStorage,
    General,
    Shortcuts,
    Notifications,
    Plugins,
    About,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum FontSize {
    Small,
    Standard,
    Large,
}

pub struct SettingsWindow {
    active_tab_ix: usize,
    current_language: String,
    current_font_size: FontSize,
    _theme_observer: Option<gpui::Subscription>,
}

impl EventEmitter<DismissEvent> for SettingsWindow {}

impl SettingsWindow {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // 订阅全局主题变化
        let theme_observer = cx.observe_global::<Theme>(|_this, cx| {
            cx.notify();
        });

        Self {
            active_tab_ix: 1, // General tab
            current_language: "简体中文".to_string(),
            current_font_size: FontSize::Standard,
            _theme_observer: Some(theme_observer),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn get_active_tab(&self) -> SettingsTab {
        match self.active_tab_ix {
            0 => SettingsTab::AccountAndStorage,
            1 => SettingsTab::General,
            2 => SettingsTab::Shortcuts,
            3 => SettingsTab::Notifications,
            4 => SettingsTab::Plugins,
            5 => SettingsTab::About,
            _ => SettingsTab::General,
        }
    }

    fn set_active_tab(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.active_tab_ix = ix;
        cx.notify();
    }

    fn render_content(&self, window: &mut Window, cx: &mut Context<Self>) -> gpui::AnyElement {
        match self.get_active_tab() {
            SettingsTab::AccountAndStorage => {
                self.render_account_and_storage(cx).into_any_element()
            }
            SettingsTab::General => self.render_general_settings(window, cx).into_any_element(),
            SettingsTab::Shortcuts => self.render_shortcuts(cx).into_any_element(),
            SettingsTab::Notifications => self.render_notifications(cx).into_any_element(),
            SettingsTab::Plugins => self.render_plugins(cx).into_any_element(),
            SettingsTab::About => self.render_about(cx).into_any_element(),
        }
    }

    fn render_account_and_storage(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let _weixin_colors = Theme::weixin_colors(cx);

        v_flex()
            .gap_6()
            .child(
                div()
                    .text_lg()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(theme.foreground)
                    .child("账号与存储"),
            )
            .child(
                v_flex()
                    .gap_4()
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(theme.foreground)
                                    .child("账号信息"),
                            )
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .w(crate::ui::constants::settings_avatar_size())
                                            .h(crate::ui::constants::settings_avatar_size())
                                            .rounded(crate::ui::constants::radius_sm())
                                            .bg(theme.secondary),
                                    )
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(theme.foreground)
                                                    .child("用户名"),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.muted_foreground)
                                                    .child("微信号: wxid_123456"),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(theme.foreground)
                                    .child("文件存储"),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.muted_foreground)
                                            .child("文件存储位置"),
                                    )
                                    .child(
                                        h_flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.muted_foreground)
                                                    .child("C:\\Users\\Documents\\WeChat Files"),
                                            )
                                            .child(
                                                div()
                                                    .px_2()
                                                    .py_1()
                                                    .rounded(px(4.))
                                                    .bg(theme.secondary)
                                                    .cursor_pointer()
                                                    .hover(|s| s.bg(theme.secondary_hover))
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(theme.foreground)
                                                            .child("更改"),
                                                    ),
                                            ),
                                    ),
                            ),
                    ),
            )
    }

    fn render_general_settings(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme_mode = cx.theme().mode;
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);
        let foreground = theme.foreground;

        let card_bg = weixin_colors.session_list_bg; // F7F7F7 / 深色下为 1F1F1F
        let border_color = theme.border;

        v_flex()
            .gap_6()
            // 语言设置 Card
            .child(
                div()
                    .bg(card_bg)
                    .rounded(crate::ui::constants::radius_lg())
                    .border_1()
                    .border_color(border_color)
                    .p_4()
                    .child(
                        h_flex()
                            .items_center()
                            .justify_between()
                            .py_2()
                            .child(div().text_sm().text_color(foreground).child("语言"))
                            .child(self.render_language_button(window, cx)),
                    ),
            )
            // 外观和字体大小设置 Card
            .child(
                div()
                    .bg(card_bg)
                    .rounded(px(8.))
                    .border_1()
                    .border_color(border_color)
                    .p_4()
                    .child(
                        v_flex()
                            .gap_4()
                            // 外观设置
                            .child(self.render_theme_setting(theme_mode, window, cx))
                            // 分割线
                            .child(
                                div()
                                    .w_full()
                                    .h(crate::ui::constants::hairline())
                                    .bg(border_color),
                            )
                            // 字体大小设置
                            .child(
                                h_flex()
                                    .items_center()
                                    .justify_between()
                                    .py_2()
                                    .child(div().text_sm().text_color(foreground).child("字体大小"))
                                    .child(self.render_font_size_button(window, cx)),
                            ),
                    ),
            )
    }

    fn render_shortcuts(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .gap_6()
            .child(
                div()
                    .text_lg()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(theme.foreground)
                    .child("快捷键"),
            )
            .child(
                v_flex()
                    .gap_3()
                    .child(self.render_shortcut_item("打开微信", "Ctrl + Alt + W", cx))
                    .child(self.render_shortcut_item("关闭窗口", "Esc", cx))
                    .child(self.render_shortcut_item("搜索", "Ctrl + F", cx))
                    .child(self.render_shortcut_item("发送消息", "Enter", cx))
                    .child(self.render_shortcut_item("换行", "Shift + Enter", cx)),
            )
    }

    fn render_notifications(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .gap_6()
            .child(
                div()
                    .text_lg()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(theme.foreground)
                    .child("通知"),
            )
            .child(
                v_flex()
                    .gap_4()
                    .child(self.render_setting_row("声音提醒", true, cx))
                    .child(self.render_setting_row("桌面通知", true, cx))
                    .child(self.render_setting_row("显示消息详情", false, cx))
                    .child(self.render_setting_row("通知免打扰", false, cx)),
            )
    }

    fn render_plugins(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .gap_6()
            .child(
                div()
                    .text_lg()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(theme.foreground)
                    .child("插件"),
            )
            .child(
                v_flex().gap_3().child(
                    div()
                        .text_sm()
                        .text_color(theme.muted_foreground)
                        .child("暂无可用插件"),
                ),
            )
    }

    fn render_about(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);

        v_flex()
            .gap_6()
            .child(
                div()
                    .text_lg()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(theme.foreground)
                    .child("关于微信"),
            )
            .child(
                v_flex()
                    .gap_4()
                    .items_center()
                    .child(
                        div()
                            .w(crate::ui::constants::about_logo_size())
                            .h(crate::ui::constants::about_logo_size())
                            .rounded(crate::ui::constants::radius_lg())
                            .bg(weixin_colors.weixin_green)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(theme.primary_foreground)
                                    .child("微"),
                            ),
                    )
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(theme.foreground)
                            .child("微信 for Windows"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.muted_foreground)
                            .child("版本 3.9.10"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.muted_foreground)
                            .child("© 2011-2025 Tencent. All Rights Reserved."),
                    ),
            )
    }

    fn render_theme_setting(
        &self,
        current_mode: ThemeMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let foreground = theme.foreground;

        h_flex()
            .items_center()
            .justify_between()
            .py_2()
            .child(div().text_sm().text_color(foreground).child("外观"))
            .child(self.render_theme_button(current_mode, window, cx))
    }

    fn render_theme_button(
        &self,
        current_mode: ThemeMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let weixin_colors = Theme::weixin_colors(cx);

        Popover::new("theme-popover")
            .anchor(gpui::Corner::BottomLeft)
            .trigger(
                Button::new("theme-btn").xsmall().outline().child(
                    h_flex()
                        .items_center()
                        .gap_2()
                        .text_xs()
                        .child(match current_mode {
                            ThemeMode::Light => "浅色",
                            ThemeMode::Dark => "深色",
                        })
                        .child(
                            div()
                                .p(crate::ui::constants::icon_badge_padding_xs())
                                .rounded_sm()
                                .bg(weixin_colors.weixin_green)
                                .child(
                                    Icon::default()
                                        .path("arrow.svg")
                                        .text_color(gpui::rgb(0xffffff)),
                                ),
                        ),
                ),
            )
            .content(move |window, cx| {
                let theme_popover = cx.theme().popover;
                // 每次打开 Popover 时重置 hover 状态
                let light_hovered = cx.new(|_| false);
                let dark_hovered = cx.new(|_| false);

                cx.new(|cx| {
                    PopoverContent::new(window, cx, move |_, cx| {
                        let theme = cx.theme();

                        v_flex()
                            .w(crate::ui::constants::popover_width_sm())
                            .gap_0()
                            .py_2()
                            .text_color(theme.foreground)
                            .child({
                                let hovered = *light_hovered.read(cx);
                                let state = light_hovered.clone();
                                Self::render_static_theme_item(
                                    "theme-light",
                                    "浅色",
                                    hovered,
                                    move |is_hovering, cx| {
                                        state.update(cx, |s, _| *s = is_hovering);
                                    },
                                    cx.listener(|_, _, window, cx| {
                                        Theme::set_light(cx);
                                        cx.refresh_windows();
                                        window.push_notification("切换到浅色主题", cx);
                                        cx.emit(DismissEvent);
                                    }),
                                )
                            })
                            .child({
                                let hovered = *dark_hovered.read(cx);
                                let state = dark_hovered.clone();
                                Self::render_static_theme_item(
                                    "theme-dark",
                                    "深色",
                                    hovered,
                                    move |is_hovering, cx| {
                                        state.update(cx, |s, _| *s = is_hovering);
                                    },
                                    cx.listener(|_, _, window, cx| {
                                        Theme::set_dark(cx);
                                        cx.refresh_windows();
                                        window.push_notification("切换到深色主题", cx);
                                        cx.emit(DismissEvent);
                                    }),
                                )
                            })
                            .into_any()
                    })
                    .p_1()
                    .bg(theme_popover)
                    .rounded(crate::ui::constants::radius_md())
                    .shadow_md()
                })
            })
    }

    fn render_static_theme_item<FSet, L>(
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
        Self::render_static_select_item(id, label, hovered, set_hover, on_mouse_down)
    }

    fn render_static_select_item<FSet, L>(
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
                    .when(hovered, |this| this.text_color(gpui::white()))
                    .child(label),
            )
    }

    fn render_setting_row(
        &self,
        label: &'static str,
        enabled: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();

        h_flex()
            .items_center()
            .justify_between()
            .py_2()
            .child(div().text_sm().text_color(theme.foreground).child(label))
            .child(self.render_toggle(enabled))
    }

    fn render_shortcut_item(
        &self,
        label: &'static str,
        shortcut: &'static str,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();

        h_flex()
            .items_center()
            .justify_between()
            .py_2()
            .child(div().text_sm().text_color(theme.foreground).child(label))
            .child(
                div()
                    .px_3()
                    .py_1()
                    .rounded(crate::ui::constants::radius_sm())
                    .bg(theme.muted)
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.muted_foreground)
                            .child(shortcut),
                    ),
            )
    }

    fn render_static_language_item<FSet, L>(
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
        Self::render_static_select_item(id, label, hovered, set_hover, on_mouse_down)
    }

    fn render_static_font_item<FSet, L>(
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
        Self::render_static_select_item(id, label, hovered, set_hover, on_mouse_down)
    }

    fn render_language_button(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let current_language = self.current_language.clone();
        let weixin_colors = Theme::weixin_colors(cx);

        Popover::new("language-popover")
            .anchor(gpui::Corner::BottomRight)
            .trigger(
                Button::new("language-btn").xsmall().outline().child(
                    h_flex()
                        .items_center()
                        .gap_2()
                        .text_xs()
                        .child(current_language.clone())
                        .child(
                            div()
                                .p(crate::ui::constants::icon_badge_padding_xs())
                                .rounded_sm()
                                .bg(weixin_colors.weixin_green)
                                .child(
                                    Icon::default()
                                        .path("arrow.svg")
                                        .text_color(gpui::rgb(0xffffff)),
                                ),
                        ),
                ),
            )
            .content(move |window, cx| {
                let theme_popover = cx.theme().popover;
                // 每次打开 Popover 时重置 hover 状态
                let chinese_hovered = cx.new(|_| false);
                let traditional_hovered = cx.new(|_| false);
                let english_hovered = cx.new(|_| false);

                cx.new(|cx| {
                    PopoverContent::new(window, cx, move |_, cx| {
                        let theme = cx.theme();

                        v_flex()
                            .w(crate::ui::constants::popover_width_md())
                            .gap_0()
                            .py_2()
                            .text_color(theme.foreground)
                            .child({
                                let hovered = *chinese_hovered.read(cx);
                                let state = chinese_hovered.clone();
                                Self::render_static_language_item(
                                    "lang-chinese",
                                    "简体中文",
                                    hovered,
                                    move |is_hovering, cx| {
                                        state.update(cx, |s, _| *s = is_hovering);
                                    },
                                    cx.listener(|_, _, window, cx| {
                                        window.push_notification("语言切换功能开发中...", cx);
                                        cx.emit(DismissEvent);
                                    }),
                                )
                            })
                            .child({
                                let hovered = *traditional_hovered.read(cx);
                                let state = traditional_hovered.clone();
                                Self::render_static_language_item(
                                    "lang-traditional",
                                    "繁體中文",
                                    hovered,
                                    move |is_hovering, cx| {
                                        state.update(cx, |s, _| *s = is_hovering);
                                    },
                                    cx.listener(|_, _, window, cx| {
                                        window.push_notification("语言切换功能开发中...", cx);
                                        cx.emit(DismissEvent);
                                    }),
                                )
                            })
                            .child({
                                let hovered = *english_hovered.read(cx);
                                let state = english_hovered.clone();
                                Self::render_static_language_item(
                                    "lang-english",
                                    "English",
                                    hovered,
                                    move |is_hovering, cx| {
                                        state.update(cx, |s, _| *s = is_hovering);
                                    },
                                    cx.listener(|_, _, window, cx| {
                                        window.push_notification("语言切换功能开发中...", cx);
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
            })
    }

    fn render_font_size_button(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let current_size = self.current_font_size;
        let current_size_str = match current_size {
            FontSize::Small => "小",
            FontSize::Standard => "标准",
            FontSize::Large => "大",
        };
        let weixin_colors = Theme::weixin_colors(cx);

        Popover::new("font-size-popover")
            .anchor(gpui::Corner::BottomRight)
            .trigger(
                Button::new("font-size-btn").xsmall().outline().child(
                    h_flex()
                        .items_center()
                        .gap_2()
                        .text_xs()
                        .child(current_size_str)
                        .child(
                            div()
                                .p(px(1.5))
                                .rounded_sm()
                                .bg(weixin_colors.weixin_green)
                                .child(
                                    Icon::default()
                                        .path("arrow.svg")
                                        .text_color(gpui::rgb(0xffffff)),
                                ),
                        ),
                ),
            )
            .content(move |window, cx| {
                let theme_popover = cx.theme().popover;
                // 每次打开 Popover 时重置 hover 状态
                let small_hovered = cx.new(|_| false);
                let standard_hovered = cx.new(|_| false);
                let large_hovered = cx.new(|_| false);

                cx.new(|cx| {
                    PopoverContent::new(window, cx, move |_, cx| {
                        let theme = cx.theme();

                        v_flex()
                            .w(crate::ui::constants::popover_width_sm())
                            .gap_0()
                            .py_2()
                            .text_color(theme.foreground)
                            .child({
                                let hovered = *small_hovered.read(cx);
                                let state = small_hovered.clone();
                                Self::render_static_font_item(
                                    "font-small",
                                    "小",
                                    hovered,
                                    move |is_hovering, cx| {
                                        state.update(cx, |s, _| *s = is_hovering);
                                    },
                                    cx.listener(|_, _, window, cx| {
                                        window.push_notification("字体大小设置功能开发中...", cx);
                                        cx.emit(DismissEvent);
                                    }),
                                )
                            })
                            .child({
                                let hovered = *standard_hovered.read(cx);
                                let state = standard_hovered.clone();
                                Self::render_static_font_item(
                                    "font-standard",
                                    "标准",
                                    hovered,
                                    move |is_hovering, cx| {
                                        state.update(cx, |s, _| *s = is_hovering);
                                    },
                                    cx.listener(|_, _, window, cx| {
                                        window.push_notification("字体大小设置功能开发中...", cx);
                                        cx.emit(DismissEvent);
                                    }),
                                )
                            })
                            .child({
                                let hovered = *large_hovered.read(cx);
                                let state = large_hovered.clone();
                                Self::render_static_font_item(
                                    "font-large",
                                    "大",
                                    hovered,
                                    move |is_hovering, cx| {
                                        state.update(cx, |s, _| *s = is_hovering);
                                    },
                                    cx.listener(|_, _, window, cx| {
                                        window.push_notification("字体大小设置功能开发中...", cx);
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
            })
    }

    fn render_tab_item(
        &self,
        ix: usize,
        label: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let weixin_colors = Theme::weixin_colors(cx);
        let theme = cx.theme();
        let is_active = self.active_tab_ix == ix;
        let active_bg = weixin_colors.item_selected; // DEDEDE / 深色下为 3A3A3A
        let hover_bg = weixin_colors.item_hover; // EAEAEA / 深色下为 333333
        let transparent_bg = gpui::transparent_black();
        let text_color = theme.foreground;

        let tab_id = match ix {
            0 => "tab-0",
            1 => "tab-1",
            2 => "tab-2",
            3 => "tab-3",
            4 => "tab-4",
            5 => "tab-5",
            _ => "tab-unknown",
        };

        div()
            .id(tab_id)
            .w_full()
            .px_4()
            .py_3()
            .cursor_pointer()
            .rounded(px(4.))
            .bg(if is_active { active_bg } else { transparent_bg })
            .hover(move |s| if is_active { s } else { s.bg(hover_bg) })
            .on_click(cx.listener(move |this, _ev, _window, cx| {
                this.set_active_tab(ix, _window, cx);
            }))
            .child(div().text_sm().text_color(text_color).child(label))
    }

    fn render_toggle(&self, enabled: bool) -> impl IntoElement {
        let weixin_green = rgb(0x07c160);
        let toggle_off = rgba(0xccccccff);

        div()
            .w(px(40.))
            .h(px(20.))
            .rounded(px(10.))
            .cursor_pointer()
            .bg(if enabled { weixin_green } else { toggle_off })
            .flex()
            .items_center()
            .px(px(2.))
            .child(
                div()
                    .w(px(16.))
                    .h(px(16.))
                    .rounded(px(8.))
                    .bg(gpui::white())
                    .when(enabled, |this| this.ml_auto()),
            )
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let weixin_colors = Theme::weixin_colors(cx);
        let theme = cx.theme();
        let close_hover = rgb(0xe81123);
        let text_color = theme.foreground;
        let left_bg = weixin_colors.session_list_bg; // 左侧背景色 (F7F7F7)
        let right_bg = weixin_colors.chat_area_bg; // 右侧背景色 (EDEDED)
        let border_color = theme.border; // 分割线颜色

        h_flex()
            .size_full()
            .child(
                // 左侧区域（包括标题栏和导航栏）
                v_flex()
                    .w(crate::ui::constants::settings_sidebar_width())
                    .h_full()
                    .bg(left_bg)
                    .border_r_1()
                    .border_color(border_color)
                    .child(
                        // 左侧标题栏区域
                        div()
                            .window_control_area(WindowControlArea::Drag)
                            .w_full()
                            .h(px(48.))
                            .flex()
                            .items_center()
                            .px_4(),
                    )
                    .child(
                        // 导航栏
                        v_flex()
                            .flex_1()
                            .w_full()
                            .py_4()
                            .px_3()
                            .gap_1()
                            .child(self.render_tab_item(0, "账号与存储", cx))
                            .child(self.render_tab_item(1, "通用", cx))
                            .child(self.render_tab_item(2, "快捷键", cx))
                            .child(self.render_tab_item(3, "通知", cx))
                            .child(self.render_tab_item(4, "插件", cx))
                            .child(self.render_tab_item(5, "关于微信", cx)),
                    ),
            )
            .child(
                // 右侧区域（包括标题栏和内容）
                v_flex()
                    .flex_1()
                    .h_full()
                    .bg(right_bg)
                    .child(
                        // 右侧标题栏
                        h_flex()
                            .w_full()
                            .h(crate::ui::constants::settings_title_height())
                            .items_center()
                            .child(
                                // 可拖动区域
                                div()
                                    .window_control_area(WindowControlArea::Drag)
                                    .flex_1()
                                    .h_full(),
                            )
                            .child(
                                // 关闭按钮
                                div()
                                    .id("settings-close-btn")
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .h_full()
                                    .w(crate::ui::constants::settings_close_button_width())
                                    .window_control_area(WindowControlArea::Close)
                                    .cursor_pointer()
                                    .hover(move |s| s.bg(close_hover).text_color(gpui::white()))
                                    .child(
                                        Icon::default()
                                            .path("window-close.svg")
                                            .text_color(text_color)
                                            .small(),
                                    ),
                            ),
                    )
                    .child(
                        // 内容区域
                        v_flex()
                            .flex_1()
                            .w_full()
                            .p_6()
                            .overflow_hidden()
                            .child(self.render_content(_window, cx)),
                    ),
            )
    }
}
