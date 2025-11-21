use crate::ui::theme::ThemeMode;
use gpui::App;
use gpui_component::ActiveTheme;
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
const CONFIG_FILE: &str = "target/weixin_config.json";
#[cfg(not(debug_assertions))]
const CONFIG_FILE: &str = "weixin_config.json";

/// 持久化的状态：布局 + 主题模式 + 字体大小，全部写在同一个 JSON 里。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutState {
    /// 左侧会话区域宽度。
    pub session_left_width: f32,
    /// 聊天输入区域高度（Pixels -> f32）。
    #[serde(default)]
    pub chat_input_height: Option<f32>,
    /// 当前主题模式（浅色 / 深色）。
    #[serde(default)]
    pub theme_mode: Option<ThemeMode>,
    /// 基础字体大小，单位 px。
    #[serde(default)]
    pub font_size: Option<f32>,
}

impl LayoutState {
    /// 从配置文件加载布局状态，如果失败则返回给定的默认值
    pub fn load_or(default: LayoutState) -> Self {
        if let Ok(json) = std::fs::read_to_string(CONFIG_FILE) {
            serde_json::from_str::<LayoutState>(&json).unwrap_or(default)
        } else {
            default
        }
    }

    /// 保存布局状态到配置文件
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(CONFIG_FILE, json);
        }
    }
}

/// 主题与字体大小用户偏好视图结构（方便在设置窗口中使用）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    /// 当前主题模式（浅色 / 深色）。
    pub theme_mode: ThemeMode,
    /// 基础字体大小，单位 px。
    pub font_size: f32,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Light,
            font_size: 16.0,
        }
    }
}

impl Preferences {
    /// 从磁盘加载用户偏好，如果失败则返回默认值。
    pub fn load() -> Self {
        if let Ok(json) = std::fs::read_to_string(CONFIG_FILE) {
            if let Ok(state) = serde_json::from_str::<LayoutState>(&json) {
                return Preferences {
                    theme_mode: state.theme_mode.unwrap_or(ThemeMode::Light),
                    font_size: state.font_size.unwrap_or(16.0),
                };
            }
        }
        Preferences::default()
    }

    /// 将当前偏好写入磁盘（与布局一起保存在同一个 JSON）。
    pub fn save(&self) {
        // 先尝试读取已有布局状态，如果不存在则创建默认值。
        let mut state = LayoutState::load_or(LayoutState {
            session_left_width: 200.0,
            chat_input_height: None,
            theme_mode: Some(self.theme_mode),
            font_size: Some(self.font_size),
        });

        state.theme_mode = Some(self.theme_mode);
        state.font_size = Some(self.font_size);

        if let Ok(json) = serde_json::to_string_pretty(&state) {
            let _ = std::fs::write(CONFIG_FILE, json);
        }
    }

    /// 从当前 App 全局 Theme 生成偏好并写入磁盘。
    pub fn save_from_app(cx: &mut App) {
        let mut prefs = Preferences::load();
        let theme = cx.theme();
        prefs.theme_mode = theme.mode;
        let font_size: f32 = theme.font_size.into();
        prefs.font_size = font_size;
        prefs.save();
    }
}
