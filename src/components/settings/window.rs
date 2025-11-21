use crate::app::config::Preferences;
use crate::ui::theme::{Theme, ThemeMode};
use crate::ui::composites::setting_card;
use gpui::{DismissEvent, EventEmitter, *};
use gpui_component::{
    button::{Button, ButtonCustomVariant, ButtonVariants as _},
    h_flex,
    input::{InputEvent, InputState},
    popover::Popover,
    slider::{Slider, SliderEvent, SliderState},
    switch::Switch,
    v_flex, ActiveTheme, Icon, Sizable, StyledExt as _, WindowExt, Size
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const SHORTCUT_INPUT_PLACEHOLDER: &str = "按下快捷键";
const SHORTCUT_SCREENSHOT_DEFAULT: &str = "Ctrl + Alt + A";
const SHORTCUT_LOCK_DEFAULT: &str = "Ctrl + Alt + L";
const SHORTCUT_TOGGLE_DEFAULT: &str = "Ctrl + Alt + W";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TranslateLanguageHover {
    None,
    SimplifiedChinese,
    TraditionalChinese,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShortcutSendHover {
    None,
    Enter,
    CtrlEnter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ShortcutInputField {
    Screenshot,
    Lock,
    ToggleWindow,
}

pub struct SettingsWindow {
    active_tab_ix: usize,
    current_language: String,
    root_focus_handle: gpui::FocusHandle,

    /// Slider state for controlling global font size.
    font_slider: Entity<SliderState>,
    /// Subscription to listen slider changes and update global font size.
    _font_slider_subscription: gpui::Subscription,
    /// Current font size value used by the slider (in px).
    current_font_size_value: f32,
    _theme_observer: Option<gpui::Subscription>,
    theme_hover: ThemeHover,
    language_hover: LanguageHover,
    translate_language_selection: String,
    translate_language_hover: TranslateLanguageHover,
    auto_download_limit_input: Entity<InputState>,
    auto_download_input_focused: bool,
    _auto_download_input_subscription: gpui::Subscription,
    shortcut_send_selection: String,
    shortcut_send_hover: ShortcutSendHover,
    shortcut_screenshot_input: Entity<InputState>,
    shortcut_lock_input: Entity<InputState>,
    shortcut_toggle_input: Entity<InputState>,
    shortcut_display_texts: HashMap<ShortcutInputField, String>,
    shortcut_input_widths: HashMap<ShortcutInputField, Pixels>,
    focused_shortcut_input: Option<ShortcutInputField>,
    _shortcut_input_subscriptions: Vec<gpui::Subscription>,
}

impl EventEmitter<DismissEvent> for SettingsWindow {}

impl Focusable for SettingsWindow {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.root_focus_handle.clone()
    }
}

impl SettingsWindow {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let theme_observer = cx.observe_global::<Theme>(|_this, cx| {
            cx.notify();
        });
        let root_focus_handle = cx.focus_handle();

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

        let auto_download_limit_input = cx.new(|cx| InputState::new(_window, cx).placeholder("20"));
        auto_download_limit_input.update(cx, |state, cx| {
            state.set_value("20", _window, cx);
        });
        let auto_download_input_subscription = cx.subscribe_in(
            &auto_download_limit_input,
            _window,
            |this, _, event: &InputEvent, _window, cx| {
                this.handle_auto_download_input_focus(event, cx);
            },
        );

        gpui_component::Theme::global_mut(cx).ring = Theme::weixin_colors(cx).input_field_focus;

        let shortcut_screenshot_input =
            cx.new(|cx| InputState::new(_window, cx).placeholder(SHORTCUT_INPUT_PLACEHOLDER));
        shortcut_screenshot_input.update(cx, |state, cx| {
            state.set_value(SHORTCUT_SCREENSHOT_DEFAULT, _window, cx);
        });

        let shortcut_lock_input =
            cx.new(|cx| InputState::new(_window, cx).placeholder(SHORTCUT_INPUT_PLACEHOLDER));
        shortcut_lock_input.update(cx, |state, cx| {
            state.set_value(SHORTCUT_LOCK_DEFAULT, _window, cx);
        });

        let shortcut_toggle_input =
            cx.new(|cx| InputState::new(_window, cx).placeholder(SHORTCUT_INPUT_PLACEHOLDER));
        shortcut_toggle_input.update(cx, |state, cx| {
            state.set_value(SHORTCUT_TOGGLE_DEFAULT, _window, cx);
        });

        let mut shortcut_display_texts = HashMap::new();
        shortcut_display_texts.insert(
            ShortcutInputField::Screenshot,
            SHORTCUT_SCREENSHOT_DEFAULT.to_string(),
        );
        shortcut_display_texts.insert(ShortcutInputField::Lock, SHORTCUT_LOCK_DEFAULT.to_string());
        shortcut_display_texts.insert(
            ShortcutInputField::ToggleWindow,
            SHORTCUT_TOGGLE_DEFAULT.to_string(),
        );

        let shortcut_screenshot_subscription = cx.subscribe_in(
            &shortcut_screenshot_input,
            _window,
            |this, _, event: &InputEvent, window, cx| {
                this.handle_shortcut_input_focus(ShortcutInputField::Screenshot, event, window, cx);
            },
        );
        let shortcut_lock_subscription = cx.subscribe_in(
            &shortcut_lock_input,
            _window,
            |this, _, event: &InputEvent, window, cx| {
                this.handle_shortcut_input_focus(ShortcutInputField::Lock, event, window, cx);
            },
        );
        let shortcut_toggle_subscription = cx.subscribe_in(
            &shortcut_toggle_input,
            _window,
            |this, _, event: &InputEvent, window, cx| {
                this.handle_shortcut_input_focus(
                    ShortcutInputField::ToggleWindow,
                    event,
                    window,
                    cx,
                );
            },
        );
        let shortcut_input_subscriptions = vec![
            shortcut_screenshot_subscription,
            shortcut_lock_subscription,
            shortcut_toggle_subscription,
        ];

        let mut this = Self {
            active_tab_ix: 1,
            current_language: "简体中文".to_string(),
            root_focus_handle,
            font_slider,
            _font_slider_subscription: font_slider_subscription,
            current_font_size_value: initial_font_size,
            _theme_observer: Some(theme_observer),
            theme_hover: ThemeHover::None,
            language_hover: LanguageHover::None,
            translate_language_selection: "简体中文".to_string(),
            translate_language_hover: TranslateLanguageHover::None,
            auto_download_limit_input,
            auto_download_input_focused: false,
            _auto_download_input_subscription: auto_download_input_subscription,
            shortcut_send_selection: "Enter".to_string(),
            shortcut_send_hover: ShortcutSendHover::None,
            shortcut_screenshot_input,
            shortcut_lock_input,
            shortcut_toggle_input,
            shortcut_display_texts,
            shortcut_input_widths: HashMap::new(),
            focused_shortcut_input: None,
            _shortcut_input_subscriptions: shortcut_input_subscriptions,
        };

        this.refresh_all_shortcut_input_widths(_window, cx);
        cx.defer_in(_window, |this, window, cx| {
            this.refresh_all_shortcut_input_widths(window, cx);
        });
        this
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub(crate) fn auto_download_limit_input(&self) -> &Entity<InputState> {
        &self.auto_download_limit_input
    }

    pub(crate) fn blur_focus(&self, window: &mut Window) {
        self.root_focus_handle.focus(window);
    }

    pub(crate) fn is_auto_download_input_focused(&self) -> bool {
        self.auto_download_input_focused
    }

    pub(crate) fn shortcut_send_selection_label(&self) -> String {
        self.shortcut_send_selection.clone()
    }

    pub(crate) fn shortcut_send_hover_state(&self) -> ShortcutSendHover {
        self.shortcut_send_hover
    }

    pub(crate) fn translate_language_selection_label(&self) -> String {
        self.translate_language_selection.clone()
    }

    pub(crate) fn translate_language_hover_state(&self) -> TranslateLanguageHover {
        self.translate_language_hover
    }

    pub(crate) fn set_shortcut_send_hover(&mut self, hover: ShortcutSendHover) {
        self.shortcut_send_hover = hover;
    }

    pub(crate) fn clear_shortcut_send_hover(&mut self, hover: ShortcutSendHover) {
        if self.shortcut_send_hover == hover {
            self.shortcut_send_hover = ShortcutSendHover::None;
        }
    }

    pub(crate) fn set_shortcut_send_selection(&mut self, value: &str) {
        self.shortcut_send_selection = value.to_string();
    }

    pub(crate) fn set_translate_language_selection(&mut self, value: &str) {
        self.translate_language_selection = value.to_string();
    }

    pub(crate) fn set_translate_language_hover(&mut self, hover: TranslateLanguageHover) {
        self.translate_language_hover = hover;
    }

    pub(crate) fn clear_translate_language_hover(&mut self, hover: TranslateLanguageHover) {
        if self.translate_language_hover == hover {
            self.translate_language_hover = TranslateLanguageHover::None;
        }
    }

    pub(crate) fn shortcut_input_state(&self, field: ShortcutInputField) -> &Entity<InputState> {
        match field {
            ShortcutInputField::Screenshot => &self.shortcut_screenshot_input,
            ShortcutInputField::Lock => &self.shortcut_lock_input,
            ShortcutInputField::ToggleWindow => &self.shortcut_toggle_input,
        }
    }

    pub(crate) fn set_shortcut_field_text(
        &mut self,
        field: ShortcutInputField,
        value: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let owned = Self::normalize_shortcut_text(value);
        self.shortcut_display_texts.insert(field, owned.clone());
        let state = self.shortcut_input_state(field).clone();
        let display_text = owned.clone();
        state.update(cx, |input, cx| {
            input.set_value(display_text.clone(), window, cx);
        });
        if self.refresh_shortcut_input_width(field, window, cx) {
            cx.notify();
        }
    }

    pub(crate) fn reset_shortcut_fields(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_shortcut_field_text(
            ShortcutInputField::Screenshot,
            SHORTCUT_SCREENSHOT_DEFAULT,
            window,
            cx,
        );
        self.set_shortcut_field_text(ShortcutInputField::Lock, SHORTCUT_LOCK_DEFAULT, window, cx);
        self.set_shortcut_field_text(
            ShortcutInputField::ToggleWindow,
            SHORTCUT_TOGGLE_DEFAULT,
            window,
            cx,
        );
    }

    pub(crate) fn shortcut_input_width(&self, field: ShortcutInputField) -> Pixels {
        self.shortcut_input_widths
            .get(&field)
            .copied()
            .unwrap_or_else(|| crate::ui::constants::settings_shortcut_input_min_width())
    }

    pub(crate) fn shortcut_display_text(
        &self,
        field: ShortcutInputField,
        cx: &mut Context<Self>,
    ) -> String {
        if self.is_shortcut_input_focused(field) {
            let live_text = self.read_shortcut_input_text(field, cx);
            return if live_text.is_empty() {
                SHORTCUT_INPUT_PLACEHOLDER.to_string()
            } else {
                live_text
            };
        }

        let text = self
            .shortcut_display_texts
            .get(&field)
            .cloned()
            .unwrap_or_else(|| self.read_shortcut_input_text(field, cx));
        if text.is_empty() {
            SHORTCUT_INPUT_PLACEHOLDER.to_string()
        } else {
            text
        }
    }

    fn read_shortcut_input_text(
        &self,
        field: ShortcutInputField,
        cx: &mut Context<Self>,
    ) -> String {
        let state = self.shortcut_input_state(field).clone();
        state.read_with(cx, |input, _| input.text().to_string())
    }

    fn normalize_shortcut_text(text: &str) -> String {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            String::new()
        } else {
            trimmed.split_whitespace().collect::<Vec<_>>().join(" ")
        }
    }

    fn commit_shortcut_input_text(
        &mut self,
        field: ShortcutInputField,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let raw = self.read_shortcut_input_text(field, cx);
        let normalized = Self::normalize_shortcut_text(&raw);
        let current_display = self
            .shortcut_display_texts
            .get(&field)
            .cloned()
            .unwrap_or_default();
        let final_text = if normalized.is_empty() {
            current_display
        } else {
            normalized
        };
        self.shortcut_display_texts
            .insert(field, final_text.clone());
        let state = self.shortcut_input_state(field).clone();
        state.update(cx, |input, cx| {
            input.set_value(final_text.clone(), window, cx);
        });
        self.refresh_shortcut_input_width(field, window, cx)
    }

    fn refresh_all_shortcut_input_widths(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let fields = [
            ShortcutInputField::Screenshot,
            ShortcutInputField::Lock,
            ShortcutInputField::ToggleWindow,
        ];
        for field in fields {
            self.refresh_shortcut_input_width(field, window, cx);
        }
    }

    fn refresh_shortcut_input_width(
        &mut self,
        field: ShortcutInputField,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let width = self.measure_shortcut_input_width(field, window, cx);
        let changed = self
            .shortcut_input_widths
            .get(&field)
            .map(|current| *current != width)
            .unwrap_or(true);
        self.shortcut_input_widths.insert(field, width);
        changed
    }

    fn measure_shortcut_input_width(
        &self,
        field: ShortcutInputField,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Pixels {
        let theme = cx.theme();
        let foreground = theme.foreground;
        let icon_color = gpui::rgb(0x7b7b7b);
        let min_width = crate::ui::constants::settings_shortcut_input_min_width();
        let max_width = crate::ui::constants::settings_shortcut_input_max_width();
        let display_text = self.shortcut_display_text(field, cx);

        let mut measure_variant = |text: &str, include_icon: bool| {
            let mut row = h_flex().items_center().gap(px(0.5)).px(px(4.));
            row = row.child(
                div()
                    .pl(px(3.))
                    .pr(px(1.5))
                    .whitespace_nowrap()
                    .text_xs()
                    .text_color(foreground)
                    .child(text.to_string()),
            );
            if include_icon {
                row = row.child(
                    div()
                        .rounded(crate::ui::constants::radius_sm())
                        .px(px(6.))
                        .py(px(2.))
                        .child(
                            Icon::default()
                                .path("setting/close_fill.svg")
                                .text_color(icon_color),
                        ),
                );
            }

            let mut element = div()
                .rounded(crate::ui::constants::radius_sm())
                .border_1()
                .border_color(gpui::transparent_black())
                .max_w(max_width)
                .child(row)
                .into_any_element();
            let available_space = size(AvailableSpace::MaxContent, AvailableSpace::MinContent);
            element.layout_as_root(available_space, window, cx).width
        };

        let show_icon = display_text != SHORTCUT_INPUT_PLACEHOLDER && display_text != "点击设置";

        let padding_adjustment = px(10.);
        let mut measured_width =
            measure_variant(display_text.as_str(), show_icon) + padding_adjustment;
        let enforce_min_width =
            display_text != "点击设置" && display_text != SHORTCUT_INPUT_PLACEHOLDER;
        if enforce_min_width {
            measured_width = measured_width.max(min_width);
        }
        measured_width = measured_width.min(max_width);

        measured_width
    }

    pub(crate) fn is_shortcut_input_focused(&self, field: ShortcutInputField) -> bool {
        self.focused_shortcut_input == Some(field)
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

    fn handle_auto_download_input_focus(&mut self, event: &InputEvent, cx: &mut Context<Self>) {
        match event {
            InputEvent::Focus => {
                if !self.auto_download_input_focused {
                    self.auto_download_input_focused = true;
                    cx.notify();
                }
            }
            InputEvent::Blur => {
                if self.auto_download_input_focused {
                    self.auto_download_input_focused = false;
                    cx.notify();
                }
            }
            _ => {}
        }
    }

    fn handle_shortcut_input_focus(
        &mut self,
        field: ShortcutInputField,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let state = self.shortcut_input_state(field).clone();
        match event {
            InputEvent::Focus => {
                let mut should_notify = false;
                if self.focused_shortcut_input != Some(field) {
                    self.focused_shortcut_input = Some(field);
                    should_notify = true;
                }
                state.update(cx, |input, cx| {
                    input.set_value("", window, cx);
                });
                if self.refresh_shortcut_input_width(field, window, cx) {
                    should_notify = true;
                }
                if should_notify {
                    cx.notify();
                }
            }
            InputEvent::Blur => {
                let mut should_notify = false;
                if self.focused_shortcut_input == Some(field) {
                    self.focused_shortcut_input = None;
                    should_notify = true;
                }
                if self.commit_shortcut_input_text(field, window, cx) {
                    should_notify = true;
                }
                if should_notify {
                    cx.notify();
                }
            }
            InputEvent::PressEnter { .. } => {
                if self.commit_shortcut_input_text(field, window, cx) {
                    cx.notify();
                }
            }
            _ => {}
        }
    }

    fn render_content(&self, window: &mut Window, cx: &mut Context<Self>) -> gpui::AnyElement {
        match self.get_active_tab() {
            SettingsTab::AccountAndStorage => {
                self.render_account_and_storage_tab(cx).into_any_element()
            }
            SettingsTab::General => self.render_general_tab(window, cx).into_any_element(),
            SettingsTab::Shortcuts => self.render_shortcuts_tab(window, cx).into_any_element(),
            SettingsTab::Notifications => self.render_notifications_tab(cx).into_any_element(),
            SettingsTab::Plugins => self.render_plugins_tab(cx).into_any_element(),
            SettingsTab::About => self.render_about_tab(cx).into_any_element(),
        }
    }

    pub(crate) fn render_theme_setting(
        &self,
        current_mode: ThemeMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let foreground = theme.foreground;

        setting_card::setting_row()
            .py_2()
            .child(div().text_sm().text_color(foreground).child("外观"))
            .child(self.render_theme_button(current_mode, window, cx))
    }

    /// 通用设置中的下拉按钮触发器（语言 / 主题 / 字体），统一使用同一风格。
    pub(crate) fn general_select_trigger_button(
        id: &'static str,
        label: String,
        cx: &mut Context<Self>,
    ) -> Button {
        let weixin_colors = Theme::weixin_colors(cx);
        let foreground = cx.theme().foreground;

        // 亮色：白底 + 边框，hover f2f2f2 / border ebebeb
        // 深色：沿用微信深色布局的搜索/hover/选中颜色
        let (bg, hover, active, border) = Theme::general_select_button_colors(cx);

        let variant = ButtonCustomVariant::new(cx)
            .color(bg)
            .foreground(foreground)
            .border(border)
            .hover(hover)
            .active(active);

        Button::new(id)
            .xsmall()
            .h(px(26.))
            .w(px(90.))
            .custom(variant)
            .child(
                h_flex()
                    .items_center()
                    .gap_2()
                    .text_color(foreground)
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

    pub(crate) fn render_static_select_item<FSet, L>(
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
        crate::ui::base::menu_item::MenuItem::new(id, label)
            .hovered(hovered)
            .compact(true)
            .on_hover(set_hover)
            .on_click(on_mouse_down)
    }

    pub(crate) fn render_setting_row(
        &self,
        label: &'static str,
        enabled: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();

        setting_card::setting_row()
            .py_2()
            .child(div().text_sm().text_color(theme.foreground).child(label))
            .child(
                Switch::new(label)
                    .checked(enabled)
                    .with_size(Size::Small),
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

    pub(crate) fn render_language_button(
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

    pub(crate) fn render_translate_language_button(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let label = self.translate_language_selection_label();
        let settings = cx.entity();

        Popover::new("translate-language-popover")
            .appearance(false)
            .anchor(gpui::Corner::BottomRight)
            .trigger(Self::general_select_trigger_button(
                "translate-language-btn",
                label,
                cx,
            ))
            .content(move |_, _window, cx| {
                let theme = cx.theme();

                let hover_state = settings.read(cx).translate_language_hover_state();
                let simplified_hovered =
                    matches!(hover_state, TranslateLanguageHover::SimplifiedChinese);
                let traditional_hovered =
                    matches!(hover_state, TranslateLanguageHover::TraditionalChinese);

                let settings_for_simplified_hover = settings.clone();
                let set_simplified_hover = move |is_hovering: bool, cx: &mut App| {
                    _ = settings_for_simplified_hover.update(
                        cx,
                        |this: &mut SettingsWindow, cx| {
                            if is_hovering {
                                this.set_translate_language_hover(
                                    TranslateLanguageHover::SimplifiedChinese,
                                );
                            } else {
                                this.clear_translate_language_hover(
                                    TranslateLanguageHover::SimplifiedChinese,
                                );
                            }
                            cx.notify();
                        },
                    );
                };

                let settings_for_traditional_hover = settings.clone();
                let set_traditional_hover = move |is_hovering: bool, cx: &mut App| {
                    _ = settings_for_traditional_hover.update(
                        cx,
                        |this: &mut SettingsWindow, cx| {
                            if is_hovering {
                                this.set_translate_language_hover(
                                    TranslateLanguageHover::TraditionalChinese,
                                );
                            } else {
                                this.clear_translate_language_hover(
                                    TranslateLanguageHover::TraditionalChinese,
                                );
                            }
                            cx.notify();
                        },
                    );
                };

                let settings_for_simplified_click = settings.clone();
                let simplified_click = cx.listener(move |_, _, _window, cx| {
                    _ = settings_for_simplified_click.update(
                        cx,
                        |this: &mut SettingsWindow, cx| {
                            this.set_translate_language_selection("简体中文");
                            this.clear_translate_language_hover(
                                TranslateLanguageHover::SimplifiedChinese,
                            );
                            cx.notify();
                        },
                    );
                    cx.emit(gpui::DismissEvent);
                });

                let settings_for_traditional_click = settings.clone();
                let traditional_click = cx.listener(move |_, _, _window, cx| {
                    _ = settings_for_traditional_click.update(
                        cx,
                        |this: &mut SettingsWindow, cx| {
                            this.set_translate_language_selection("繁体中文");
                            this.clear_translate_language_hover(
                                TranslateLanguageHover::TraditionalChinese,
                            );
                            cx.notify();
                        },
                    );
                    cx.emit(gpui::DismissEvent);
                });

                v_flex()
                    .w(crate::ui::constants::popover_width_sm())
                    .gap_0()
                    .py_2()
                    .bg(theme.popover)
                    .p_1()
                    .rounded(crate::ui::constants::radius_md())
                    .shadow_md()
                    .child(Self::render_static_language_item(
                        "translate-language-simplified",
                        "简体中文",
                        simplified_hovered,
                        set_simplified_hover,
                        simplified_click,
                    ))
                    .child(Self::render_static_language_item(
                        "translate-language-traditional",
                        "繁体中文",
                        traditional_hovered,
                        set_traditional_hover,
                        traditional_click,
                    ))
            })
    }

    pub(crate) fn render_font_size_slider(
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
            .py_2()
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
                div()
                    .w(px(0.))
                    .h(px(0.))
                    .track_focus(&self.root_focus_handle),
            )
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
                        div()
                            .flex_1()
                            .w_full()
                            .h(crate::ui::constants::settings_window_content_height())
                            .relative()
                            .child(
                                v_flex()
                                    .size_full()
                                    .scrollable(Axis::Vertical)
                                    .p_6()
                                    .child(self.render_content(window, cx)),
                            ),
                    ),
            )
    }
}
