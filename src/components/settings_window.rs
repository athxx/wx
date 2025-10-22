use crate::theme::{Theme, ThemeMode, WeixinThemeColors};
use gpui::{prelude::FluentBuilder, *};
use gpui_component::{h_flex, v_flex, ActiveTheme, Icon, Sizable};

#[derive(Debug, Clone, Copy, PartialEq)]
enum SettingsTab {
    AccountAndStorage,
    General,
    Shortcuts,
    Notifications,
    Plugins,
    About,
}

pub struct SettingsWindow {
    active_tab: SettingsTab,
    _theme_observer: Option<gpui::Subscription>,
}

impl SettingsWindow {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // 订阅全局主题变化
        let theme_observer = cx.observe_global::<Theme>(|_this, cx| {
            cx.notify();
        });

        Self {
            active_tab: SettingsTab::General,
            _theme_observer: Some(theme_observer),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn render_sidebar_item(
        &self,
        tab: SettingsTab,
        label: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = self.active_tab == tab;
        let theme = cx.theme();

        let tab_id = match tab {
            SettingsTab::AccountAndStorage => "tab-account",
            SettingsTab::General => "tab-general",
            SettingsTab::Shortcuts => "tab-shortcuts",
            SettingsTab::Notifications => "tab-notifications",
            SettingsTab::Plugins => "tab-plugins",
            SettingsTab::About => "tab-about",
        };

        div()
            .id(tab_id)
            .w_full()
            .px_4()
            .py_3()
            .cursor_pointer()
            .rounded(px(4.))
            .bg(if is_active {
                theme.secondary
            } else {
                theme.transparent
            })
            .hover(move |s| {
                if is_active {
                    s
                } else {
                    s.bg(theme.secondary_hover)
                }
            })
            .on_click(cx.listener(move |this, _ev, _window, cx| {
                this.active_tab = tab;
                cx.notify();
            }))
            .child(div().text_sm().text_color(theme.foreground).child(label))
    }

    fn render_content(&self, cx: &mut Context<Self>) -> gpui::AnyElement {
        match self.active_tab {
            SettingsTab::AccountAndStorage => {
                self.render_account_and_storage(cx).into_any_element()
            }
            SettingsTab::General => self.render_general_settings(cx).into_any_element(),
            SettingsTab::Shortcuts => self.render_shortcuts(cx).into_any_element(),
            SettingsTab::Notifications => self.render_notifications(cx).into_any_element(),
            SettingsTab::Plugins => self.render_plugins(cx).into_any_element(),
            SettingsTab::About => self.render_about(cx).into_any_element(),
        }
    }

    fn render_account_and_storage(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);

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
                                            .w(px(50.))
                                            .h(px(50.))
                                            .rounded(px(4.))
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

    fn render_general_settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_mode = cx.theme().mode;
        let foreground = cx.theme().foreground;
        let secondary = cx.theme().secondary;
        let secondary_hover = cx.theme().secondary_hover;
        let muted_foreground = cx.theme().muted_foreground;
        let weixin_colors = Theme::weixin_colors(cx);

        v_flex()
            .gap_6()
            .child(
                div()
                    .text_lg()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(foreground)
                    .child("通用"),
            )
            .child(
                v_flex()
                    .gap_4()
                    .child(self.render_setting_row("开机时自动启动微信", true, cx))
                    .child(self.render_setting_row("登录后自动打开文件传输助手", false, cx))
                    .child(self.render_setting_row("保持在其他窗口前端", false, cx))
                    .child(self.render_setting_row("使用系统默认浏览器打开网页", true, cx)),
            )
            .child(self.render_theme_setting(theme_mode, cx))
            .child(
                v_flex()
                    .gap_3()
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(foreground)
                            .child("语言"),
                    )
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(muted_foreground)
                                    .child("简体中文"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(4.))
                                    .bg(secondary)
                                    .cursor_pointer()
                                    .hover(move |s| s.bg(secondary_hover))
                                    .child(div().text_xs().text_color(foreground).child("更改")),
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
                            .w(px(80.))
                            .h(px(80.))
                            .rounded(px(8.))
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
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .gap_3()
            .child(
                div()
                    .text_base()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(theme.foreground)
                    .child("外观"),
            )
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .py_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.foreground)
                            .child("主题模式"),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(self.render_theme_button(
                                ThemeMode::Light,
                                "浅色",
                                current_mode,
                                cx,
                            ))
                            .child(self.render_theme_button(
                                ThemeMode::Dark,
                                "深色",
                                current_mode,
                                cx,
                            )),
                    ),
            )
    }

    fn render_theme_button(
        &self,
        mode: ThemeMode,
        label: &'static str,
        current_mode: ThemeMode,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = current_mode == mode;
        let btn_id = match mode {
            ThemeMode::Light => "theme-btn-light",
            ThemeMode::Dark => "theme-btn-dark",
        };
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);

        div()
            .id(btn_id)
            .px_3()
            .py_1()
            .rounded(px(4.))
            .cursor_pointer()
            .when(is_active, |this| {
                this.bg(weixin_colors.weixin_green)
                    .text_color(theme.primary_foreground)
            })
            .when(!is_active, |this| {
                this.bg(theme.secondary).text_color(theme.foreground)
            })
            .hover(|s| {
                s.bg(weixin_colors.weixin_green_hover)
                    .text_color(theme.primary_foreground)
            })
            .on_click(cx.listener(move |_this, _ev, _window, cx| {
                match mode {
                    ThemeMode::Light => Theme::set_light(cx),
                    ThemeMode::Dark => Theme::set_dark(cx),
                }
                // 通知所有窗口主题已更改
                cx.refresh_windows();
            }))
            .child(div().text_xs().child(label))
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
                div().px_3().py_1().rounded(px(4.)).bg(theme.muted).child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child(shortcut),
                ),
            )
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
        let theme = cx.theme();
        let panel_bg = theme.secondary;
        let close_hover = rgb(0xe81123);
        let text_color = theme.foreground;
        let background = theme.background;

        v_flex()
            .size_full()
            .bg(panel_bg)
            .child(
                // 自定义标题栏
                h_flex()
                    .w_full()
                    .h(px(48.))
                    .bg(panel_bg)
                    .items_center()
                    .child(
                        // 可拖动区域
                        div()
                            .window_control_area(WindowControlArea::Drag)
                            .flex_1()
                            .h_full()
                            .flex()
                            .items_center()
                            .px_4(),
                    )
                    .child(
                        // 关闭按钮
                        div()
                            .id("settings-close-btn")
                            .flex()
                            .items_center()
                            .justify_center()
                            .h_full()
                            .w(px(48.))
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
                h_flex()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(
                        // 左侧导航栏
                        v_flex()
                            .w(px(180.))
                            .h_full()
                            .bg(panel_bg)
                            .py_4()
                            .gap_1()
                            .child(self.render_sidebar_item(
                                SettingsTab::AccountAndStorage,
                                "账号与存储",
                                cx,
                            ))
                            .child(self.render_sidebar_item(SettingsTab::General, "通用", cx))
                            .child(self.render_sidebar_item(SettingsTab::Shortcuts, "快捷键", cx))
                            .child(self.render_sidebar_item(SettingsTab::Notifications, "通知", cx))
                            .child(self.render_sidebar_item(SettingsTab::Plugins, "插件", cx))
                            .child(self.render_sidebar_item(SettingsTab::About, "关于微信", cx)),
                    )
                    .child(
                        // 右侧内容区
                        v_flex()
                            .flex_1()
                            .h_full()
                            .bg(background)
                            .p_6()
                            .overflow_hidden()
                            .child(self.render_content(cx)),
                    ),
            )
    }
}
