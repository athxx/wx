use crate::app::state::WeixinApp;
use gpui::App;

impl WeixinApp {
    /// 对外暴露的打开设置窗口接口，内部委托给 app::bootstrap，统一窗口创建路径。
    pub fn open_settings_window(cx: &mut App) {
        crate::app::bootstrap::open_settings_window(cx);
    }
}
