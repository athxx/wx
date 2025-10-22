use gpui::{rgb, rgba, App, Context, Entity, Global, Hsla, Render};
use serde::{Deserialize, Serialize};

/// 主题模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn toggle(&self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }
}

/// 主题颜色定义
#[derive(Debug, Clone)]
pub struct ThemeColors {
    // 工具栏
    pub toolbar_background: Hsla,
    pub toolbar_icon_normal: Hsla,
    pub toolbar_icon_hover: Hsla,
    pub toolbar_icon_active: Hsla,
    pub toolbar_active_bg: Hsla,

    // 会话列表
    pub session_list_background: Hsla,
    pub session_list_search_bg: Hsla,
    pub session_list_item_hover: Hsla,
    pub session_list_item_active: Hsla,
    pub session_list_text: Hsla,
    pub session_list_text_muted: Hsla,
    pub session_list_border: Hsla,

    // 聊天区域
    pub chat_background: Hsla,
    pub chat_input_bg: Hsla,
    pub chat_toolbar_bg: Hsla,
    pub chat_text: Hsla,
    pub chat_text_muted: Hsla,

    // 消息气泡
    pub message_bubble_self: Hsla,
    pub message_bubble_other: Hsla,
    pub message_text_self: Hsla,
    pub message_text_other: Hsla,

    // 按钮
    pub button_primary: Hsla,
    pub button_primary_hover: Hsla,
    pub button_primary_text: Hsla,

    // 标题栏
    pub titlebar_background: Hsla,
    pub titlebar_text: Hsla,
    pub titlebar_button_hover: Hsla,
    pub titlebar_close_hover: Hsla,

    // 通用
    pub border: Hsla,
    pub divider: Hsla,
    pub shadow: Hsla,

    // 未读徽章
    pub unread_badge: Hsla,
    pub unread_badge_text: Hsla,

    // 滚动条
    pub scrollbar_track: Hsla,
    pub scrollbar_thumb: Hsla,
    pub scrollbar_thumb_hover: Hsla,

    // 设置窗口
    pub settings_sidebar_bg: Hsla,
    pub settings_sidebar_active_bg: Hsla,
    pub settings_sidebar_hover_bg: Hsla,
    pub settings_content_bg: Hsla,
    pub settings_text_primary: Hsla,
    pub settings_text_secondary: Hsla,
    pub settings_input_bg: Hsla,
}

impl ThemeColors {
    /// 亮色主题
    pub fn light() -> Self {
        Self {
            // 工具栏 - 左侧底色 EDEDED
            toolbar_background: rgb(0xEDEDED).into(),
            toolbar_icon_normal: rgb(0x666666).into(),
            toolbar_icon_hover: rgb(0x333333).into(),
            toolbar_icon_active: rgb(0x07c160).into(),
            toolbar_active_bg: rgb(0xDCDCDC).into(),

            // 会话列表 - 中间底色 F7F7F7
            session_list_background: rgb(0xF7F7F7).into(),
            session_list_search_bg: rgb(0xEAEAEA).into(),
            session_list_item_hover: rgb(0xEDEDED).into(),
            session_list_item_active: rgb(0xD6D6D6).into(),
            session_list_text: rgb(0x333333).into(),
            session_list_text_muted: rgb(0x999999).into(),
            session_list_border: rgb(0xe7e7e7).into(),

            // 聊天区域 - 右侧底色 EDEDED
            chat_background: rgb(0xEDEDED).into(),
            chat_input_bg: rgb(0xffffff).into(),
            chat_toolbar_bg: rgb(0xffffff).into(),
            chat_text: rgb(0x333333).into(),
            chat_text_muted: rgb(0x999999).into(),

            // 消息气泡
            message_bubble_self: rgb(0x95ec69).into(),
            message_bubble_other: rgb(0xffffff).into(),
            message_text_self: rgb(0x000000).into(),
            message_text_other: rgb(0x333333).into(),

            // 按钮
            button_primary: rgb(0x07c160).into(),
            button_primary_hover: rgb(0x06ad56).into(),
            button_primary_text: rgb(0xffffff).into(),

            // 标题栏 (应该跟随具体区域，但默认使用会话列表颜色)
            titlebar_background: rgb(0xEDEDED).into(),
            titlebar_text: rgb(0x333333).into(),
            titlebar_button_hover: rgb(0xe0e0e0).into(),
            titlebar_close_hover: rgb(0xe81123).into(),

            // 通用
            border: rgb(0xe7e7e7).into(),
            divider: rgb(0xdddddd).into(),
            shadow: rgba(0x00000020).into(),

            // 未读徽章
            unread_badge: rgb(0xfa5151).into(),
            unread_badge_text: rgb(0xffffff).into(),

            // 滚动条
            scrollbar_track: rgba(0x00000010).into(),
            scrollbar_thumb: rgba(0x00000040).into(),
            scrollbar_thumb_hover: rgba(0x00000060).into(),

            // 设置窗口 - 左侧 F7F7F7，右侧 EDEDED
            settings_sidebar_bg: rgb(0xF7F7F7).into(),
            settings_sidebar_active_bg: rgb(0xe7e7e7).into(),
            settings_sidebar_hover_bg: rgb(0xEDEDED).into(),
            settings_content_bg: rgb(0xEDEDED).into(),
            settings_text_primary: rgb(0x333333).into(),
            settings_text_secondary: rgb(0x999999).into(),
            settings_input_bg: rgb(0xf0f0f0).into(),
        }
    }

