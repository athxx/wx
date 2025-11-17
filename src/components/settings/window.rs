use crate::ui::theme::{Theme, ThemeMode};
use crate::ui::widgets::setting_card;
use crate::app::state::Preferences;
use gpui::{DismissEvent, EventEmitter, *};
use gpui_component::{
    button::Button,
    h_flex,
    popover::{Popover, PopoverContent},
    slider::{Slider, SliderEvent, SliderState},
    v_flex, ActiveTheme, ContextModal, Icon, Selectable, Sizable,
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

pub struct SettingsWindow {
    active_tab_ix: usize,
    current_language: String,
    current_font_size: FontSize,
    /// Slider state for controlling global font size.
    font_slider: Entity<SliderState>,
    /// Subscription to listen slider changes and update global font size.
    font_slider_subscription: gpui::Subscription,
    /// Current font size value used by the slider (in px).
    current_font_size_value: f32,
    _theme_observer: Option<gpui::Subscription>,
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
        let font_slider_subscription = cx.subscribe(&font_slider, |this, _, event: &SliderEvent, cx| {
            if let SliderEvent::Change(value) = event {
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
            }
        });

        Self {
            active_tab_ix: 1,
            current_language: "简体中文".to_string(),
            current_font_size: FontSize::Standard,
            font_slider,
            font_slider_subscription,
            current_font_size_value: initial_font_size,
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
                            .child(crate::ui::widgets::setting_card::section_title(
                                cx,
                                "账号信息",
                            ))
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
                            .child(crate::ui::widgets::setting_card::section_title(
                                cx,
                                "文件存储",
                            ))
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
                                                    .rounded(crate::ui::constants::radius_sm())
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
                    .child(
                        h_flex()
                            .flex_1()
                            .justify_end()
                            .child(slider),
                    )
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

        Popover::new("theme-popover")
            .anchor(gpui::Corner::BottomLeft)
            .trigger(Self::general_select_trigger_button("theme-btn", label, cx))
            .content(move |window, cx| {
                let theme_popover = cx.theme().popover;
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
                                    "浅色模式",
                                    hovered,
                                    move |is_hovering, cx| {
                                        state.update(cx, |s, _| *s = is_hovering);
                                    },
                                    cx.listener(|_, _, window, cx| {
                                        Theme::set_light(cx);
                                        // 保存主题偏好
                                        Preferences::save_from_app(cx);
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
                                    "深色模式",
                                    hovered,
                                    move |is_hovering, cx| {
                                        state.update(cx, |s, _| *s = is_hovering);
                                    },
                                    cx.listener(|_, _, window, cx| {
                                        Theme::set_dark(cx);
                                        // 保存主题偏好
                                        Preferences::save_from_app(cx);
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
        crate::ui::widgets::toolbar::hover_menu_item::hover_menu_item(
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

        Popover::new("language-popover")
            .anchor(gpui::Corner::BottomRight)
            .trigger(Self::general_select_trigger_button(
                "language-btn",
                current_language.clone(),
                cx,
            ))
            .content(move |window, cx| {
                let theme_popover = cx.theme().popover;
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
                    .rounded(crate::ui::constants::radius_md())
                    .shadow_md()
                })
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
                    .child("小")      // 第 1 节：小
                    .child("标准")    // 第 2 节：标准
                    .child("")       // 第 3 节
                    .child("")       // 第 4 节
                    .child("")       // 第 5 节
                    .child("")       // 第 6 节
                    .child("")       // 第 7 节
                    .child("大"),     // 第 8 节：大
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
            .child(div().text_sm().text_color(text_color).child(label))
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
