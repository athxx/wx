use gpui::{rgb, rgba, App, Hsla};
use gpui_component::{ActiveTheme as _, Theme as GpuiTheme, ThemeMode as GpuiThemeMode};

// Re-export ThemeMode from gpui-component
pub use gpui_component::ThemeMode;

/// WeChat特定的主题颜色扩展
#[derive(Debug, Clone)]
pub struct WeixinThemeColors {
    // 布局背景色
    pub toolbar_bg: Hsla,           // 左侧工具栏背景 EDEDED
    pub session_list_bg: Hsla,      // 中间会话列表背景 F7F7F7
    pub chat_area_bg: Hsla,         // 右侧聊天区域背景 EDEDED
    pub search_bar_bg: Hsla,        // 搜索框和加号背景 EDEDED
    pub item_hover: Hsla,           // hover颜色 EAEAEA
    pub item_selected: Hsla,        // 选中颜色 DEDEDE
    
    // 消息气泡
    pub message_bubble_self: Hsla,
    pub message_bubble_other: Hsla,
    pub message_text_self: Hsla,
    pub message_text_other: Hsla,
    
    // WeChat特色的绿色
    pub weixin_green: Hsla,
    pub weixin_green_hover: Hsla,
    
    // 未读徽章
    pub unread_badge: Hsla,
    pub unread_badge_text: Hsla,
}

impl WeixinThemeColors {
    pub fn light() -> Self {
        Self {
            // 布局背景色
            toolbar_bg: rgb(0xEDEDED).into(),          // 左侧工具栏背景
            session_list_bg: rgb(0xF7F7F7).into(),     // 中间会话列表背景
            chat_area_bg: rgb(0xEDEDED).into(),        // 右侧聊天区域背景
            search_bar_bg: rgb(0xEDEDED).into(),       // 搜索框和加号背景
            item_hover: rgb(0xEAEAEA).into(),          // hover颜色
            item_selected: rgb(0xDEDEDE).into(),       // 选中颜色
            
            // 消息气泡
            message_bubble_self: rgb(0x95ec69).into(),
            message_bubble_other: rgb(0xffffff).into(),
            message_text_self: rgb(0x000000).into(),
            message_text_other: rgb(0x333333).into(),
            
            // WeChat特色的绿色
            weixin_green: rgb(0x07c160).into(),
            weixin_green_hover: rgb(0x06ad56).into(),
            
            // 未读徽章
            unread_badge: rgb(0xfa5151).into(),
            unread_badge_text: rgb(0xffffff).into(),
        }
    }

    pub fn dark() -> Self {
        Self {
            // 布局背景色 - 深色模式
            toolbar_bg: rgb(0x2A2A2A).into(),          // 左侧工具栏背景
            session_list_bg: rgb(0x1F1F1F).into(),     // 中间会话列表背景
            chat_area_bg: rgb(0x2A2A2A).into(),        // 右侧聊天区域背景
            search_bar_bg: rgb(0x2A2A2A).into(),       // 搜索框和加号背景
            item_hover: rgb(0x333333).into(),          // hover颜色
            item_selected: rgb(0x3A3A3A).into(),       // 选中颜色
            
            // 消息气泡
            message_bubble_self: rgb(0x95ec69).into(),
            message_bubble_other: rgb(0x3a3a3a).into(),
            message_text_self: rgb(0x000000).into(),
            message_text_other: rgb(0xe6e6e6).into(),
            
            // WeChat特色的绿色
            weixin_green: rgb(0x07c160).into(),
            weixin_green_hover: rgb(0x06ad56).into(),
            
            // 未读徽章
            unread_badge: rgb(0xfa5151).into(),
            unread_badge_text: rgb(0xffffff).into(),
        }
    }
}

/// 主题助手 - 提供对gpui-component主题系统的便捷访问
pub struct Theme;

impl Theme {
    /// 获取gpui-component的全局主题
    pub fn get(cx: &App) -> &GpuiTheme {
        cx.theme()
    }

    /// 获取WeChat特定的主题颜色
    pub fn weixin_colors(cx: &App) -> WeixinThemeColors {
        match cx.theme().mode {
            GpuiThemeMode::Light => WeixinThemeColors::light(),
            GpuiThemeMode::Dark => WeixinThemeColors::dark(),
        }
    }

    /// 切换主题模式
    pub fn toggle(cx: &mut App) {
        let current_mode = cx.theme().mode;
        let new_mode = match current_mode {
            GpuiThemeMode::Light => GpuiThemeMode::Dark,
            GpuiThemeMode::Dark => GpuiThemeMode::Light,
        };
        GpuiTheme::change(new_mode, None, cx);
    }

    /// 设置为亮色主题
    pub fn set_light(cx: &mut App) {
        GpuiTheme::change(GpuiThemeMode::Light, None, cx);
    }

    /// 设置为暗色主题
    pub fn set_dark(cx: &mut App) {
        GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
    }

    /// 判断是否为暗色主题
    pub fn is_dark(cx: &App) -> bool {
        matches!(cx.theme().mode, GpuiThemeMode::Dark)
    }
}