    /// 暗色主题
    pub fn dark() -> Self {
        Self {
            // 工具栏 - 左侧深色
            toolbar_background: rgb(0x1e1e1e).into(),
            toolbar_icon_normal: rgba(0xffffffaa).into(),
            toolbar_icon_hover: rgb(0xffffff).into(),
            toolbar_icon_active: rgb(0x07c160).into(),
            toolbar_active_bg: rgb(0x2a2a2a).into(),

            // 会话列表 - 中间深灰
            session_list_background: rgb(0x252525).into(),
            session_list_search_bg: rgb(0x1e1e1e).into(),
            session_list_item_hover: rgb(0x2e2e2e).into(),
            session_list_item_active: rgb(0x333333).into(),
            session_list_text: rgb(0xe6e6e6).into(),
            session_list_text_muted: rgb(0x888888).into(),
            session_list_border: rgb(0x3a3a3a).into(),

            // 聊天区域 - 右侧深色 (全部都是同一个颜色)
            chat_background: rgb(0x1e1e1e).into(),
            chat_input_bg: rgb(0x2b2b2b).into(),
            chat_toolbar_bg: rgb(0x252525).into(),
            chat_text: rgb(0xe6e6e6).into(),
            chat_text_muted: rgb(0x888888).into(),

            // 消息气泡 - 别人白色，我的绿色
            message_bubble_self: rgb(0x95ec69).into(),
            message_bubble_other: rgb(0xffffff).into(),
            message_text_self: rgb(0x000000).into(),
            message_text_other: rgb(0x333333).into(),

            // 按钮
            button_primary: rgb(0x07c160).into(),
            button_primary_hover: rgb(0x06ad56).into(),
            button_primary_text: rgb(0xffffff).into(),

            // 标题栏
            titlebar_background: rgb(0x252525).into(),
            titlebar_text: rgb(0xe6e6e6).into(),
            titlebar_button_hover: rgb(0x3a3a3a).into(),
            titlebar_close_hover: rgb(0xe81123).into(),

            // 通用
            border: rgb(0x3a3a3a).into(),
            divider: rgb(0x404040).into(),
            shadow: rgba(0x00000040).into(),

            // 未读徽章
            unread_badge: rgb(0xfa5151).into(),
            unread_badge_text: rgb(0xffffff).into(),

            // 滚动条
            scrollbar_track: rgba(0xffffff10).into(),
            scrollbar_thumb: rgba(0xffffff30).into(),
            scrollbar_thumb_hover: rgba(0xffffff50).into(),

            // 设置窗口
            settings_sidebar_bg: rgb(0x252525).into(),
            settings_sidebar_active_bg: rgb(0x333333).into(),
            settings_sidebar_hover_bg: rgb(0x2e2e2e).into(),
            settings_content_bg: rgb(0x1e1e1e).into(),
            settings_text_primary: rgb(0xe6e6e6).into(),
            settings_text_secondary: rgb(0x888888).into(),
            settings_input_bg: rgb(0x2b2b2b).into(),
        }
    }
}

/// 主题状态
#[derive(Clone)]
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: ThemeColors,
}

impl Global for Theme {}

impl Theme {
    pub fn new(mode: ThemeMode) -> Self {
        let colors = match mode {
            ThemeMode::Light => ThemeColors::light(),
            ThemeMode::Dark => ThemeColors::dark(),
        };
        Self { mode, colors }
    }

    pub fn light() -> Self {
        Self::new(ThemeMode::Light)
    }

    pub fn dark() -> Self {
        Self::new(ThemeMode::Dark)
    }

    /// 获取当前全局主题
    pub fn get(cx: &App) -> Self {
        cx.try_global::<Theme>()
            .cloned()
            .unwrap_or_else(|| Self::light())
    }

    /// 设置全局主题
    pub fn set(theme: Theme, cx: &mut App) {
        cx.set_global(theme);
    }

    /// 切换主题
    pub fn toggle(cx: &mut App) {
        let current = Self::get(cx);
        let new_mode = current.mode.toggle();
        let new_theme = Self::new(new_mode);
        Self::set(new_theme, cx);
    }

    /// 切换到亮色主题
    pub fn set_light(cx: &mut App) {
        Self::set(Self::light(), cx);
    }

    /// 切换到暗色主题
    pub fn set_dark(cx: &mut App) {
        Self::set(Self::dark(), cx);
    }

    /// 判断是否为暗色主题
    pub fn is_dark(&self) -> bool {
        matches!(self.mode, ThemeMode::Dark)
    }
}
