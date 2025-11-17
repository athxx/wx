use crate::app::state::Preferences;
use crate::ui::theme::{Theme, ThemeMode};
use crate::ui::widgets::setting_card;
use gpui::{DismissEvent, EventEmitter, *};
use gpui_component::{
    avatar::Avatar,
    button::Button,
    h_flex,
    popover::Popover,
    slider::{Slider, SliderEvent, SliderState},
    v_flex, ActiveTheme, Icon, Selectable, Sizable, WindowExt,
};
use std::sync::atomic::{AtomicBool, Ordering};

pub static SETTINGS_WINDOW_OPEN: AtomicBool = AtomicBool::new(false);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeHover {
    None,
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LanguageHover {
    None,
    ChineseSimplified,
    ChineseTraditional,
    English,
}

pub struct SettingsWindow {
    active_tab_ix: usize,
    current_language: String,
    #[allow(dead_code)]
    current_font_size: FontSize,
    /// Slider state for controlling global font size.
    font_slider: Entity<SliderState>,
    /// Subscription to listen slider changes and update global font size.
    #[allow(dead_code)]
    font_slider_subscription: gpui::Subscription,
    /// Current font size value used by the slider (in px).
    current_font_size_value: f32,
    _theme_observer: Option<gpui::Subscription>,
    theme_hover: ThemeHover,
    language_hover: LanguageHover,
}

impl EventEmitter<DismissEvent> for SettingsWindow {}

impl SettingsWindow {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let theme_observer = cx.observe_global::<Theme>(|_this, cx| {
            cx.notify();
        });

        // 字体大小 Slider：0..7 共 8 档，根据当前全局字体大小计算初始档位
        let current_font_size: f32 = cx.theme().font_size.into();
        let initial_index = match current_font_size.round() as i32 {
            14 => 0.0, // 小
            16 => 1.0, // 标准
            17 => 2.0,
            18 => 3.0,
            19 => 4.0,
            20 => 5.0,
            21 => 6.0,
            22 => 7.0, // 大
            _ => 1.0,
        };
        let initial_font_size = current_font_size;

        let font_slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(7.0)
                .step(1.0)
                .default_value(initial_index)
        });

        // Slider 变化时，映射到具体像素并更新全局字体大小
        let font_slider_subscription =
            cx.subscribe(&font_slider, |this, _, event: &SliderEvent, cx| {
                let SliderEvent::Change(value) = event;
                let idx = value.start().round().clamp(0.0, 7.0) as i32;
                let size = match idx {
                    0 => 14.0, // 小
                    1 => 16.0, // 标准
                    2 => 17.0,
                    3 => 18.0,
                    4 => 19.0,
                    5 => 20.0,
                    6 => 21.0,
                    7 => 22.0, // 大
                    _ => 16.0,
                };
                this.current_font_size_value = size;
                gpui_component::Theme::global_mut(cx).font_size = px(size);
                // 保存到偏好 JSON
                Preferences::save_from_app(cx);
                cx.refresh_windows();
            });

        Self {
            active_tab_ix: 1,
            current_language: "简体中文".to_string(),
            current_font_size: FontSize::Standard,
            font_slider,
            font_slider_subscription,
            current_font_size_value: initial_font_size,
            _theme_observer: Some(theme_observer),
            theme_hover: ThemeHover::None,
            language_hover: LanguageHover::None,
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
        let foreground = theme.foreground;
        let muted = theme.muted_foreground;

        // 顶部账号信息 + 自动登录/保留聊天记录卡片
        let account_card = {
            let avatar_and_info = h_flex()
                .items_center()
                .gap_3()
                .child(
                    // 与标题栏 render_user_avatar 中一致的头像风格和尺寸
                    Avatar::new()
                        .w(crate::ui::constants::title_avatar_size())
                        .h(crate::ui::constants::title_avatar_size())
                        .rounded(crate::ui::constants::radius_md())
                        .src(crate::ui::avatar::avatar_for_key("self")),
                )
                .child(
                    v_flex()
                        .gap_1()
                        .child(div().text_sm().text_color(foreground).child("@@@"))
                        .child(div().text_xs().text_color(muted).child("H1548772930")),
                );

            let header_row = h_flex()
                .items_center()
                .justify_between()
                .child(avatar_and_info)
                .child(
                    Button::new("settings-account-logout")
                        .small()
                        .bg(gpui::rgb(0xEAEAEA))
                        .hover(|s| s.bg(gpui::rgb(0xE4E4E4)))
                        .child("退出登录"),
                );

            let auto_login_row = h_flex()
                .items_center()
                .justify_between()
                .py_2()
                .child(
                    v_flex()
                        .gap_1()
                        .child(div().text_sm().text_color(foreground).child("自动登录"))
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted)
                                .child("在本机登录微信将无需手机确认。"),
                        ),
                )
                .child(crate::ui::widgets::toggle::toggle_small(cx, true));

            let keep_history_row = h_flex()
                .items_center()
                .justify_between()
                .py_2()
                .child(
                    v_flex()
                        .gap_1()
                        .child(div().text_sm().text_color(foreground).child("保留聊天记录"))
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted)
                                .child("退出登录时保留本机聊天记录。"),
                        ),
                )
                .child(crate::ui::widgets::toggle::toggle_small(cx, true));

            crate::ui::widgets::setting_card::card(
                cx,
                v_flex()
                    .gap_0()
                    .child(header_row)
                    .child(crate::ui::widgets::setting_card::divider(cx))
                    .child(auto_login_row)
                    .child(crate::ui::widgets::setting_card::divider(cx))
                    .child(keep_history_row),
            )
        };

        // 存储空间 + 存储位置 + 自动下载 + 清空聊天记录卡片
        let storage_card = {
            let storage_space_row = h_flex()
                .items_center()
                .justify_between()
                .py_4()
                .child(div().text_sm().text_color(foreground).child("存储空间"))
                .child(
                    Button::new("settings-storage-manage")
                        .small()
                        .bg(gpui::rgb(0xEAEAEA))
                        .hover(|s| s.bg(gpui::rgb(0xE4E4E4)))
                        .child("管理"),
                );

            let path_row = v_flex()
                .gap_1()
                .py_4()
                .child(div().text_sm().text_color(foreground).child("存储位置"))
                .child(
                    h_flex()
                        .items_center()
                        .justify_between()
                        .child(div().text_xs().text_color(muted).child("D\\wxwechat_files"))
                        .child(
                            Button::new("settings-storage-change")
                                .small()
                                .bg(gpui::rgb(0xEAEAEA))
                                .hover(|s| s.bg(gpui::rgb(0xE4E4E4)))
                                .child("更改"),
                        ),
                );

            let auto_download_row = h_flex()
                .items_center()
                .justify_between()
                .py_4()
                .child(
                    h_flex()
                        .items_center()
                        .gap_2()
                        .child(div().text_sm().text_color(foreground).child("自动下载小于"))
                        .child(
                            div()
                                .w(px(40.))
                                .h(px(24.))
                                .rounded(crate::ui::constants::radius_sm())
                                .bg(theme.secondary)
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(div().text_sm().text_color(foreground).child("20")),
                        )
                        .child(div().text_sm().text_color(foreground).child("MB的文件")),
                )
                .child(crate::ui::widgets::toggle::toggle_small(cx, true));

            let clear_button_row = h_flex().justify_end().py_4().child(
                Button::new("settings-clear-messages")
                    .small()
                    .bg(gpui::rgb(0xEAEAEA))
                    .hover(|s| s.bg(gpui::rgb(0xE4E4E4)))
                    .label("清空全部聊天记录"),
            );

            crate::ui::widgets::setting_card::card(
                cx,
                v_flex()
                    .gap_0()
                    .child(storage_space_row)
                    .child(crate::ui::widgets::setting_card::divider(cx))
                    .child(path_row)
                    .child(crate::ui::widgets::setting_card::divider(cx))
                    .child(auto_download_row)
                    .child(crate::ui::widgets::setting_card::divider(cx))
                    .child(clear_button_row),
            )
        };

        v_flex().gap_6().child(account_card).child(storage_card)
    }

    fn render_general_settings(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme_mode = cx.theme().mode;
        let theme = cx.theme();
        let foreground = theme.foreground;

        let language_row = {
            let label = div().text_sm().text_color(foreground).child("语言");
            let btn = self.render_language_button(window, cx);
            h_flex()
                .items_center()
                .justify_between()
                .py_2()
                .child(label)
                .child(btn)
        };

        let appearance_card_content = {
            let theme_row = self.render_theme_setting(theme_mode, window, cx);
            let font_row = {
                let label = div().text_sm().text_color(foreground).child("字体大小");
                let slider = self.render_font_size_slider(window, cx);
                h_flex()
                    .items_center()
                    .py_2()
                    .child(label)
                    .child(h_flex().flex_1().justify_end().child(slider))
            };
            v_flex()
                .gap_0()
                .child(theme_row)
                .child(setting_card::divider(cx))
                .child(font_row)
        };

        v_flex()
            .gap_6()
            .child(setting_card::card(cx, language_row))
            .child(setting_card::card(cx, appearance_card_content))
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

    /// 通用设置中的下拉按钮触发器（语言 / 主题 / 字体），统一使用同一风格。
    fn general_select_trigger_button(
        id: &'static str,
        label: String,
        cx: &mut Context<Self>,
    ) -> Button {
        let weixin_colors = Theme::weixin_colors(cx);

        Button::new(id)
            .selected(false)
            .xsmall()
            .outline()
            .h(px(26.))
            .w(px(90.))
            .child(
                h_flex()
                    .items_center()
                    .gap_2()
                    .text_size(px(11.))
                    .child(label)
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
            )
    }

    fn render_theme_button(
        &self,
        current_mode: ThemeMode,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let label = match current_mode {
            ThemeMode::Light => "浅色模式".to_string(),
            ThemeMode::Dark => "深色模式".to_string(),
        };

        let settings = cx.entity();

        Popover::new("theme-popover")
            .appearance(false)
            .anchor(gpui::Corner::BottomLeft)
            .trigger(Self::general_select_trigger_button("theme-btn", label, cx))
            .content(move |_, _window, cx| {
                let theme = cx.theme();

                let theme_hover = settings.read(cx).theme_hover;
                let light_hovered = matches!(theme_hover, ThemeHover::Light);
                let dark_hovered = matches!(theme_hover, ThemeHover::Dark);

                let settings_for_light = settings.clone();
                let set_light_hover = move |is_hovering: bool, cx: &mut App| {
                    _ = settings_for_light.update(cx, |this: &mut SettingsWindow, cx| {
                        this.theme_hover = if is_hovering {
                            ThemeHover::Light
                        } else if matches!(this.theme_hover, ThemeHover::Light) {
                            ThemeHover::None
                        } else {
                            this.theme_hover
                        };
                        cx.notify();
                    });
                };

                let settings_for_dark = settings.clone();
                let set_dark_hover = move |is_hovering: bool, cx: &mut App| {
                    _ = settings_for_dark.update(cx, |this: &mut SettingsWindow, cx| {
                        this.theme_hover = if is_hovering {
                            ThemeHover::Dark
                        } else if matches!(this.theme_hover, ThemeHover::Dark) {
                            ThemeHover::None
                        } else {
                            this.theme_hover
                        };
                        cx.notify();
                    });
                };

                let settings_for_light_click = settings.clone();
                let settings_for_dark_click = settings.clone();

                v_flex()
                    .w(crate::ui::constants::popover_width_sm())
                    .gap_0()
                    .py_2()
                    .bg(theme.popover)
                    .p_1()
                    .rounded(crate::ui::constants::radius_md())
                    .shadow_md()
                    .child(Self::render_static_theme_item(
                        "theme-light",
                        "浅色模式",
                        light_hovered,
                        set_light_hover,
                        cx.listener(move |_, _, window, cx| {
                            Theme::set_light(cx);
                            Preferences::save_from_app(cx);
                            cx.refresh_windows();
                            window.push_notification("切换到浅色主题", cx);
                            cx.emit(gpui::DismissEvent);
                            _ = settings_for_light_click.update(
                                cx,
                                |this: &mut SettingsWindow, cx| {
                                    this.theme_hover = ThemeHover::None;
                                    cx.notify();
                                },
                            );
                        }),
                    ))
                    .child(Self::render_static_theme_item(
                        "theme-dark",
                        "深色模式",
                        dark_hovered,
                        set_dark_hover,
                        cx.listener(move |_, _, window, cx| {
                            Theme::set_dark(cx);
                            Preferences::save_from_app(cx);
                            cx.refresh_windows();
                            window.push_notification("切换到深色主题", cx);
                            cx.emit(gpui::DismissEvent);
                            _ = settings_for_dark_click.update(
                                cx,
                                |this: &mut SettingsWindow, cx| {
                                    this.theme_hover = ThemeHover::None;
                                    cx.notify();
                                },
                            );
                        }),
                    ))
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
        crate::ui::widgets::toolbar::hover_menu_item::hover_menu_item_compact(
            id,
            label,
            hovered,
            set_hover,
            on_mouse_down,
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
            .child(crate::ui::widgets::toggle::toggle(cx, enabled))
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

    #[allow(dead_code)]
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
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let current_language = self.current_language.clone();
        let settings = cx.entity();

        Popover::new("language-popover")
            .appearance(false)
            .anchor(gpui::Corner::BottomRight)
            .trigger(Self::general_select_trigger_button(
                "language-btn",
                current_language.clone(),
                cx,
            ))
            .content(move |_, _window, cx| {
                let theme = cx.theme();

                let language_hover = settings.read(cx).language_hover;
                let chinese_hovered = matches!(language_hover, LanguageHover::ChineseSimplified);
                let traditional_hovered =
                    matches!(language_hover, LanguageHover::ChineseTraditional);
                let english_hovered = matches!(language_hover, LanguageHover::English);

                let settings_for_chinese = settings.clone();
                let set_chinese_hover = move |is_hovering: bool, cx: &mut App| {
                    _ = settings_for_chinese.update(cx, |this: &mut SettingsWindow, cx| {
                        this.language_hover = if is_hovering {
                            LanguageHover::ChineseSimplified
                        } else if matches!(this.language_hover, LanguageHover::ChineseSimplified) {
                            LanguageHover::None
                        } else {
                            this.language_hover
                        };
                        cx.notify();
                    });
                };

                let settings_for_traditional = settings.clone();
                let set_traditional_hover = move |is_hovering: bool, cx: &mut App| {
                    _ = settings_for_traditional.update(cx, |this: &mut SettingsWindow, cx| {
                        this.language_hover = if is_hovering {
                            LanguageHover::ChineseTraditional
                        } else if matches!(this.language_hover, LanguageHover::ChineseTraditional) {
                            LanguageHover::None
                        } else {
                            this.language_hover
                        };
                        cx.notify();
                    });
                };

                let settings_for_english = settings.clone();
                let set_english_hover = move |is_hovering: bool, cx: &mut App| {
                    _ = settings_for_english.update(cx, |this: &mut SettingsWindow, cx| {
                        this.language_hover = if is_hovering {
                            LanguageHover::English
                        } else if matches!(this.language_hover, LanguageHover::English) {
                            LanguageHover::None
                        } else {
                            this.language_hover
                        };
                        cx.notify();
                    });
                };

                let settings_for_chinese_click = settings.clone();
                let settings_for_traditional_click = settings.clone();
                let settings_for_english_click = settings.clone();

                v_flex()
                    .w(crate::ui::constants::popover_width_md())
                    .gap_0()
                    .py_2()
                    .bg(theme.popover)
                    .p_1()
                    .rounded(crate::ui::constants::radius_md())
                    .shadow_md()
                    .child(Self::render_static_language_item(
                        "lang-chinese",
                        "简体中文",
                        chinese_hovered,
                        set_chinese_hover,
                        cx.listener(move |_, _, window, cx| {
                            window.push_notification("语言切换功能开发中...", cx);
                            cx.emit(gpui::DismissEvent);
                            _ = settings_for_chinese_click.update(
                                cx,
                                |this: &mut SettingsWindow, cx| {
                                    this.language_hover = LanguageHover::None;
                                    cx.notify();
                                },
                            );
                        }),
                    ))
                    .child(Self::render_static_language_item(
                        "lang-traditional",
                        "繁體中文",
                        traditional_hovered,
                        set_traditional_hover,
                        cx.listener(move |_, _, window, cx| {
                            window.push_notification("语言切换功能开发中...", cx);
                            cx.emit(gpui::DismissEvent);
                            _ = settings_for_traditional_click.update(
                                cx,
                                |this: &mut SettingsWindow, cx| {
                                    this.language_hover = LanguageHover::None;
                                    cx.notify();
                                },
                            );
                        }),
                    ))
                    .child(Self::render_static_language_item(
                        "lang-english",
                        "English",
                        english_hovered,
                        set_english_hover,
                        cx.listener(move |_, _, window, cx| {
                            window.push_notification("语言切换功能开发中...", cx);
                            cx.emit(gpui::DismissEvent);
                            _ = settings_for_english_click.update(
                                cx,
                                |this: &mut SettingsWindow, cx| {
                                    this.language_hover = LanguageHover::None;
                                    cx.notify();
                                },
                            );
                        }),
                    ))
            })
    }

    fn render_font_size_slider(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);

        v_flex()
            .gap_1()
            .w(px(160.))
            .child(
                Slider::new(&self.font_slider)
                    .bg(weixin_colors.weixin_green)
                    .text_color(theme.slider_thumb)
                    .rounded(crate::ui::constants::radius_sm()),
            )
            .child(
                h_flex()
                    .justify_between()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .child("小") // 第 1 节：小
                    .child("标准") // 第 2 节：标准
                    .child("") // 第 3 节
                    .child("") // 第 4 节
                    .child("") // 第 5 节
                    .child("") // 第 6 节
                    .child("") // 第 7 节
                    .child("大"), // 第 8 节：大
            )
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
        let active_bg = weixin_colors.item_selected;
        let hover_bg = weixin_colors.item_hover;
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

        // 为每个设置页 tab 配置一个图标，图标文件位于 assets/setting 目录下。
        let icon_path = match ix {
            // 账号与存储
            0 => "setting/user.svg",
            // 通用
            1 => "setting/setting.svg",
            // 快捷键
            2 => "setting/arrow-up9.svg",
            // 通知
            3 => "setting/notifications.svg",
            // 插件
            4 => "setting/risk.svg",
            // 关于微信
            5 => "setting/about.svg",
            _ => "setting/setting.svg",
        };

        let content = h_flex()
            .items_center()
            .gap_2()
            .child(
                Icon::default()
                    .path(icon_path)
                    .w(crate::ui::constants::icon_sm())
                    .h(crate::ui::constants::icon_sm())
                    .text_color(text_color),
            )
            .child(div().text_sm().text_color(text_color).child(label));

        div()
            .id(tab_id)
            .w_full()
            .px_4()
            .py_3()
            .cursor_pointer()
            .rounded(crate::ui::constants::radius_sm())
            .bg(if is_active { active_bg } else { transparent_bg })
            .hover(move |s| if is_active { s } else { s.bg(hover_bg) })
            .on_click(cx.listener(move |this, _ev, _window, cx| {
                this.set_active_tab(ix, _window, cx);
            }))
            .child(content)
    }
}

impl Drop for SettingsWindow {
    fn drop(&mut self) {
        SETTINGS_WINDOW_OPEN.store(false, Ordering::SeqCst);
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 固定设置窗口的 rem 基础字号，使其不随全局字体缩放变化
        window.set_rem_size(px(16.0));

        let weixin_colors = Theme::weixin_colors(cx);
        let theme = cx.theme();
        let close_hover = rgb(0xe81123);
        let text_color = theme.foreground;
        let left_bg = weixin_colors.session_list_bg;
        let right_bg = weixin_colors.chat_area_bg;
        let border_color = theme.border;

        h_flex()
            .size_full()
            .child(
                v_flex()
                    .w(crate::ui::constants::settings_sidebar_width())
                    .h_full()
                    .bg(left_bg)
                    .border_r_1()
                    .border_color(border_color)
                    .child(
                        div()
                            .window_control_area(WindowControlArea::Drag)
                            .w_full()
                            .h(crate::ui::constants::settings_title_height())
                            .flex()
                            .items_center()
                            .px_4(),
                    )
                    .child(
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
                v_flex()
                    .flex_1()
                    .h_full()
                    .bg(right_bg)
                    .child(
                        h_flex()
                            .w_full()
                            .h(crate::ui::constants::settings_title_height())
                            .items_center()
                            .child(
                                div()
                                    .window_control_area(WindowControlArea::Drag)
                                    .flex_1()
                                    .h_full(),
                            )
                            .child(
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
                        v_flex()
                            .flex_1()
                            .w_full()
                            .p_6()
                            .overflow_hidden()
                            .child(self.render_content(window, cx)),
                    ),
            )
    }
}
